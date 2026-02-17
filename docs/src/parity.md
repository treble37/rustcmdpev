# Parity Checklist

This checklist summarizes feature parity with gocmdpev based on its README. See `CODEBASE_OVERVIEW.md` for more context.

- [x] Accepts PostgreSQL EXPLAIN JSON via stdin and renders a tree.
- [x] Documents a macOS pbpaste + psql workflow.
- [ ] Documents a Rust install equivalent to `go get -u` (e.g., `cargo install`).
- [ ] Documents a Homebrew install path.
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
