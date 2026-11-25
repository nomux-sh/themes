#!/usr/bin/env bash
# Prints the release notes of one version: the section of the version in
# CHANGELOG.md and the minimum supported Rust version.
#
# Usage: scripts/release-notes.sh <version>, for example 0.1.0.
set -euo pipefail

version="$1"

cd "$(git rev-parse --show-toplevel)"

section="$(awk -v heading="## [$version]" '
  index($0, heading) == 1 { found = 1; next }
  found && /^## \[/ { exit }
  found { print }
' CHANGELOG.md)"
if [ -z "${section//[[:space:]]/}" ]; then
  echo "error: CHANGELOG.md has no section for version $version." >&2
  exit 1
fi

rust_version="$(sed -n 's/^rust-version = "\(.*\)"$/\1/p' crates/nomux-theme/Cargo.toml)"

printf '%s\n\nMinimum supported Rust version: %s\n' "$section" "$rust_version"
