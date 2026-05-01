use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run(args: &[&str], stdin_payload: &str) -> Output {
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

const PAYLOAD: &str = r#"[{"Plan":{"Node Type":"Hash Join","Total Cost":4.0,"Actual Total Time":1.0,"Actual Rows":2,"Actual Loops":1,"Plans":[{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":0.5,"Actual Rows":2,"Actual Loops":1,"Shared Hit Blocks":7,"I/O Read Time":1.5}]},"Execution Time":2.0}]"#;

#[test]
fn default_summary_keeps_three_line_header() {
    let output = run(&["--color", "never"], PAYLOAD);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("○ Total Cost"));
    assert!(stdout.contains("○ Planning Time"));
    assert!(stdout.contains("○ Execution Time"));
    assert!(!stdout.contains("○ Total Loops"));
    assert!(!stdout.contains("○ Total Nodes"));
}

#[test]
fn detailed_summary_includes_loops_buffers_and_io_time() {
    let output = run(&["--color", "never", "--summary", "detailed"], PAYLOAD);
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("○ Total Loops:"));
    assert!(stdout.contains("○ Total Nodes: 2"));
    assert!(stdout.contains("○ Buffers: shared hit=7"));
    assert!(stdout.contains("○ I/O Time:"));
}

#[test]
fn compat_rejects_detailed_summary() {
    let output = run(&["--compat", "--summary", "detailed", "--color", "never"], PAYLOAD);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--compat"));
}
