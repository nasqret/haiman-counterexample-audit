#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

python3 -m json.tool STATUS.json >/dev/null
python3 -m json.tool sources/source-manifest.json >/dev/null
python3 -m json.tool book/book-spec.json >/dev/null
python3 -m json.tool book/notebooks/matrix_counts.ipynb >/dev/null

./scripts/execute-notebooks.sh

if command -v sage >/dev/null 2>&1; then
  mkdir -p "$ROOT/.cache/home" "$ROOT/.cache/sage"
  env HOME="$ROOT/.cache/home" SAGE_HOME="$ROOT/.cache/sage" \
    sage artifacts/sage/generate_matrix.sage
  env HOME="$ROOT/.cache/home" SAGE_HOME="$ROOT/.cache/sage" \
    sage artifacts/sage/enumerate_predecessors.sage
fi

if [[ -x "$ROOT/.venv/bin/python" ]]; then
  "$ROOT/.venv/bin/python" artifacts/common/analyze_minor_weight.py
else
  python3 artifacts/common/analyze_minor_weight.py
fi

cargo test --manifest-path artifacts/rust/Cargo.toml
cargo run --quiet --locked --manifest-path artifacts/rust/Cargo.toml

if command -v Singular >/dev/null 2>&1; then
  singular_output="$(Singular -q artifacts/singular/verify_minor.sing)"
  printf '%s\n' "$singular_output"
  grep -q '"implementation":"Singular".*"status":"verified"' <<<"$singular_output"
fi

if command -v julia >/dev/null 2>&1 && [[ -f artifacts/oscar/Manifest.toml ]]; then
  oscar_output="$(julia --project=artifacts/oscar artifacts/oscar/verify_minor.jl)"
  printf '%s\n' "$oscar_output"
  grep -q '"implementation":"Oscar.jl".*"status":"verified"' <<<"$oscar_output"
fi

echo "Local certificate validation passed. Representation and minimality stages remain open."
