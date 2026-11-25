#!/usr/bin/env bash
# Writes the files that come from the token set: the JSON Schema of a theme
# file and the token reference.
#
# Usage: scripts/generate.sh
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

cargo run --quiet --locked -p nomux-theme -- schema > schema/theme.schema.json
cargo run --quiet --locked -p nomux-theme -- reference > docs/tokens.md
