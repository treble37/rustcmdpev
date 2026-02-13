# Product Requirements: MVP v1 (Feature Parity with gocmdpev)

## Goal
Port `rustcmdpev` to achieve practical feature parity with `gocmdpev` for an MVP v1 release.

## Scope
MVP v1 focuses on parity-critical behavior, compatibility, testability, and installability. Large architectural refactors and optional integrations are out of scope unless required to meet parity.

## Requirements

### 1) Define a parity scope contract
- The project must define a versioned parity contract in `docs/src/parity.md`.
- The contract must clearly state what is included in MVP v1 and what is deferred.
- Every parity requirement must map to a binary pass/fail check.

Acceptance criteria:
- `docs/src/parity.md` includes a dated MVP v1 section.
- Each parity feature has `status`, `owner`, and `target version`.

### 2) Golden parity harness against upstream behavior
- The project must include fixture-based output parity tests.
- Tests should use upstream-style plans (`example.json` plus additional real plans).
- Output comparisons should normalize ANSI colors and insignificant whitespace.

Acceptance criteria:
- At least 4 fixtures are covered.
- Snapshot/parity tests run in CI and gate merges.

### 3) Strict CLI input and error contract
- CLI must support stdin JSON workflows equivalent to current upstream usage.
- Invalid JSON and empty input must produce non-zero exits and actionable errors.
- CLI behavior must be deterministic and documented.

Acceptance criteria:
- Integration tests validate success and failure exit codes.
- Error messages are documented and tested.

### 4) Output compatibility mode
- Add a `--compat` mode to preserve parity-target rendering semantics.
- `--compat` mode should prioritize stable output over new formatting changes.

Acceptance criteria:
- Parity harness runs with `--compat`.
- `--compat` snapshots are deterministic across supported OSes.

### 5) Bundled sample file parity
- Repository must ship an `example.json` sample explain plan for quickstart and tests.

Acceptance criteria:
- `cat example.json | rustcmdpev` works in docs and CI smoke checks.

### 6) Install and distribution parity
- Provide `cargo install` documentation and release-ready installation guidance.
- Provide Homebrew installation path or explicit Homebrew roadmap if not yet live.

Acceptance criteria:
- README includes validated install instructions.
- Release checklist includes install smoke verification.

### 7) Python and Rails parity decision
- Explicitly decide whether Python bindings and Rails integration are in MVP v1 or deferred.
- Record this decision with rationale.

Acceptance criteria:
- `docs/src/parity.md` records inclusion/defer decision with date and owner.

### 8) Cross-platform workflow documentation
- Document psql-to-tool workflows for Linux, macOS, and Windows.
- Document shell-specific examples where required (PowerShell/CMD/Bash).

Acceptance criteria:
- README or docs include tested commands for all 3 OSes.

### 9) Deterministic rendering and panic-free behavior
- Tool should avoid panics for valid explain JSON.
- Rendering should be stable for the same input across runs and OSes.

Acceptance criteria:
- Core parsing/processing path returns typed errors.
- Snapshot tests confirm deterministic output.

### 10) Versioned parity sign-off
- MVP release process must include a parity report artifact.
- Parity report must enumerate pass/fail status of parity items.

Acceptance criteria:
- Release is blocked when parity P0 items are not complete.
- Parity report is linked from release notes.

## Non-goals for MVP v1
- Full workspace restructuring unless necessary for parity.
- Telemetry features.
- Direct Postgres connection mode.
- Deep micro-optimizations that do not materially affect parity.

## Dependencies
- Upstream behavior reference: https://github.com/simon-engledew/gocmdpev
- Internal parity tracker: `docs/src/parity.md`
- Existing architecture context: `CODEBASE_OVERVIEW.md`
