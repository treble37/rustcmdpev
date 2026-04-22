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
