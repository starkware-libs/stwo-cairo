#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Update stwo workspace dependency source for all stwo crates.

Usage:
  scripts/bump_stwo_deps.sh -v <version>
  scripts/bump_stwo_deps.sh -r <commit_hash>

Options:
  -v <version>      Use crates.io dependencies (e.g. 2.1.0)
  -r <commit_hash>  Use git rev dependencies from https://github.com/starkware-libs/stwo
  -h                Show this help
USAGE
}

rev=""
version=""

while getopts ":r:v:h" opt; do
  case "$opt" in
    r)
      rev="$OPTARG"
      ;;
    v)
      version="$OPTARG"
      ;;
    h)
      usage
      exit 0
      ;;
    :)
      echo "Error: -$OPTARG requires a value." >&2
      usage >&2
      exit 2
      ;;
    \?)
      echo "Error: Unknown option -$OPTARG" >&2
      usage >&2
      exit 2
      ;;
  esac
done

shift $((OPTIND - 1))

if [[ -n "$rev" && -n "$version" ]]; then
  echo "Error: use either -r or -v, not both." >&2
  usage >&2
  exit 2
fi

if [[ -z "$rev" && -z "$version" ]]; then
  echo "Error: one of -r or -v is required." >&2
  usage >&2
  exit 2
fi

if [[ $# -gt 0 ]]; then
  echo "Error: unexpected positional arguments: $*" >&2
  usage >&2
  exit 2
fi

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="${SCRIPT_DIR}/.."
CARGO_TOML="${WORKSPACE_ROOT}/Cargo.toml"

if [[ ! -f "$CARGO_TOML" ]]; then
  echo "Error: could not find workspace Cargo.toml at $CARGO_TOML" >&2
  exit 1
fi

deps=(
  stwo
  stwo-types
  stwo-constraint-framework
  stwo-air-utils-derive
  stwo-air-utils
)

for dep in "${deps[@]}"; do
  if [[ -n "$version" ]]; then
    replacement="${dep} = \"${version}\""
  else
    replacement="${dep} = { git = \"https://github.com/starkware-libs/stwo\", rev = \"${rev}\" }"
  fi

  sed -E -i "s|^${dep}[[:space:]]*=.*$|${replacement}|" "$CARGO_TOML"
done

echo "Updated ${CARGO_TOML}:"
for dep in "${deps[@]}"; do
  grep -nE "^${dep}[[:space:]]*=" "$CARGO_TOML"
done
