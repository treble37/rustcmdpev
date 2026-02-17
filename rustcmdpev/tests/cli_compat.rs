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
fn compat_pretty_with_default_width_succeeds() {
    let output = run_with_args_and_stdin(
        &["--compat", "--format", "pretty"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
    );
    assert!(output.status.success());
}

#[test]
fn compat_rejects_non_pretty_format() {
    let output = run_with_args_and_stdin(
        &["--compat", "--format", "json"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--compat currently supports only --format pretty"));
}

#[test]
fn compat_rejects_non_legacy_width() {
    let output = run_with_args_and_stdin(
        &["--compat", "--width", "80"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--compat requires --width 60"));
}
