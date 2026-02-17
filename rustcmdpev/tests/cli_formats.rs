use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run_with_args_and_stdin(args: &[&str], stdin_payload: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn rustcmdpev");

    {
        let stdin = child.stdin.as_mut().expect("failed to open stdin");
        stdin
            .write_all(stdin_payload.as_bytes())
            .expect("failed to write stdin");
    }

    child.wait_with_output().expect("failed to wait on child")
}

#[test]
fn format_json_succeeds_and_contains_plan() {
    let output = run_with_args_and_stdin(
        &["--format", "json"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"plan\""));
    assert!(stdout.contains("\"node_type\""));
}

#[test]
fn format_table_succeeds_and_contains_header_and_node() {
    let output = run_with_args_and_stdin(
        &["--format", "table"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("NODE | DURATION_MS | COST | ROWS | TAGS"));
    assert!(stdout.contains("Seq Scan"));
}
