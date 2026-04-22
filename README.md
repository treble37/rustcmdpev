# Rustcmdpev

[![Crates.io](https://img.shields.io/crates/v/rustcmdpev.svg)](https://crates.io/crates/rustcmdpev)
[![Docs.rs](https://docs.rs/rustcmdpev/badge.svg)](https://docs.rs/rustcmdpev/latest/rustcmdpev/)
[![CI](https://github.com/treble37/rustcmdpev/actions/workflows/ci.yml/badge.svg)](https://github.com/treble37/rustcmdpev/actions/workflows/ci.yml)
[![Release](https://github.com/treble37/rustcmdpev/actions/workflows/release.yml/badge.svg)](https://github.com/treble37/rustcmdpev/actions/workflows/release.yml)

A command-line Rust Postgres query visualizer, heavily inspired by the excellent (web-based) [pev](https://github.com/AlexTatiyants/pev).
It started out being ported from [gocmdpev](https://github.com/simon-engledew/gocmdpev)

# Demo

![rustcmdpev screenshot](https://user-images.githubusercontent.com/777964/96496883-ad318080-11fe-11eb-8bbb-b81a52676787.png)

# Documentation

- Deep dive: `requirements/CODEBASE_OVERVIEW.md`
- Docs site source: `docs/`
- Docs site instructions: `docs/README.md`
- Build the docs site (requires `mdbook`):

```bash
mdbook build docs
```

# Installation

Right now the installation process is still source-first, but the repository is prepared for packaged releases.

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Build the workspace:

```bash
cargo build --release -p rustcmdpev
```

3. Run the binary:

```bash
./target/release/rustcmdpev --help
```

Tagged releases publish platform archives with checksums through `.github/workflows/release.yml`.

# Quickstart

1. Build the CLI:

```bash
cargo build --release -p rustcmdpev
```

2. Run the bundled example plan:

```bash
./target/release/rustcmdpev --input examples/hash_join.json --format pretty --width 80
```

3. Or use the reproducible demo alias:

```bash
cargo demo
```

# Overview

## Usage

Generate a query plan with all the trimmings by prefixing your query with:

```pgsql
EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON)
```

Then pipe the resulting query plan into `rustcmdpev`.

On MacOS you can just grab a query on your clipboard and run this one-liner:

```bash
pbpaste | sed '1s/^/EXPLAIN (ANALYZE, COSTS, VERBOSE, BUFFERS, FORMAT JSON) /' | psql -qXAt <DATABASE> | rustcmdpev
```

Quickstart with a bundled example:

```bash
cargo run -p rustcmdpev -- --input examples/hash_join.json --format pretty --width 80
```

Reproducible local demo alias:

```bash
cargo demo
```

### Stdin JSON contract (MVP parity)

- When `--input` is not provided, stdin must contain JSON text.
- Top-level JSON must be an array with at least one object containing `Plan`.
- Empty stdin, invalid JSON, or unsupported top-level shape are contract errors and should exit non-zero.

Source of truth: `docs/src/parity.md` -> "MVP v1 stdin JSON contract".

### CLI flags

```bash
rustcmdpev [--input <PATH>] [--format pretty|json|table] [--color auto|always|never] [--width <N>] [--compat]
```

- `--input, -i <PATH>`: read EXPLAIN JSON from a file instead of stdin
- `--format`: output format (`pretty`, `json`, `table`)
- `--color`: color policy (`always` force ANSI, `never` disable, `auto` = TTY-detect and respect `NO_COLOR`)
- `--width`: tree render width (default: `60`)
- `--compat`: parity-target mode (`--format pretty` only, legacy width `60`)
- `-v, --verbose`: increase log verbosity (`warn` default, `-v` = `info`, `-vv`+ = `debug`)
- `-q, --quiet`: reduce logs to `error` only
- `RUST_LOG`: override log filter via `tracing-subscriber` env filter syntax

### Exit codes

- `0`: success
- `2`: input read error (stdin/file)
- `3`: invalid/contract-violating input payload
- `4`: invalid compatibility flag combination
- `5`: output serialization error
- `6`: core processing/render error

Run help:

```bash
rustcmdpev --help
```

### Examples

- `examples/basic_seq_scan.json`: smallest useful sample plan for smoke tests and docs.
- `examples/hash_join.json`: nested tree sample for demos and screenshots.

## Local development

### View sample output

```bash
cargo demo
```

## Testing

To see output from print statements, run with nocapture flag:

`cargo test -- --nocapture`

Project hygiene commands:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo deny check --all-features`
- `git cliff --config cliff.toml`
