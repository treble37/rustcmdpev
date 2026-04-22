#!/usr/bin/env bash

set -euo pipefail

cargo run -p rustcmdpev -- --help >/dev/null
cargo run -p rustcmdpev -- --input example.json --format pretty --width 80 >/dev/null
cargo demo >/dev/null
