# rustcmdpev docs

## Install mdBook (macOS + Linux)

Preferred (works on macOS and Linux if you have Rust installed):

```bash
cargo install mdbook
```

Ensure Cargo bin is on your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Alternative (macOS):

```bash
brew install mdbook
```

Alternative (Linux with Rust via asdf):

```bash
cargo install mdbook
asdf reshim rust
```

Ensure the asdf shims are on PATH (typical setup):

```bash
export PATH="$HOME/.asdf/shims:$PATH"
```

## Local preview

```bash
mdbook serve docs
```

## Build

```bash
mdbook build docs
```

## Output directory

- `docs/book/`
