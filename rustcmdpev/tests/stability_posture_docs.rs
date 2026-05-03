use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn read(path: PathBuf) -> String {
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

#[test]
fn readme_documents_pre_1_0_stability_posture() {
    let readme = read(repo_root().join("README.md"));
    assert!(
        readme.contains("Stability:") && readme.contains("pre-stable"),
        "README.md must keep the pre-1.0 stability posture callout (look for 'Stability:' and 'pre-stable')"
    );
    assert!(
        readme.contains("docs/src/versioning.md"),
        "README.md must link to docs/src/versioning.md for the full posture"
    );
}

#[test]
fn versioning_docs_documents_pre_1_0_stability_posture() {
    let versioning = read(repo_root().join("docs/src/versioning.md"));
    assert!(
        versioning.contains("Pre-1.0 stability posture"),
        "docs/src/versioning.md must keep the 'Pre-1.0 stability posture' section"
    );
    assert!(
        versioning.contains("`0.x`") || versioning.contains("0.x.y"),
        "versioning docs must reference the 0.x pre-stable series"
    );
}
