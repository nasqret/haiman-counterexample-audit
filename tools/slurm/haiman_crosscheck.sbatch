#!/usr/bin/env bash
#SBATCH --job-name=hca-cert
#SBATCH --partition=cpu
#SBATCH --cpus-per-task=1
#SBATCH --mem=1G
#SBATCH --time=00:05:00
#SBATCH --output=results/cluster/slurm-%j.out

set -euo pipefail
cd "$SLURM_SUBMIT_DIR"

printf 'host=%s\n' "$(hostname)"
printf 'job_id=%s\n' "$SLURM_JOB_ID"
python3 --version
python3 artifacts/common/verify_specialization.py
sha256sum \
  artifacts/common/verify_specialization.py \
  results/certificates/lemma19_nonzero_minor.json
