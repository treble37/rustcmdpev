# Install

`rustcmdpev` supports both a published crates.io install path and a local workspace install path.

## Install from crates.io

Use this after a published release is available:

```bash
cargo install rustcmdpev --locked
```

Then verify the binary is available:

```bash
rustcmdpev --help
```

## Install from the local workspace

Use this when working from a checkout before a release is published:

```bash
cargo install --path rustcmdpev --locked
```

Then verify the local install:

```bash
rustcmdpev --help
```

## Alternative: build without installing

If you only want a local release binary in the repo checkout:

```bash
cargo build --release -p rustcmdpev
./target/release/rustcmdpev --help
```

## Homebrew roadmap

Homebrew packaging is not published yet.

- Short term: use `cargo install rustcmdpev --locked` or download a tagged release archive.
- Roadmap: add a Homebrew tap/formula after the `v1.0.0` release process is stable and install smoke checks are part of release verification.
