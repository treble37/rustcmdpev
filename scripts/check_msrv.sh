#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <msrv-version>" >&2
  exit 2
fi

expected_msrv="$1"

for manifest in rustcmdpev/Cargo.toml rustcmdpev-core/Cargo.toml; do
  actual_msrv="$(sed -nE 's/^rust-version = "([^"]+)"$/\1/p' "$manifest")"

  if [[ -z "$actual_msrv" ]]; then
    echo "missing rust-version in $manifest" >&2
    exit 1
  fi

  if [[ "$actual_msrv" != "$expected_msrv" ]]; then
    echo "rust-version mismatch in $manifest: expected $expected_msrv, found $actual_msrv" >&2
    exit 1
  fi
done
