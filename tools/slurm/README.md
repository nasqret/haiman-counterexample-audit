# WMI/SLURM operations

This project uses the WMI gateway alias `wmicluster`. Heavy computation must run under `sbatch` or `srun`, never directly on the gateway.

Before any submission:

1. inspect `squeue` and recent `sacct`;
2. check for an equivalent active/completed project job;
3. benchmark local memory and time;
4. validate the exact input revision;
5. record job ID, commit, command, resources, and output paths in `results/`.

The project does not own unrelated jobs visible in the user's queue and will not alter them.

## Portable certificate cross-check

`haiman_crosscheck.sbatch` runs a standard-library verifier on one CPU with a
1 GB, five-minute limit. It reconstructs the matrix and canonical SHA-256
payload, rather than trusting a local CAS export. The reviewed sync-and-submit
entrypoint is:

```bash
bash tools/slurm/sync_and_submit_crosscheck.sh
```

It prints exactly one SLURM job ID. Inspect that ID before resubmitting, and
fetch `results/cluster/slurm-<job-id>.out` after completion.
