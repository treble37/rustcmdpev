# Contributing

## Development setup

1. Install stable Rust.
2. Run `cargo test` to verify the workspace.
3. Run `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings` before opening a PR.

## Working norms

- Keep changes scoped and explain the behavior change in the PR description.
- Add or update tests whenever behavior changes.
- Update `requirements/todos.md`, docs, examples, and release notes when user-facing behavior changes.
- Prefer small commits with descriptive messages.

## Demo and examples

- Run `cargo demo` for a reproducible local example.
- Example plans live in `examples/` and should stay small, portable, and anonymized.

## Release hygiene

- Tag releases with `vMAJOR.MINOR.PATCH`.
- Generate changelog previews with the `Changelog` workflow or `git cliff --config cliff.toml`.
- Validate dependency policy locally with `cargo deny check --all-features`.
