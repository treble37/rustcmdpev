# Rustcmdpev

[![Build Status](https://travis-ci.org/treble37/rustcmdpev.svg)](https://travis-ci.org/treble37/rustcmdpev)

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

Right now the installation process is manual and assumes you can compile the rust source to a binary executable for your system.

1. [Install Rust](https://www.rust-lang.org/tools/install) and clone the repo
2. Build the binary executable
3. Add the executable to your system `$PATH` variable

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
- `-v, --verbose`: increase verbosity level (`-vv` supported)
- `-q, --quiet`: quiet mode flag

Run help:

```bash
rustcmdpev --help
```

## Local development

### View sample output

```
echo '[{"Plan":{"Alias":"c0","Node Type":"Seq Scan","Parallel Aware":false,"Plan Rows":50,"Plan Width":1572,"Relation Name":"coaches","Startup Cost":0.0,"Total Cost":10.5}}]' | cargo run -p rustcmdpev -- --format pretty --width 80
```

## Testing

To see output from print statements, run with nocapture flag:

`cargo test -- --nocapture`
