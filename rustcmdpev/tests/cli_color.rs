use std::io::Write;
use std::process::{Command, Output, Stdio};

fn run_with_args_and_stdin(args: &[&str], stdin_payload: &str, no_color: bool) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"));
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if no_color {
        cmd.env("NO_COLOR", "1");
    }

    let mut child = cmd.spawn().expect("failed to spawn rustcmdpev");

    {
        let stdin = child.stdin.as_mut().expect("failed to open stdin");
        stdin
            .write_all(stdin_payload.as_bytes())
            .expect("failed to write stdin");
    }

    child.wait_with_output().expect("failed to wait on child")
}

#[test]
fn color_always_emits_ansi_sequences() {
    let output = run_with_args_and_stdin(
        &["--color", "always", "--format", "pretty"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
        false,
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\u{1b}["));
}

#[test]
fn color_never_disables_ansi_sequences() {
    let output = run_with_args_and_stdin(
        &["--color", "never", "--format", "pretty"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
        false,
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("\u{1b}["));
}

#[test]
fn color_auto_respects_no_color() {
    let output = run_with_args_and_stdin(
        &["--color", "auto", "--format", "pretty"],
        "[{\"Plan\":{\"Node Type\":\"Seq Scan\"}}]",
        true,
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("\u{1b}["));
}
