use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::WalkBuilder;

use crate::adapters::{Adapter, parse_file};
use crate::model::{
    EdgeKind, LensEdge, LensGraph, LensNode, Limits, NodeKind, ParsedKind, ParsedSymbol,
    ScanSummary, SourceLocation,
};

pub struct AnalyzeOptions {
    pub root: PathBuf,
    pub symbol: String,
    pub depth: usize,
    pub max_nodes: usize,
    pub include_generated: bool,
    pub excludes: Vec<String>,
    pub link_template: String,
}

#[derive(Debug)]
pub enum AnalyzeError {
    NotFound(String),
    Ambiguous(String),
}

impl std::fmt::Display for AnalyzeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(symbol) => write!(
                f,
                "symbol `{symbol}` was not found in supported source files"
            ),
            Self::Ambiguous(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for AnalyzeError {}

pub fn analyze(options: &AnalyzeOptions) -> Result<LensGraph> {
    let root = options
        .root
        .canonicalize()
        .with_context(|| format!("cannot open repository root {}", options.root.display()))?;
    if !root.is_dir() {
        bail!("repository root {} is not a directory", root.display());
    }

    let excludes = compile_excludes(&options.excludes)?;
    let (symbols, summary, mut warnings) = scan(&root, options.include_generated, &excludes)?;
    let mut by_name: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, symbol) in symbols.iter().enumerate() {
        by_name.entry(symbol.name.clone()).or_default().push(index);
    }

    let matches: Vec<usize> = symbols
        .iter()
        .enumerate()
        .filter(|(_, symbol)| symbol.kind == ParsedKind::Function)
        .filter(|(_, candidate)| {
            candidate.name == options.symbol
                || candidate.id == options.symbol
                || candidate.id.ends_with(&format!(":{}", options.symbol))
        })
        .map(|(index, _)| index)
        .collect();

    let entry_index = match matches.as_slice() {
        [] => return Err(AnalyzeError::NotFound(options.symbol.clone()).into()),
        [only] => *only,
        many => {
            let candidates = many
                .iter()
                .map(|index| {
                    let item = &symbols[*index];
                    format!("  {} ({}:{})", item.id, item.rel_path, item.line)
                })
                .collect::<Vec<_>>()
                .join("\n");
            return Err(AnalyzeError::Ambiguous(format!(
                "symbol `{}` is ambiguous; use one of these stable ids:\n{}",
                options.symbol, candidates
            ))
            .into());
        }
    };

    let mut nodes: BTreeMap<String, LensNode> = BTreeMap::new();
    let mut edges: BTreeMap<(String, String, String), LensEdge> = BTreeMap::new();
    let mut queue = VecDeque::from([(entry_index, 0usize)]);
    let mut visited = BTreeSet::new();
    let mut truncated = false;

    while let Some((current_index, distance)) = queue.pop_front() {
        let current = &symbols[current_index];
        if !visited.insert(current.id.clone()) {
            continue;
        }
        if !insert_symbol_node(&mut nodes, current, current_index == entry_index, options) {
            truncated = true;
            break;
        }

        add_types(
            current,
            &symbols,
            &by_name,
            &mut nodes,
            &mut edges,
            options,
            &mut truncated,
        );
        add_call_evidence(
            current,
            distance,
            &symbols,
            &by_name,
            &mut nodes,
            &mut edges,
            &mut queue,
            options,
            &mut truncated,
        );

        if distance < options.depth {
            for (caller_index, caller) in symbols.iter().enumerate() {
                if caller.kind != ParsedKind::Function || caller_index == current_index {
                    continue;
                }
                if caller.calls.iter().any(|call| call.name == current.name) {
                    if insert_symbol_node(&mut nodes, caller, false, options) {
                        insert_edge(
                            &mut edges,
                            &current.id,
                            &caller.id,
                            EdgeKind::CalledBy,
                            format!("{} calls {}", caller.name, current.name),
                        );
                        queue.push_back((caller_index, distance + 1));
                    } else {
                        truncated = true;
                    }
                }
            }
        }
    }

    if truncated {
        warnings.push(format!(
            "Graph reached the {} node budget; some evidence is omitted. Re-run with --max-nodes to change the explicit bound.",
            options.max_nodes
        ));
    }
    if summary.skipped_generated > 0 {
        warnings.push(format!(
            "Skipped {} generated source file(s); pass --include-generated to inspect them.",
            summary.skipped_generated
        ));
    }
    warnings.push(
        "Static approximation: dynamic dispatch, reflection, macros, generated code, and runtime configuration may change the executed path."
            .to_string(),
    );

    let mut node_list: Vec<_> = nodes.into_values().collect();
    node_list.sort_by_key(|node| {
        let rank = match node.kind {
            NodeKind::Entry => 0,
            NodeKind::Function => 1,
            NodeKind::Type => 2,
            NodeKind::DataBoundary => 3,
            NodeKind::Unresolved => 4,
        };
        (rank, node.id.clone())
    });

    Ok(LensGraph {
        schema_version: 1,
        symbol: options.symbol.clone(),
        root: root.display().to_string(),
        approximation_notice: "Bounded static evidence, not a runtime-complete call trace."
            .to_string(),
        limits: Limits {
            depth: options.depth,
            max_nodes: options.max_nodes,
            truncated,
        },
        scanned: summary,
        nodes: node_list,
        edges: edges.into_values().collect(),
        warnings,
    })
}

fn scan(
    root: &Path,
    include_generated: bool,
    excludes: &GlobSet,
) -> Result<(Vec<ParsedSymbol>, ScanSummary, Vec<String>)> {
    let mut builder = WalkBuilder::new(root);
    builder
        .hidden(false)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .parents(true)
        .follow_links(false);
    let mut summary = ScanSummary::default();
    let mut symbols = Vec::new();
    let mut warnings = Vec::new();

    for result in builder.build() {
        let entry = match result {
            Ok(entry) => entry,
            Err(error) => {
                warnings.push(format!("walk warning: {error}"));
                continue;
            }
        };
        if !entry.file_type().is_some_and(|kind| kind.is_file()) {
            continue;
        }
        summary.files += 1;
        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(path);
        if excluded_by_default(rel) || excludes.is_match(rel) {
            continue;
        }
        let Some(adapter) = Adapter::for_path(path) else {
            summary.unsupported_files += 1;
            continue;
        };
        let source = match fs::read_to_string(path) {
            Ok(source) => source,
            Err(error) => {
                warnings.push(format!("could not read {}: {error}", rel.display()));
                continue;
            }
        };
        if !include_generated && is_generated(rel, &source) {
            summary.skipped_generated += 1;
            continue;
        }
        summary.parsed_files += 1;
        match parse_file(
            adapter,
            &source,
            &rel.to_string_lossy().replace('\\', "/"),
            &path.display().to_string(),
        ) {
            Ok(mut found) => symbols.append(&mut found),
            Err(error) => warnings.push(format!("could not parse {}: {error:#}", rel.display())),
        }
    }
    symbols.sort_by(|a, b| a.id.cmp(&b.id));
    Ok((symbols, summary, warnings))
}

fn compile_excludes(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(
            Glob::new(pattern).with_context(|| format!("invalid --exclude glob `{pattern}`"))?,
        );
    }
    builder.build().context("could not compile exclusion globs")
}

fn excluded_by_default(path: &Path) -> bool {
    path.components().any(|part| {
        matches!(
            part.as_os_str().to_str(),
            Some("node_modules" | "target" | "dist" | "build" | "vendor" | ".next" | "coverage")
        )
    })
}

fn is_generated(path: &Path, source: &str) -> bool {
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    name.ends_with(".generated.rs")
        || name.ends_with(".generated.ts")
        || name.ends_with(".generated.go")
        || name.ends_with("_pb2.py")
        || name.ends_with(".min.js")
        || source.lines().take(8).any(|line| {
            let lower = line.to_ascii_lowercase();
            lower.contains("code generated")
                || lower.contains("@generated")
                || lower.contains("do not edit")
        })
}

fn insert_symbol_node(
    nodes: &mut BTreeMap<String, LensNode>,
    symbol: &ParsedSymbol,
    entry: bool,
    options: &AnalyzeOptions,
) -> bool {
    if nodes.contains_key(&symbol.id) {
        return true;
    }
    if nodes.len() >= options.max_nodes {
        return false;
    }
    let kind = if entry {
        NodeKind::Entry
    } else if symbol.kind == ParsedKind::Type {
        NodeKind::Type
    } else {
        NodeKind::Function
    };
    nodes.insert(symbol.id.clone(), node_for_symbol(symbol, kind, options));
    true
}

fn node_for_symbol(symbol: &ParsedSymbol, kind: NodeKind, options: &AnalyzeOptions) -> LensNode {
    LensNode {
        id: symbol.id.clone(),
        label: symbol.name.clone(),
        kind,
        language: Some(symbol.language.clone()),
        location: Some(SourceLocation {
            path: symbol.rel_path.clone(),
            line: symbol.line,
            end_line: symbol.end_line,
            link: options
                .link_template
                .replace("{path}", &symbol.rel_path)
                .replace("{abs}", &symbol.abs_path)
                .replace("{line}", &symbol.line.to_string()),
        }),
        excerpt: Some(symbol.excerpt.clone()),
        evidence: match kind {
            NodeKind::Entry => "requested entry symbol".to_string(),
            NodeKind::Type => "declared type referenced by a visible signature".to_string(),
            _ => "parsed function declaration".to_string(),
        },
    }
}

#[allow(clippy::too_many_arguments)]
fn add_types(
    current: &ParsedSymbol,
    symbols: &[ParsedSymbol],
    by_name: &BTreeMap<String, Vec<usize>>,
    nodes: &mut BTreeMap<String, LensNode>,
    edges: &mut BTreeMap<(String, String, String), LensEdge>,
    options: &AnalyzeOptions,
    truncated: &mut bool,
) {
    for name in &current.type_names {
        let Some(candidates) = by_name.get(name) else {
            continue;
        };
        if let Some(index) = candidates
            .iter()
            .find(|index| symbols[**index].kind == ParsedKind::Type)
        {
            let target = &symbols[*index];
            if insert_symbol_node(nodes, target, false, options) {
                insert_edge(
                    edges,
                    &current.id,
                    &target.id,
                    EdgeKind::UsesType,
                    format!("{} appears in {}'s declaration", name, current.name),
                );
            } else {
                *truncated = true;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn add_call_evidence(
    current: &ParsedSymbol,
    distance: usize,
    symbols: &[ParsedSymbol],
    by_name: &BTreeMap<String, Vec<usize>>,
    nodes: &mut BTreeMap<String, LensNode>,
    edges: &mut BTreeMap<(String, String, String), LensEdge>,
    queue: &mut VecDeque<(usize, usize)>,
    options: &AnalyzeOptions,
    truncated: &mut bool,
) {
    for call in &current.calls {
        if let Some(boundary) = &call.boundary {
            let id = format!("boundary:{}:{}:{}", current.id, call.line, boundary);
            if insert_evidence_node(
                nodes,
                &id,
                boundary,
                NodeKind::DataBoundary,
                format!(
                    "call `{}` at {}:{} matched the {} boundary rule",
                    call.name, current.rel_path, call.line, boundary
                ),
                options.max_nodes,
            ) {
                insert_edge(
                    edges,
                    &current.id,
                    &id,
                    EdgeKind::CrossesBoundary,
                    format!("{}:{} calls {}", current.rel_path, call.line, call.name),
                );
            } else {
                *truncated = true;
            }
        }

        let matches: Vec<_> = by_name
            .get(&call.name)
            .into_iter()
            .flatten()
            .filter(|index| symbols[**index].kind == ParsedKind::Function)
            .copied()
            .collect();
        if distance < options.depth && matches.len() == 1 {
            let target_index = matches[0];
            let target = &symbols[target_index];
            if insert_symbol_node(nodes, target, false, options) {
                insert_edge(
                    edges,
                    &current.id,
                    &target.id,
                    EdgeKind::Calls,
                    format!("call expression at {}:{}", current.rel_path, call.line),
                );
                queue.push_back((target_index, distance + 1));
                continue;
            }
            *truncated = true;
        }

        if matches.len() != 1 {
            let reason = if matches.is_empty() {
                "no matching declaration in scanned files"
            } else {
                "multiple matching declarations"
            };
            let id = format!("unresolved:{}:{}:{}", current.id, call.line, call.name);
            if insert_evidence_node(
                nodes,
                &id,
                &call.name,
                NodeKind::Unresolved,
                format!("{} at {}:{}", reason, current.rel_path, call.line),
                options.max_nodes,
            ) {
                insert_edge(
                    edges,
                    &current.id,
                    &id,
                    EdgeKind::UnresolvedCall,
                    format!(
                        "call expression at {}:{}; {}",
                        current.rel_path, call.line, reason
                    ),
                );
            } else {
                *truncated = true;
            }
        }
    }
}

fn insert_evidence_node(
    nodes: &mut BTreeMap<String, LensNode>,
    id: &str,
    label: &str,
    kind: NodeKind,
    evidence: String,
    max_nodes: usize,
) -> bool {
    if nodes.contains_key(id) {
        return true;
    }
    if nodes.len() >= max_nodes {
        return false;
    }
    nodes.insert(
        id.to_string(),
        LensNode {
            id: id.to_string(),
            label: label.to_string(),
            kind,
            language: None,
            location: None,
            excerpt: None,
            evidence,
        },
    );
    true
}

fn insert_edge(
    edges: &mut BTreeMap<(String, String, String), LensEdge>,
    from: &str,
    to: &str,
    kind: EdgeKind,
    evidence: String,
) {
    let kind_key = format!("{kind:?}");
    edges
        .entry((from.to_string(), to.to_string(), kind_key))
        .or_insert_with(|| LensEdge {
            from: from.to_string(),
            to: to.to_string(),
            kind,
            evidence,
        });
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn builds_a_bounded_graph_and_keeps_unresolved_evidence() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("src")).unwrap();
        fs::write(
            dir.path().join("src/lib.rs"),
            "struct Order {}\nfn entry(x: Order) { known(); mystery(); }\nfn known() {}\nfn caller() { entry(Order {}); }\n",
        )
        .unwrap();
        let graph = analyze(&AnalyzeOptions {
            root: dir.path().to_path_buf(),
            symbol: "entry".into(),
            depth: 2,
            max_nodes: 40,
            include_generated: false,
            excludes: vec![],
            link_template: "vscode://file/{abs}:{line}".into(),
        })
        .unwrap();
        assert!(graph.nodes.iter().any(|node| node.label == "known"));
        assert!(graph.nodes.iter().any(|node| node.label == "caller"));
        assert!(
            graph
                .nodes
                .iter()
                .any(|node| node.label == "mystery" && node.kind == NodeKind::Unresolved)
        );
        assert!(
            graph
                .nodes
                .iter()
                .any(|node| node.label == "Order" && node.kind == NodeKind::Type)
        );
    }

    #[test]
    fn reports_ambiguous_symbols() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("a.py"), "def run():\n    pass\n").unwrap();
        fs::write(dir.path().join("b.py"), "def run():\n    pass\n").unwrap();
        let error = analyze(&AnalyzeOptions {
            root: dir.path().to_path_buf(),
            symbol: "run".into(),
            depth: 1,
            max_nodes: 10,
            include_generated: false,
            excludes: vec![],
            link_template: "{path}:{line}".into(),
        })
        .unwrap_err();
        assert!(error.to_string().contains("ambiguous"));
    }
}
