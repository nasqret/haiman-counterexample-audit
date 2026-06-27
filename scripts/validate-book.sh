#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

EXECUTE_NOTEBOOKS="${EXECUTE_NOTEBOOKS:-1}"
BUILD_ALL="${BUILD_ALL:-1}"
STRICT="${STRICT:-1}"

if [[ "$EXECUTE_NOTEBOOKS" == "1" ]]; then
  ./scripts/execute-notebooks.sh
fi

args=(build book)
if [[ "$BUILD_ALL" == "1" ]]; then
  args+=(--all)
fi
if [[ "$STRICT" == "1" ]]; then
  args+=(-W -n)
fi

if [[ -x "$ROOT/.venv/bin/jupyter-book" ]]; then
  "$ROOT/.venv/bin/jupyter-book" "${args[@]}"
else
  jupyter-book "${args[@]}"
fi
