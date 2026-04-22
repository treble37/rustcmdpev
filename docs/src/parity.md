# Parity Checklist

This checklist summarizes feature parity with gocmdpev based on its README. See `CODEBASE_OVERVIEW.md` for more context.

- [x] Accepts PostgreSQL EXPLAIN JSON via stdin and renders a tree.
- [x] Documents a macOS pbpaste + psql workflow.
- [ ] Documents a Rust install equivalent to `go get -u` (e.g., `cargo install`).
- [x] Documents a Homebrew install path or explicit roadmap entry.
- [ ] Provides Python 3 bindings via a build target.
- [ ] Provides Ruby on Rails integration guidance (gem).
- [ ] Includes a bundled `example.json` sample file.

## MVP v1 stdin JSON contract

This section defines the parity contract for stdin ingestion in MVP v1.

### Input source

- If `--input <PATH>` is not provided, `rustcmdpev` reads from stdin.
- Stdin is read as UTF-8 text.

### Accepted JSON shape

- Payload must be valid JSON.
- Top-level value must be a JSON array.
- Array must contain at least one explain object.
- The first explain object must contain `Plan` as an object.
- Typical upstream-compatible payload is PostgreSQL:
  `EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON)`.

### Field handling

- Known fields are parsed into the internal model.
- Missing optional fields are tolerated via serde defaults.
- Unknown extra fields are ignored.

### Error contract

- Empty stdin is an error and must return non-zero exit code.
- Invalid JSON is an error and must return non-zero exit code.
- Unsupported top-level shape (not an array / empty array / missing `Plan`) is an error and must return non-zero exit code.
- Errors must be printed to stderr with actionable text.

### Determinism expectations

- Given identical input JSON and flags, output must be deterministic.
- Contract applies to parity mode (`--compat`) and default pretty mode.

## MVP v1 parity contract (2026-04-22)

This section is the dated parity contract for the MVP v1 release target. It tracks the MVP parity scope against the requirements in `requirements/PRODUCT_REQUIREMENTS.md` and is the source of truth for release sign-off.

| Parity item | Status | Owner | Target version | Acceptance check | Release gate |
| --- | --- | --- | --- | --- | --- |
| Define a parity scope contract | Complete | Docs | v1.0.0 | `docs/src/parity.md` contains a dated MVP v1 contract section with tracked parity items. | Pass |
| Golden parity harness against upstream behavior | Planned | Core | v1.0.0 | At least four normalized parity fixtures run in CI and compare output snapshots against expected results. | Fail |
| Strict CLI input and error contract | Complete | CLI | v1.0.0 | CLI integration tests cover valid stdin, invalid JSON, empty stdin, and contract-violating payloads with non-zero exits. | Pass |
| Output compatibility mode | Complete | CLI | v1.0.0 | `--compat` mode is implemented, documented, and validated by CLI tests for allowed and rejected flag combinations. | Pass |
| Bundled sample file parity | Planned | Docs | v1.0.0 | Repository ships `example.json` and docs/CI validate `cat example.json | rustcmdpev`. | Fail |
| Install and distribution parity | Planned | Release | v1.0.0 | README/docs include release-ready install guidance and the release checklist records install smoke verification. | Fail |
| Python and Rails parity decision | Planned | Product | v1.0.0 | `docs/src/parity.md` records whether Python bindings and Rails integration are included or deferred for MVP v1. | Fail |
| Cross-platform workflow documentation | Planned | Docs | v1.0.0 | README or docs contain tested macOS, Linux, PowerShell, and CMD `psql` workflows. | Fail |
| Deterministic rendering and panic-free behavior | Partial | Core | v1.0.0 | Parsing/processing returns typed errors and deterministic render snapshots pass across supported environments. | Fail |
| Versioned parity sign-off | Planned | Release | v1.0.0 | Release process attaches a completed parity report artifact and blocks v1.0.0 when any `P0` item remains incomplete. | Fail |
