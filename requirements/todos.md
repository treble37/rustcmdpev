# Todos

Single consolidated checklist for MVP v1 feature parity with `gocmdpev`, plus immediate hardening and follow-on work.

## P0 (must ship for MVP v1)

### G0: Parity contract and release gating
- [ ] Add a dated MVP v1 parity contract section in `docs/src/parity.md`.
- [ ] For each parity item, record `status`, `owner`, and `target version`.
- [ ] Mark each parity item as binary pass/fail with explicit acceptance checks.
- [ ] Create a parity report artifact template for release.
- [ ] Block v1.0 release if any P0 parity requirement is incomplete.
- [ ] Link parity report from release notes/checklist.

### G1: CLI, input, and output compatibility
- [x] Add `clap`-based CLI with documented flags and help output.
- [x] Define and document stdin JSON contract for MVP parity behavior.
- [x] Support input from file (`--input`) in addition to stdin.
- [x] Add `--compat` mode for parity-target rendering behavior.
- [x] Support output formats required for parity (`pretty`, `json`, `table`).
- [x] Support `--color auto|always|never`, TTY detection, and `NO_COLOR`.
- [ ] Ensure Windows console color behavior works correctly.
- [x] Add verbose/quiet modes and structured logging (`tracing` + `tracing-subscriber`).

### G2: Safety and error handling
- [x] Remove panic paths (`unwrap`/`expect`) from core parse/process flow for valid inputs.
- [x] Return typed errors from core processing path and map them to CLI exit codes.
- [x] Return non-zero exit code for empty stdin input with actionable error text.
- [x] Return non-zero exit code for invalid JSON with actionable error text.
- [x] Validate input JSON structure and required plan invariants before processing.
- [ ] Add max plan depth / large-input guards to prevent failure on extreme plans.

### G3: Core model and processing correctness
- [x] Implement a clear parser pipeline: raw JSON -> domain model -> validated plan tree.
- [x] Keep strongly typed serde models for Postgres EXPLAIN fields with version-tolerant optional fields.
- [x] Add schema-aware handling for Postgres version field differences.
- [x] Represent plan structure with explicit node/tree semantics and invariant checks.
- [x] Separate analysis logic (estimates, actuals, outliers) from rendering logic.
- [x] Extract hardcoded thresholds/labels/tree characters into named constants.

### G4: Rendering determinism and parity testing
- [ ] Add fixture-based parity tests comparing rendered output to expected snapshots.
- [ ] Add at least 4 parity fixtures, including `example.json` and 3 real-world plans.
- [ ] Normalize ANSI and insignificant whitespace in parity comparisons.
- [ ] Run parity snapshot tests in `--compat` mode.
- [ ] Add deterministic rendering snapshots to verify stable output across runs.
- [ ] Add integration tests for success and failure exit codes.
- [ ] Gate merges in CI on parity test pass.
- [ ] Verify deterministic `--compat` output on Linux/macOS/Windows in CI.

### G5: CI/CD and release essentials
- [ ] Replace `.travis.yml` with GitHub Actions CI for `build`, `test`, `clippy`, and `fmt`.
- [ ] Add CI matrix for Linux/macOS/Windows.
- [ ] Define and enforce MSRV in CI.
- [ ] Publish versioned releases with binaries for Linux/macOS/Windows.
- [ ] Establish semantic versioning and tag conventions.

### G6: Documentation and install baseline
- [ ] Add `cargo install rustcmdpev` instructions to README/docs.
- [ ] Add Homebrew install path or explicit Homebrew roadmap entry.
- [ ] Add release checklist item for install smoke verification.
- [ ] Add Linux/macOS/Windows psql workflow examples (including PowerShell/CMD where needed).
- [ ] Validate documented command examples in CI/docs checks.
- [ ] Add `example.json` as a bundled sample plan fixture.
- [ ] Add CI smoke test for `cat example.json | rustcmdpev`.

## P1 (should ship shortly after MVP cut)

### G7: Architecture and maintainability
- [x] Convert to a workspace with `rustcmdpev-core` (lib) and `rustcmdpev` (bin).
- [ ] Finalize render module boundaries (`display/colors`, `display/tree`, `display/format`).
- [ ] Refactor large `Plan` model into grouped sub-structs where practical.
- [ ] Improve function signatures by introducing context structs and reducing large parameter lists.
- [ ] Reduce unnecessary cloning and ownership churn in hot paths.

### G8: Output features and UX polish
- [ ] Add theme support (`dark`, `light`, `no-color`) and condensed/verbose render modes.
- [ ] Add summary block improvements (cost, time, loops, memory, buffers).
- [ ] Add customization for tree drawing characters/styles.
- [ ] Add `--postgres-version` hinting for parser quirks.

### G9: Quality and testing depth
- [ ] Expand unit tests for parsing oddities and calculation functions.
- [ ] Expand integration tests across diverse PostgreSQL plan types.
- [ ] Add property tests for plan invariants (e.g., non-negative costs, child consistency).
- [ ] Add fixture utilities and helpers for assertions.
- [ ] Add performance benchmarks for parsing/rendering.
- [ ] Enable strict linting policy (`clippy`, warning policy) and project rustfmt config.

### G10: Distribution and project hygiene
- [x] Add automated changelog generation (`git-cliff`).
- [x] Publish prebuilt tarballs with checksums (and package metadata for platform managers).
- [x] Add license/header and dependency policy checks (`cargo-deny`).
- [x] Add `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, and issue templates.
- [x] Add README badges for crates/docs/CI and improve quickstart docs.
- [x] Add `examples/` plans and a reproducible local demo target.
- [x] Add API/module-level docs for public surfaces and complex logic.

## P2 (backlog and optional extensions)

### G11: Performance and optimization
- [ ] Investigate zero-copy deserialization opportunities (`Cow<'_, str>`) where beneficial.
- [ ] Support streaming JSON parse from reader for very large plans.
- [ ] Pre-compute aggregate metrics once and reuse in rendering.
- [ ] Optimize string allocation patterns (`with_capacity`, reuse buffers, reduced concatenation).
- [ ] Evaluate iterative traversal/memoization for deep tree performance.
- [ ] Reduce allocations in hot paths (reuse objects/buffers where justified).

### G12: Dependency and modernization follow-up
- [ ] Continue modernizing Rust patterns (`?`, targeted `const fn`, idiomatic APIs).
- [ ] Audit and update dependency set; evaluate replacing `phf` if maintainability improves.
- [ ] Add automated dependency update workflow.
- [ ] Add CI coverage reporting.

### G13: Feature extensions (non-MVP)
- [ ] Decide and document MVP posture for Python bindings (`include` or `defer`) with date/owner/rationale.
- [ ] Decide and document MVP posture for Rails integration (`include` or `defer`) with date/owner/rationale.
- [ ] Evaluate optional direct Postgres connection mode (feature-flagged).
- [ ] Evaluate optional HTML export.
- [ ] Evaluate optional CSV export.
- [ ] Evaluate optional opt-in anonymous usage telemetry (off by default).

### G14: Advanced docs and i18n ideas
- [ ] Improve inline comments for complex algorithms and design tradeoffs.
- [ ] Expand duration formatting options and user preference support.
- [ ] Evaluate making operator descriptions externally configurable and i18n-ready.
