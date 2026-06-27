#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
HOST="${HAIMAN_SLURM_HOST:-wmicluster}"
REMOTE_DIR="${HAIMAN_SLURM_DIR:-codex-haiman-audit-wmi}"

ssh "$HOST" "mkdir -p ~/$REMOTE_DIR/results/cluster"
rsync -az \
  --exclude '.git/' \
  --exclude '.venv/' \
  --exclude '.cache/' \
  --exclude 'Lee_principal_component_not_CM.pdf' \
  --exclude 'artifacts/rust/target/' \
  --exclude 'book/_build/' \
  --exclude 'output/' \
  --exclude 'tmp/' \
  "$ROOT/" "$HOST:$REMOTE_DIR/"

ssh "$HOST" "cd ~/$REMOTE_DIR && sbatch --parsable tools/slurm/haiman_crosscheck.sbatch"
