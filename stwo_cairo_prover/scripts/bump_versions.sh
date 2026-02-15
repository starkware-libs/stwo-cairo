#!/usr/bin/env bash

set -euo pipefail

CURRENT_VERSION='1.0.1'
NEW_VERSION="$@"

if [ -z "$NEW_VERSION" ]; then
  echo "usage: $0 <new_version>" >&2
  exit 1
fi

# NOTE: This blindly replaces the version string; it WILL update any crate entries
# that share the same CURRENT_VERSION. Review other crates manually if needed.
sed -i "s/$CURRENT_VERSION/$NEW_VERSION/g" \
    $(find .. -type f \( -name "Cargo.toml" \)) \
    ./scripts/bump_versions.sh
