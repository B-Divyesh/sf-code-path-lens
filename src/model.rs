use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Entry,
    Function,
    Type,
    DataBoundary,
    Unresolved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    Calls,
    CalledBy,
    UsesType,
    CrossesBoundary,
    UnresolvedCall,
}

#[derive(Clone, Debug, Serialize)]
pub struct SourceLocation {
    pub path: String,
    pub line: usize,
    pub end_line: usize,
    pub link: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LensNode {
    pub id: String,
    pub label: String,
    pub kind: NodeKind,
    pub language: Option<String>,
    pub location: Option<SourceLocation>,
    pub excerpt: Option<String>,
    pub evidence: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LensEdge {
    pub from: String,
    pub to: String,
    pub kind: EdgeKind,
    pub evidence: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LensGraph {
    pub schema_version: u8,
    pub symbol: String,
    pub root: String,
    pub approximation_notice: String,
    pub limits: Limits,
    pub scanned: ScanSummary,
    pub nodes: Vec<LensNode>,
    pub edges: Vec<LensEdge>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Limits {
    pub depth: usize,
    pub max_nodes: usize,
    pub truncated: bool,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ScanSummary {
    pub files: usize,
    pub parsed_files: usize,
    pub skipped_generated: usize,
    pub unsupported_files: usize,
}

#[derive(Clone, Debug)]
pub struct ParsedSymbol {
    pub id: String,
    pub name: String,
    pub kind: ParsedKind,
    pub language: String,
    pub rel_path: String,
    pub abs_path: String,
    pub line: usize,
    pub end_line: usize,
    pub excerpt: String,
    pub calls: Vec<CallSite>,
    pub type_names: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParsedKind {
    Function,
    Type,
}

#[derive(Clone, Debug)]
pub struct CallSite {
    pub name: String,
    pub line: usize,
    pub boundary: Option<String>,
}
