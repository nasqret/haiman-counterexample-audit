#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export IPYTHONDIR="${IPYTHONDIR:-$ROOT/.cache/ipython}"
export JUPYTER_CONFIG_DIR="${JUPYTER_CONFIG_DIR:-$ROOT/.cache/jupyter}"
mkdir -p "$IPYTHONDIR" "$JUPYTER_CONFIG_DIR"

if [[ -x "$ROOT/.venv/bin/python" ]]; then
  JB_PYTHON="${JB_PYTHON:-$ROOT/.venv/bin/python}"
else
  JB_BIN="$(command -v jupyter-book)"
  JB_PYTHON="${JB_PYTHON:-$(sed -n '1s/^#!//p' "$JB_BIN")}" 
fi

"$JB_PYTHON" scripts/execute_notebooks.py --timeout 180 book/notebooks/*.ipynb
