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
