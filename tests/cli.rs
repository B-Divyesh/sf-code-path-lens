use std::fs;
use std::process::Command;

use tempfile::tempdir;

#[test]
fn documented_html_and_json_commands_work_end_to_end() {
    let repo = tempdir().unwrap();
    fs::write(
        repo.path().join("orders.rs"),
        "struct Order {}\nfn handle_order(order: Order) { validate(order); }\nfn validate(order: Order) {}\n",
    )
    .unwrap();
    let html_path = repo.path().join("order-lens.html");
    let status = Command::new(env!("CARGO_BIN_EXE_code-path-lens"))
        .args(["trace", "handle_order", "--root"])
        .arg(repo.path())
        .args(["--output"])
        .arg(&html_path)
        .status()
        .unwrap();
    assert!(status.success());
    let html = fs::read_to_string(html_path).unwrap();
    assert!(html.contains("<html lang=\"en\""));
    assert!(html.contains("handle_order"));

    let output = Command::new(env!("CARGO_BIN_EXE_code-path-lens"))
        .args(["trace", "handle_order", "--root"])
        .arg(repo.path())
        .arg("--json")
        .output()
        .unwrap();
    assert!(output.status.success());
    let graph: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(graph["symbol"], "handle_order");
    assert!(graph["nodes"].as_array().unwrap().len() >= 2);
}

#[test]
fn bundled_demo_runs_without_a_repository_and_reports_its_output() {
    let directory = tempdir().unwrap();
    let output = directory.path().join("sample-lens.html");
    let result = Command::new(env!("CARGO_BIN_EXE_code-path-lens"))
        .args(["demo", "--output"])
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(String::from_utf8_lossy(&result.stderr).contains("Bundled sample repository:"));
    let html = fs::read_to_string(output).unwrap();
    assert!(html.contains("handle_order"));
    assert!(html.contains("database I/O"));
    assert!(html.contains("emit_receipt"));
}
