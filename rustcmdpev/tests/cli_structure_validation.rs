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
fn rejects_non_array_plans_field() {
    let output = run_with_stdin("[{\"Plan\":{\"Plans\":{}}}]");
    assert_eq!(output.status.code(), Some(3));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Plans must be an array"));
}

#[test]
fn rejects_negative_numeric_invariant() {
    let output = run_with_stdin("[{\"Plan\":{\"Total Cost\":-1.0}}]");
    assert_eq!(output.status.code(), Some(3));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Total Cost must be non-negative"));
}

#[test]
fn rejects_non_object_child_plan() {
    let output = run_with_stdin("[{\"Plan\":{\"Plans\":[1]}}]");
    assert_eq!(output.status.code(), Some(3));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Plans[0] must be an object"));
}

#[test]
fn accepts_valid_nested_plan_shape() {
    let output = run_with_stdin(
        "[{\"Plan\":{\"Node Type\":\"Hash Join\",\"Total Cost\":1.0,\"Plans\":[{\"Node Type\":\"Seq Scan\",\"Total Cost\":0.5}]}}]",
    );
    assert_eq!(output.status.code(), Some(0));
}

fn nested_plan_json(depth: usize) -> String {
    let mut json = String::from("[{\"Plan\":");
    for _ in 0..depth {
        json.push_str("{\"Node Type\":\"Nested Loop\",\"Plans\":[");
    }
    json.push_str("{\"Node Type\":\"Seq Scan\"}");
    for _ in 0..depth {
        json.push_str("]}");
    }
    json.push_str("}]");
    json
}

#[test]
fn rejects_excessive_plan_depth() {
    let output = run_with_stdin(&nested_plan_json(33));
    assert_eq!(output.status.code(), Some(3));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("maximum supported plan depth"));
}

#[test]
fn rejects_excessive_plan_node_count() {
    let mut json = String::from("[{\"Plan\":{\"Node Type\":\"Append\",\"Plans\":[");
    for idx in 0..10_001 {
        if idx > 0 {
            json.push(',');
        }
        json.push_str("{\"Node Type\":\"Seq Scan\"}");
    }
    json.push_str("]}}]");

    let output = run_with_stdin(&json);
    assert_eq!(output.status.code(), Some(3));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("maximum supported node count"));
}
