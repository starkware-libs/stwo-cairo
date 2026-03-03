#!/bin/bash

set -eou pipefail

# Crates to exclude from stable check.
EXCLUDE=(
  stwo-cairo-prover
  stwo-cairo-dev-utils
)

crates=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[].name' | sort -u)
for ex in "${EXCLUDE[@]}"; do
  crates=$(echo "$crates" | grep -v "^${ex}$" || true)
done

stable=""
not_stable=""

for crate in $crates; do
  if cargo +stable check --package "$crate"; then
    stable+="$crate, "
  else
    not_stable+="$crate, "
  fi
done

echo "Stable: ${stable%, }"

if [ -n "$not_stable" ]; then
  echo "Not stable: ${not_stable%, }"
  exit 1
fi
