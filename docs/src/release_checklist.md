# Release Checklist

Use this checklist when preparing a release tag.

## MVP v1 release checklist

- [ ] Confirm the `P0` release gate passes.
- [ ] Complete the [parity report artifact](parity_report_template.md) for the tagged release.
- [ ] Link the completed parity report from the release notes.
- [ ] Attach or reference platform release artifacts and checksums.
- [ ] Verify `cargo install rustcmdpev --locked` works for the release version.
- [ ] Verify one tagged release archive installs or extracts correctly on each supported platform.
- [ ] Verify `cat example.json | rustcmdpev` succeeds against the release binary.

## crates.io publish dry-runs

Run these from a clean working tree before tagging the release. Both must
succeed (full verify, not `--no-verify`) before publishing for real.

- [ ] `cargo publish -p rustcmdpev-core --dry-run` succeeds (packages, verifies
  by compiling the packaged tarball, and aborts only at the upload step).
- [ ] `cargo publish -p rustcmdpev --dry-run` succeeds. This dry-run depends on
  the published `rustcmdpev-core` version and **must be run after** the core
  crate has been published, because cargo resolves the path-pinned
  `rustcmdpev-core` dep against the crates.io index during verification.

## Publish order

The two crates must be published in the following order — `rustcmdpev` depends
on `rustcmdpev-core`, and the published tarball cannot be verified until the
core version it pins is live on crates.io:

1. `cargo publish -p rustcmdpev-core`
2. Wait for the new version to appear on the crates.io index (usually within
   seconds).
3. `cargo publish -p rustcmdpev --dry-run` to verify the CLI tarball builds
   against the freshly published core.
4. `cargo publish -p rustcmdpev`.

If a publish fails mid-sequence, do **not** yank the core release unless
strictly necessary; instead, fix the CLI and bump its patch version.

## README badge verification

The crates.io and docs.rs badges in `README.md` (top of file, lines 3–4) point
to URLs that do not exist until the first published release. Expect them to
render as broken/404 placeholders prior to publish.

After the first publish:

- [ ] Confirm `https://crates.io/crates/rustcmdpev` resolves and the version
  badge shows the published version.
- [ ] Confirm `https://docs.rs/rustcmdpev` resolves and the docs badge shows
  the docs.rs build status (allow a few minutes for docs.rs to build).
