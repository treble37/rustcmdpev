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
