# Versioning and Tags

`rustcmdpev` follows semantic versioning for crate releases and GitHub release tags.

## Versioning policy

- Use `MAJOR.MINOR.PATCH` crate versions.
- Increment `MAJOR` for breaking CLI, output, or library API changes.
- Increment `MINOR` for backward-compatible features and behavior expansions.
- Increment `PATCH` for backward-compatible fixes, docs-only corrections, and release tooling updates.

## Tag conventions

- Stable releases use tags in the form `vMAJOR.MINOR.PATCH`, for example `v1.2.3`.
- Pre-release builds use tags in the form `vMAJOR.MINOR.PATCH-PRERELEASE`, for example `v1.2.3-rc.1`.
- Release automation only publishes from tags that match those conventions.

## Release expectations

- Keep the crate versions in `rustcmdpev/Cargo.toml` and `rustcmdpev-core/Cargo.toml` aligned with the intended release series.
- Create and push the release tag only after the release checklist and parity report are complete.
- Treat the tag as the source of truth for the published archive names and GitHub release title.

## Pre-1.0 stability posture

While the crate version is in the `0.x.y` series, downstream consumers should
treat both `rustcmdpev` and `rustcmdpev-core` as **pre-stable**:

- The CLI surface (flags, output formats, exit codes) and the `rustcmdpev-core`
  library API may change between `0.MINOR` releases.
- Patch releases (`0.x.y` → `0.x.(y+1)`) remain backward-compatible bug fixes.
- A `1.0.0` release will be cut once the MVP v1 parity contract has been
  stable across at least one minor cycle and the public library API has been
  reviewed for long-term commitments.
- Pin to a fully-qualified `0.x.y` (not `0.x`) when depending on
  `rustcmdpev-core` from another crate during the pre-1.0 window.
