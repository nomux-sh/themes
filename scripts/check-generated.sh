#!/usr/bin/env bash
# Fails when the JSON Schema or the token reference does not agree with the
# token set. To update the files, run scripts/generate.sh.
#
# Usage: scripts/check-generated.sh
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

scripts/generate.sh
if ! git diff --exit-code -- schema/theme.schema.json docs/tokens.md; then
  echo "error: the generated files are stale. Run scripts/generate.sh and commit the result." >&2
  exit 1
fi
