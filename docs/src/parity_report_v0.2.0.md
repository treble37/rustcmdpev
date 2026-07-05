# Parity Report — v0.2.0

Completed from the [parity report template](parity_report_template.md) against the
MVP v1 parity contract in [`parity.md`](parity.md) (dated 2026-04-22). The contract
targets v1.0.0; this report records the pass/fail status of each item as shipped in
v0.2.0.

## Release metadata

- Release version: 0.2.0
- Release date: pending (set when the `v0.2.0` tag is cut)
- Commit/tag: `v0.2.0` (pending)
- Report owner: Bruce Park (treble37)

## Summary

- Overall parity decision: `pass`
- Blocking `P0` gaps: none — the P0 release gate passed on 2026-07-05
- Notes: the Python/Rails posture decision is the only open contract item; it is a
  non-P0 backlog item with a decision ADR scheduled post-release. The `requirements/`
  directory (including `todos.md`, the P0 gate input) now lives in the project vault,
  so `scripts/check_p0_release_gate.sh` must be given the vault path explicitly.

## Parity item results

| Parity item | Status | Acceptance check | Result | Evidence |
| --- | --- | --- | --- | --- |
| Define a parity scope contract | Complete | `docs/src/parity.md` contains a dated MVP v1 contract section with tracked parity items. | Pass | `docs/src/parity.md`, "MVP v1 parity contract (2026-04-22)" section. |
| Golden parity harness against upstream behavior | Complete | At least four normalized parity fixtures run in CI and compare output snapshots against expected results. | Pass | Four snapshot fixtures (`example`, `real_world_bitmap_heap_scan`, `real_world_hash_join`, `real_world_nested_loop`) in `rustcmdpev/tests/fixtures/parity/`, driven by `tests/parity_snapshots.rs`; CI `parity` job runs them on ubuntu, macOS, and Windows. |
| Strict CLI input and error contract | Complete | CLI integration tests cover valid stdin, invalid JSON, empty stdin, and contract-violating payloads with non-zero exits. | Pass | `tests/cli_stdin_contract.rs`, `tests/cli_invalid_json.rs`, `tests/cli_exit_codes.rs`, `tests/parity_exit_codes.rs`; error fixtures `invalid_json.json`, `invalid_shape.json`. |
| Output compatibility mode | Complete | `--compat` mode is implemented, documented, and validated by CLI tests for allowed and rejected flag combinations. | Pass | `tests/cli_compat.rs`; flag documented in `--help` and docs. |
| Bundled sample file parity | Complete | Repository ships `example.json` and docs/CI validate `cat example.json \| rustcmdpev`. | Pass | `example.json` at repo root; CI `example-smoke` job pipes it through the built binary and asserts "Seq Scan" in the output. |
| Install and distribution parity | Complete | README documents `cargo install rustcmdpev --locked` and a Homebrew roadmap note; release checklist covers install smoke verification. | Pass | README install section; Homebrew roadmap note; post-publish install smoke runs as a release-checklist step after the crates.io publish. |
| Python and Rails parity decision | Deferred | `docs/src/parity.md` records whether Python bindings and Rails integration are included or deferred for MVP v1. | Fail (non-blocking) | Not shipped in v0.2.0 (unchecked in the parity checklist). Formal include/defer posture ADR is scheduled in the post-release milestone; not a `P0` item, so it does not gate this release. |
| Cross-platform workflow documentation | Complete | README or docs contain tested macOS, Linux, PowerShell, and CMD `psql` workflows. | Pass | `docs/src/workflows.md` documents Linux, macOS (`pbpaste`), Windows PowerShell, and Windows CMD pipelines; CI builds and tests the binary on all three OSes. |
| Deterministic rendering and panic-free behavior | Complete | Parsing/processing returns typed errors and deterministic render snapshots pass across supported environments. | Pass | Typed error path in `rustcmdpev-core` (no `unwrap`/`expect` on the valid-input flow); parity snapshots pass on ubuntu, macOS, and Windows CI legs. |
| Versioned parity sign-off | Complete | Release process attaches a completed parity report artifact and blocks the release when any `P0` item remains incomplete. | Pass | This report; `scripts/check_p0_release_gate.sh` passed on 2026-07-05 against the vault `requirements/todos.md`. |

## P0 release gate review

- [x] All `P0` items in `requirements/todos.md` are complete. (Gate script run
  2026-07-05 against the vault copy — the `requirements/` directory moved out of the
  repo — output: "P0 release gate passed: all P0 todos are complete.")
- [x] Attached evidence includes the parity fixtures or equivalent test output
  (see the Evidence column above; full suite green locally on 2026-07-05).
- [ ] Release notes/checklist link this completed parity report. (Staged: link this
  report from the v0.2.0 release notes when the tag is cut.)
