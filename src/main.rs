mod adapters;
mod analyzer;
mod license;
mod model;
mod render;

use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use analyzer::{AnalyzeError, AnalyzeOptions};
use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand, ValueEnum};

const FREE_DEPTH: usize = 2;
const FREE_NODES: usize = 40;
const PRO_DEPTH: usize = 8;
const PRO_NODES: usize = 250;

#[derive(Parser)]
#[command(
    name = "code-path-lens",
    version,
    about = "Build a bounded, evidence-backed code path around one symbol",
    long_about = "Code Path Lens parses local source with tree-sitter and emits a deterministic review slice: callers, callees, types, data boundaries, excerpts, and explicit unresolved calls. It never uploads source and never claims runtime completeness."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Trace a symbol and emit a self-contained viewer, JSON, or DOT
    Trace(TraceArgs),
    /// List language adapters and their deliberate fallbacks
    Languages,
}

#[derive(Args)]
struct TraceArgs {
    /// Exact function/method name, or the stable id shown by an ambiguity error
    symbol: String,
    /// Repository root to scan
    #[arg(long, default_value = ".")]
    root: PathBuf,
    /// Call steps to follow in each direction (free: 0-2; Pro: 0-8)
    #[arg(long, default_value_t = 2, value_parser = parse_depth)]
    depth: usize,
    /// Hard node budget (free: 1-40; Pro: 1-250)
    #[arg(long, default_value_t = 40, value_parser = parse_nodes)]
    max_nodes: usize,
    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Html)]
    format: OutputFormat,
    /// Shortcut for --format json and stdout
    #[arg(long, conflicts_with = "format")]
    json: bool,
    /// Output path; use - for stdout
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Extra repository-relative exclusion glob (repeatable)
    #[arg(long)]
    exclude: Vec<String>,
    /// Include files carrying common generated-code markers
    #[arg(long)]
    include_generated: bool,
    /// Source URL template; supports {path}, {abs}, and {line}
    #[arg(long, default_value = "vscode://file/{abs}:{line}")]
    link_template: String,
    /// Pro token; prefer CODE_PATH_LENS_LICENSE to avoid shell history
    #[arg(long)]
    license: Option<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum OutputFormat {
    Html,
    Json,
    Dot,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error:#}");
            if error.downcast_ref::<AnalyzeError>().is_some() {
                ExitCode::from(3)
            } else if error.to_string().contains("license") || error.to_string().contains("Pro") {
                ExitCode::from(4)
            } else {
                ExitCode::from(1)
            }
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Languages => {
            println!("Supported tree-sitter adapters:");
            println!("  Rust        .rs");
            println!("  TypeScript  .ts .tsx .mts .cts");
            println!("  JavaScript  .js .jsx .mjs .cjs (TypeScript grammar fallback)");
            println!("  Python      .py");
            println!("  Go          .go");
            println!();
            println!(
                "Unsupported languages are counted and skipped. Calls that cannot be resolved"
            );
            println!("inside parsed files remain visible as unresolved edges.");
        }
        Command::Trace(args) => trace(args)?,
    }
    Ok(())
}

fn trace(args: TraceArgs) -> Result<()> {
    if args.depth > FREE_DEPTH || args.max_nodes > FREE_NODES {
        license::require_pro(args.license.as_deref())?;
    }
    let format = if args.json {
        OutputFormat::Json
    } else {
        args.format
    };
    let graph = analyzer::analyze(&AnalyzeOptions {
        root: args.root,
        symbol: args.symbol.clone(),
        depth: args.depth,
        max_nodes: args.max_nodes,
        include_generated: args.include_generated,
        excludes: args.exclude,
        link_template: args.link_template,
    })?;
    let rendered = match format {
        OutputFormat::Html => render::render_html(&graph)?,
        OutputFormat::Json => render::render_json(&graph)?,
        OutputFormat::Dot => render::render_dot(&graph),
    };
    let output = args.output.unwrap_or_else(|| match format {
        OutputFormat::Html => PathBuf::from(format!("{}-lens.html", safe_filename(&args.symbol))),
        OutputFormat::Json => PathBuf::from("-"),
        OutputFormat::Dot => PathBuf::from(format!("{}-lens.dot", safe_filename(&args.symbol))),
    });
    if output.as_os_str() == "-" {
        let mut stdout = io::stdout().lock();
        stdout
            .write_all(rendered.as_bytes())
            .context("could not write stdout")?;
        stdout.write_all(b"\n").context("could not finish stdout")?;
    } else {
        fs::write(&output, rendered)
            .with_context(|| format!("could not write {}", output.display()))?;
        eprintln!(
            "Wrote {} nodes and {} edges to {}",
            graph.nodes.len(),
            graph.edges.len(),
            output.display()
        );
        for warning in &graph.warnings {
            eprintln!("warning: {warning}");
        }
    }
    Ok(())
}

fn safe_filename(value: &str) -> String {
    let value: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect();
    value
        .trim_matches('-')
        .to_string()
        .chars()
        .take(80)
        .collect::<String>()
        .to_lowercase()
}

fn parse_depth(value: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|_| "depth must be an integer".to_string())?;
    (parsed <= PRO_DEPTH)
        .then_some(parsed)
        .ok_or_else(|| format!("depth must be between 0 and {PRO_DEPTH}"))
}

fn parse_nodes(value: &str) -> Result<usize, String> {
    let parsed = value
        .parse::<usize>()
        .map_err(|_| "max-nodes must be an integer".to_string())?;
    (1..=PRO_NODES)
        .contains(&parsed)
        .then_some(parsed)
        .ok_or_else(|| format!("max-nodes must be between 1 and {PRO_NODES}"))
}

#[cfg(test)]
mod tests {
    use super::safe_filename;

    #[test]
    fn filenames_are_portable() {
        assert_eq!(safe_filename("crate::handle/order"), "crate--handle-order");
    }
}
