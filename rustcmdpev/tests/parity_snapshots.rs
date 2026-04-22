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

fn run_pretty_fixture(name: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rustcmdpev"))
        .arg("--input")
        .arg(fixture_path(name))
        .output()
        .expect("failed to run rustcmdpev");
    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).expect("stdout should be valid utf-8")
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
        let actual = run_pretty_fixture(fixture_name);
        assert_eq!(actual, expected, "snapshot mismatch for {fixture_name}");
    }
}
