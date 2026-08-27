use std::collections::BTreeSet;
use std::path::Path;

use anyhow::{Context, Result};
use tree_sitter::{Language, Node, Parser};

use crate::model::{CallSite, ParsedKind, ParsedSymbol};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adapter {
    Rust,
    TypeScript,
    Tsx,
    JavaScript,
    Python,
    Go,
}

impl Adapter {
    pub fn for_path(path: &Path) -> Option<Self> {
        match path.extension()?.to_str()?.to_ascii_lowercase().as_str() {
            "rs" => Some(Self::Rust),
            "ts" | "mts" | "cts" => Some(Self::TypeScript),
            "tsx" => Some(Self::Tsx),
            "js" | "mjs" | "cjs" => Some(Self::JavaScript),
            "jsx" => Some(Self::Tsx),
            "py" => Some(Self::Python),
            "go" => Some(Self::Go),
            _ => None,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::TypeScript => "TypeScript",
            Self::Tsx => "TSX",
            Self::JavaScript => "JavaScript",
            Self::Python => "Python",
            Self::Go => "Go",
        }
    }

    fn language(self) -> Language {
        match self {
            Self::Rust => tree_sitter_rust::LANGUAGE.into(),
            Self::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Self::Tsx => tree_sitter_typescript::LANGUAGE_TSX.into(),
            Self::JavaScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            Self::Python => tree_sitter_python::LANGUAGE.into(),
            Self::Go => tree_sitter_go::LANGUAGE.into(),
        }
    }

    fn function_kinds(self) -> &'static [&'static str] {
        match self {
            Self::Rust => &["function_item"],
            Self::TypeScript | Self::Tsx | Self::JavaScript => {
                &["function_declaration", "method_definition"]
            }
            Self::Python => &["function_definition"],
            Self::Go => &["function_declaration", "method_declaration"],
        }
    }

    fn type_kinds(self) -> &'static [&'static str] {
        match self {
            Self::Rust => &["struct_item", "enum_item", "trait_item", "type_item"],
            Self::TypeScript | Self::Tsx | Self::JavaScript => &[
                "class_declaration",
                "interface_declaration",
                "type_alias_declaration",
                "enum_declaration",
            ],
            Self::Python => &["class_definition"],
            Self::Go => &["type_declaration", "type_spec"],
        }
    }
}

pub fn parse_file(
    adapter: Adapter,
    source: &str,
    rel_path: &str,
    abs_path: &str,
) -> Result<Vec<ParsedSymbol>> {
    let mut parser = Parser::new();
    parser
        .set_language(&adapter.language())
        .with_context(|| format!("could not load the {} parser", adapter.label()))?;
    let tree = parser
        .parse(source, None)
        .with_context(|| format!("{} parser returned no tree", adapter.label()))?;
    let mut symbols = Vec::new();
    visit_declarations(
        tree.root_node(),
        adapter,
        source,
        rel_path,
        abs_path,
        &mut symbols,
    );
    symbols.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(symbols)
}

fn visit_declarations(
    node: Node<'_>,
    adapter: Adapter,
    source: &str,
    rel_path: &str,
    abs_path: &str,
    out: &mut Vec<ParsedSymbol>,
) {
    let kind = node.kind();
    let parsed_kind =
        if adapter.function_kinds().contains(&kind) || is_function_variable(node, adapter) {
            Some(ParsedKind::Function)
        } else if adapter.type_kinds().contains(&kind) {
            Some(ParsedKind::Type)
        } else {
            None
        };

    if let Some(parsed_kind) = parsed_kind {
        if let Some(name) = declaration_name(node, source) {
            let start = node.start_position().row + 1;
            let end = node.end_position().row + 1;
            let excerpt = source_excerpt(source, start, end);
            let mut calls = Vec::new();
            let mut type_names = BTreeSet::new();
            if parsed_kind == ParsedKind::Function {
                collect_evidence(node, source, start, &mut calls, &mut type_names);
            }
            out.push(ParsedSymbol {
                id: format!("{}:{}:{}", rel_path, start, name),
                name,
                kind: parsed_kind,
                language: adapter.label().to_string(),
                rel_path: rel_path.to_string(),
                abs_path: abs_path.to_string(),
                line: start,
                end_line: end,
                excerpt,
                calls,
                type_names: type_names.into_iter().collect(),
            });
        }
        // Avoid treating nested declarations as evidence owned by an outer function.
        if adapter.function_kinds().contains(&kind) {
            return;
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        visit_declarations(child, adapter, source, rel_path, abs_path, out);
    }
}

fn is_function_variable(node: Node<'_>, adapter: Adapter) -> bool {
    if !matches!(
        adapter,
        Adapter::TypeScript | Adapter::Tsx | Adapter::JavaScript
    ) || node.kind() != "variable_declarator"
    {
        return false;
    }
    node.child_by_field_name("value")
        .is_some_and(|value| matches!(value.kind(), "arrow_function" | "function_expression"))
}

fn declaration_name(node: Node<'_>, source: &str) -> Option<String> {
    node.child_by_field_name("name")
        .or_else(|| find_first_kind(node, &["type_identifier", "identifier", "field_identifier"]))
        .and_then(|name| name.utf8_text(source.as_bytes()).ok())
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
}

fn find_first_kind<'a>(node: Node<'a>, kinds: &[&str]) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if kinds.contains(&child.kind()) {
            return Some(child);
        }
        if let Some(found) = find_first_kind(child, kinds) {
            return Some(found);
        }
    }
    None
}

fn collect_evidence(
    node: Node<'_>,
    source: &str,
    declaration_line: usize,
    calls: &mut Vec<CallSite>,
    types: &mut BTreeSet<String>,
) {
    if node.kind() == "call_expression" {
        if let Some(callee) = node
            .child_by_field_name("function")
            .or_else(|| node.named_child(0))
        {
            if let Some(name) = final_identifier(callee, source) {
                calls.push(CallSite {
                    boundary: boundary_for(&name),
                    name,
                    line: node.start_position().row + 1,
                });
            }
        }
    }

    if matches!(
        node.kind(),
        "type_identifier" | "generic_type" | "scoped_type_identifier"
    ) && node.start_position().row < declaration_line + 8
    {
        if let Some(name) = final_identifier(node, source) {
            if !is_primitive(&name) {
                types.insert(name);
            }
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_evidence(child, source, declaration_line, calls, types);
    }
}

fn final_identifier(node: Node<'_>, source: &str) -> Option<String> {
    if matches!(
        node.kind(),
        "identifier" | "field_identifier" | "type_identifier" | "property_identifier"
    ) {
        return node
            .utf8_text(source.as_bytes())
            .ok()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToOwned::to_owned);
    }
    let mut cursor = node.walk();
    node.children(&mut cursor)
        .filter_map(|child| final_identifier(child, source))
        .last()
}

fn source_excerpt(source: &str, start: usize, end: usize) -> String {
    let lines: Vec<_> = source.lines().collect();
    let excerpt_end = end.min(start + 17).min(lines.len());
    let begin = start.saturating_sub(1);
    lines[begin..excerpt_end]
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{:>4} │ {}", start + i, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn boundary_for(name: &str) -> Option<String> {
    let lower = name.to_ascii_lowercase();
    let label = if ["fetch", "request", "send", "recv", "connect"]
        .iter()
        .any(|part| lower.contains(part))
    {
        "network I/O"
    } else if ["read", "write", "open", "file", "mkdir"]
        .iter()
        .any(|part| lower.contains(part))
    {
        "filesystem I/O"
    } else if ["query", "execute", "insert", "update", "transaction"]
        .iter()
        .any(|part| lower.contains(part))
    {
        "database I/O"
    } else if ["serialize", "deserialize", "parse", "decode", "encode"]
        .iter()
        .any(|part| lower.contains(part))
    {
        "serialization"
    } else if ["getenv", "var", "environment"]
        .iter()
        .any(|part| lower.contains(part))
    {
        "process environment"
    } else {
        return None;
    };
    Some(label.to_string())
}

fn is_primitive(name: &str) -> bool {
    matches!(
        name,
        "String"
            | "str"
            | "bool"
            | "usize"
            | "isize"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "u8"
            | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "f32"
            | "f64"
            | "None"
            | "Any"
            | "unknown"
            | "number"
            | "string"
            | "void"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_adapter_finds_function_calls_and_types() {
        let source = r#"
struct Order { id: u64 }
fn handle(order: Order) -> Result<()> {
    validate(order);
    db.execute("insert");
    Ok(())
}
fn validate(order: Order) {}
"#;
        let symbols = parse_file(Adapter::Rust, source, "src/lib.rs", "/repo/src/lib.rs").unwrap();
        let handle = symbols.iter().find(|s| s.name == "handle").unwrap();
        assert!(handle.calls.iter().any(|c| c.name == "validate"));
        assert!(
            handle
                .calls
                .iter()
                .any(|c| c.boundary.as_deref() == Some("database I/O"))
        );
        assert!(handle.type_names.contains(&"Order".to_string()));
    }

    #[test]
    fn polyglot_adapters_find_documented_declarations() {
        let cases = [
            (
                Adapter::TypeScript,
                "type Order = { id: number };\nconst handle = (order: Order) => validate(order);\nfunction validate(order: Order) {}\n",
                "handle",
            ),
            (
                Adapter::Python,
                "class Order:\n    pass\n\ndef handle(order: Order):\n    validate(order)\n",
                "handle",
            ),
            (
                Adapter::Go,
                "package shop\ntype Order struct { ID int }\nfunc handle(order Order) { validate(order) }\n",
                "handle",
            ),
        ];
        for (adapter, source, expected) in cases {
            let symbols = parse_file(adapter, source, "fixture", "/fixture").unwrap();
            assert!(
                symbols.iter().any(|symbol| symbol.name == expected),
                "{} adapter missed {expected}",
                adapter.label()
            );
        }
    }
}
