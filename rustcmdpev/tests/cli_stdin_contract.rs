use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run_with_stdin(stdin_payload: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
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
fn empty_stdin_returns_non_zero_and_actionable_error() {
    let output = run_with_stdin("");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no input provided"));
}

#[test]
fn invalid_json_returns_non_zero_and_error() {
    let output = run_with_stdin("not-json");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid JSON input"));
}

#[test]
fn invalid_top_level_shape_returns_non_zero_and_error() {
    let output = run_with_stdin("{\"Plan\":{}}");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("top-level JSON must be an array"));
}

#[test]
fn empty_array_returns_non_zero_and_error() {
    let output = run_with_stdin("[]");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("top-level JSON array must contain at least one explain object"));
}

#[test]
fn missing_plan_returns_non_zero_and_error() {
    let output = run_with_stdin("[{\"Planning Time\": 1.0}]");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("must contain 'Plan' object"));
}

#[test]
fn valid_minimal_explain_array_succeeds() {
    let output = run_with_stdin("[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]");
    assert!(output.status.success());
}
