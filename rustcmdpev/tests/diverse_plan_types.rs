//! G9.2 — Integration coverage across a representative set of PostgreSQL
//! plan shapes. Each fixture exercises a different node mix (sort/limit,
//! aggregate+filter, CTE, merge join, append/union, nested loop w/ inner
//! index scan) and asserts the rendered output names the right node types,
//! propagates relation/index annotations, and exits successfully with a
//! non-empty payload.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("diverse")
        .join(format!("{name}.json"))
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("failed to run rustcmdpev")
}

fn run_stdin(args: &[&str], payload: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn");
    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin.write_all(payload.as_bytes()).expect("write");
    }
    child.wait_with_output().expect("wait")
}

fn render_pretty(fixture: &str) -> String {
    let path = fixture_path(fixture);
    let output = run(&[
        "--input",
        path.to_str().unwrap(),
        "--color",
        "never",
        "--format",
        "pretty",
    ]);
    assert!(
        output.status.success(),
        "{fixture} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("utf-8")
}

#[test]
fn sort_limit_fixture_renders_expected_node_chain() {
    let rendered = render_pretty("sort_limit");
    assert!(rendered.contains("Limit"));
    assert!(rendered.contains("Sort"));
    assert!(rendered.contains("Seq Scan"));
    assert!(rendered.contains("accounts"));
}

#[test]
fn aggregate_groupby_fixture_renders_filter_and_strategy() {
    let rendered = render_pretty("aggregate_groupby");
    assert!(rendered.contains("Aggregate"));
    assert!(rendered.contains("Hashed"));
    assert!(rendered.contains("filter"));
    assert!(rendered.contains("250 rows") || rendered.contains("[-250"));
}

#[test]
fn cte_recursive_fixture_renders_cte_label_and_index_scan() {
    let rendered = render_pretty("cte_recursive");
    assert!(rendered.contains("CTE Scan"));
    assert!(rendered.contains("ancestors"));
    assert!(rendered.contains("Index Scan"));
    assert!(rendered.contains("categories_pkey"));
}

#[test]
fn merge_join_fixture_renders_two_children_and_index_only_scan() {
    let rendered = render_pretty("merge_join");
    assert!(rendered.contains("Merge Join"));
    assert!(rendered.contains("Index Only Scan"));
    assert!(rendered.contains("orders"));
    assert!(rendered.contains("customers"));
    assert!(rendered.contains("Sort"));
}

#[test]
fn append_union_fixture_renders_both_children() {
    let rendered = render_pretty("append_union");
    assert!(rendered.contains("Append"));
    assert!(rendered.contains("orders_2024"));
    assert!(rendered.contains("orders_2025"));
}

#[test]
fn nested_loop_with_index_fixture_renders_join_and_inner_index() {
    let rendered = render_pretty("nested_loop_with_index");
    assert!(rendered.contains("Nested Loop"));
    assert!(rendered.contains("Index Scan"));
    assert!(rendered.contains("logins_user_id_idx"));
    assert!(rendered.contains("users"));
}

#[test]
fn json_format_round_trips_diverse_plan_shapes() {
    for fixture in [
        "sort_limit",
        "aggregate_groupby",
        "cte_recursive",
        "merge_join",
        "append_union",
        "nested_loop_with_index",
    ] {
        let path = fixture_path(fixture);
        let raw = std::fs::read_to_string(&path).expect("read fixture");
        let output = run_stdin(&["--color", "never", "--format", "json"], &raw);
        assert!(
            output.status.success(),
            "json round-trip failed for {fixture}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("\"plan\""));
    }
}

#[test]
fn table_format_emits_header_and_node_rows() {
    let path = fixture_path("merge_join");
    let output = run(&[
        "--input",
        path.to_str().unwrap(),
        "--color",
        "never",
        "--format",
        "table",
    ]);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("NODE | DURATION_MS"));
    assert!(stdout.contains("Merge Join"));
    assert!(stdout.contains("Index Only Scan"));
}
