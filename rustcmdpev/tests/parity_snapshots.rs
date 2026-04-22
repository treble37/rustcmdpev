use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("parity")
        .join(format!("{name}.json"))
}

fn snapshot_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("parity")
        .join(format!("{name}.snap"))
}

fn strip_ansi(input: &str) -> String {
    let mut normalized = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            let _ = chars.next();
            for next in chars.by_ref() {
                if ('@'..='~').contains(&next) {
                    break;
                }
            }
            continue;
        }
        normalized.push(ch);
    }

    normalized
}

fn normalize_snapshot_text(input: &str) -> String {
    let stripped = strip_ansi(input).replace("\r\n", "\n");
    let lines = stripped
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");
    format!("{lines}\n")
}

fn run_compat_fixture(name: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--input")
        .arg(fixture_path(name))
        .arg("--compat")
        .arg("--format")
        .arg("pretty")
        .arg("--color")
        .arg("always")
        .output()
        .expect("failed to run rustcmdpev");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    normalize_snapshot_text(
        &String::from_utf8(output.stdout).expect("stdout should be valid utf-8"),
    )
}

#[test]
fn example_fixture_matches_snapshot() {
    for fixture_name in [
        "example",
        "real_world_hash_join",
        "real_world_nested_loop",
        "real_world_bitmap_heap_scan",
    ] {
        let expected = std::fs::read_to_string(snapshot_path(fixture_name))
            .expect("expected parity snapshot file");
        let expected = normalize_snapshot_text(&expected);
        let actual = run_compat_fixture(fixture_name);
        assert_eq!(actual, expected, "snapshot mismatch for {fixture_name}");
    }
}

#[test]
fn compat_snapshots_are_deterministic_across_repeated_runs() {
    for fixture_name in [
        "example",
        "real_world_hash_join",
        "real_world_nested_loop",
        "real_world_bitmap_heap_scan",
    ] {
        let first = run_compat_fixture(fixture_name);
        let second = run_compat_fixture(fixture_name);
        let expected = normalize_snapshot_text(
            &std::fs::read_to_string(snapshot_path(fixture_name))
                .expect("expected parity snapshot file"),
        );

        assert_eq!(
            first, second,
            "non-deterministic compat output for {fixture_name}"
        );
        assert_eq!(
            first, expected,
            "deterministic output drift for {fixture_name}"
        );
    }
}

#[test]
fn snapshot_normalization_strips_ansi_and_trailing_whitespace() {
    let raw = "\u{1b}[31mline one\u{1b}[0m  \r\nline two\t \r\n";
    assert_eq!(normalize_snapshot_text(raw), "line one\nline two\n");
}
