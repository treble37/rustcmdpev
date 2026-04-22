#!/usr/bin/env bash

set -euo pipefail

todos_file="${1:-requirements/todos.md}"

if [[ ! -f "$todos_file" ]]; then
  echo "error: todos file not found: $todos_file" >&2
  exit 2
fi

unchecked_p0_items="$(
  awk '
    /^## P0 / { in_p0 = 1; next }
    /^## P[0-9]+ / && in_p0 { in_p0 = 0 }
    in_p0 && /^- \[ \]/ { print }
  ' "$todos_file"
)"

if [[ -n "$unchecked_p0_items" ]]; then
  echo "error: v1.0 release is blocked because P0 items remain incomplete:" >&2
  echo "$unchecked_p0_items" >&2
  exit 1
fi

echo "P0 release gate passed: all P0 todos are complete."
