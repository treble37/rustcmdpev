#!/usr/bin/env bash

set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <tag>" >&2
  exit 2
fi

tag="$1"
semver_pattern='^v[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$'

if [[ ! "$tag" =~ $semver_pattern ]]; then
  echo "release tag '$tag' does not match the required format vMAJOR.MINOR.PATCH or vMAJOR.MINOR.PATCH-PRERELEASE" >&2
  exit 1
fi
