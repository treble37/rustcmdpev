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

const SEQ_SCAN_PAYLOAD: &str = r#"[{"Plan":{"Node Type":"Seq Scan","Total Cost":1.0,"Actual Total Time":1.0,"Actual Rows":2,"Actual Loops":1,"Shared Hit Blocks":3}}]"#;

#[test]
fn no_color_theme_strips_ansi_even_with_color_always() {
    let output = run(
        &["--color", "always", "--theme", "no-color"],
        SEQ_SCAN_PAYLOAD,
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !stdout.contains("\u{1b}["),
        "expected no ANSI escapes, got:\n{stdout}"
    );
    assert!(stdout.contains("Seq Scan"));
}

#[test]
fn condensed_render_mode_skips_descriptions() {
    let descriptive = run(
        &["--color", "never", "--render-mode", "default"],
        SEQ_SCAN_PAYLOAD,
    );
    let condensed = run(
        &["--color", "never", "--render-mode", "condensed"],
        SEQ_SCAN_PAYLOAD,
    );
    assert!(descriptive.status.success() && condensed.status.success());
    let descriptive_stdout = String::from_utf8_lossy(&descriptive.stdout);
    let condensed_stdout = String::from_utf8_lossy(&condensed.stdout);
    assert!(descriptive_stdout.contains("Finds relevant records"));
    assert!(!condensed_stdout.contains("Finds relevant records"));
}

#[test]
fn verbose_render_mode_includes_loops_and_buffers() {
    let output = run(
        &["--color", "never", "--render-mode", "verbose"],
        SEQ_SCAN_PAYLOAD,
    );
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("○ Loops:"));
    assert!(stdout.contains("○ Buffers:"));
}

#[test]
fn compat_rejects_non_default_render_mode() {
    let output = run(
        &["--color", "never", "--compat", "--render-mode", "verbose"],
        SEQ_SCAN_PAYLOAD,
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--compat"));
}
