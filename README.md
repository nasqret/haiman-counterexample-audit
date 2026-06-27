# Haiman conjecture counterexample audit

This repository reconstructs and independently audits the counterexample claimed in Kyungyong Lee, *The singularities of the principal component of the Hilbert scheme of points*, Journal of Algebra 324 (2010), 1347-1363, DOI: 10.1016/j.jalgebra.2010.07.001.

> **Current verdict: open audit.** The paper's conclusion has not yet been independently certified here. One part of Lemma 19 is now certified: a reconstructed maximal minor is nonzero. Schur-module membership and degree-90 minimality remain open.

The target claim is that the principal component of \(\operatorname{Hilb}^9(\mathbb C^8)\) is not locally Cohen-Macaulay at \(\mathfrak m^2\), hence Haiman's stated Cohen-Macaulay conjecture fails for \(d\ge 8\), \(n\ge 9\).

## Evidence policy

- `reported`: asserted by the source paper.
- `reproduced`: obtained by code reconstructed from the paper.
- `cross_checked`: obtained by an independent implementation or CAS.
- `certified`: accompanied by a small verifier and immutable certificate.
- `proved`: justified by a complete written mathematical argument.

No statement is promoted across these levels merely because the paper was published.

## First certified milestone

Independent SageMath, Magma, Singular, Oscar.jl, and Rust generators agree on
a sparse matrix with shape `90 x 115`, 1410 nonzero entries, and canonical
payload SHA-256
`dc4bb49eaab862c6f11ce83b9129fd9c16adc6b6d4e591fc4be8ea972d9176ac`.
A recorded `90 x 90` pivot minor has determinant `970351` modulo the prime
`1000003`. This proves that the selected integer determinant polynomial is
nonzero in characteristic zero. It does **not** yet prove that it is a minimal
generator of `J_8`.

## Repository map

- `PLAN.md`, `JOURNAL.md`, `MEMORY.md`, `STATUS.json`: resumable project control.
- `research-vault/`: Obsidian-compatible knowledge base.
- `book/`: Jupyter Book source.
- `artifacts/`: SageMath, Magma, Singular, Oscar.jl, and Rust implementations.
- `results/`: machine-readable certificates and execution logs.
- `tools/slurm/`: WMI SLURM synchronization and batch entrypoints.
- `sources/`: bibliographic metadata and source integrity records; the copyrighted PDF is excluded.

## Reproducibility entrypoints

These will become authoritative as implementations land:

```bash
./scripts/validate-all.sh
./scripts/validate-book.sh
```

## Licensing

Code is MIT-licensed. Original documentation in this repository is CC BY 4.0. The source article remains under its publisher's copyright and is not covered by either license.
