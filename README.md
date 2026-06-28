# Haiman conjecture counterexample audit

[Documentation](https://nasqret.github.io/haiman-counterexample-audit/) ·
[GitHub repository](https://github.com/nasqret/haiman-counterexample-audit)

This repository reconstructs and independently audits the counterexample claimed in Kyungyong Lee, *The singularities of the principal component of the Hilbert scheme of points*, Journal of Algebra 324 (2010), 1347-1363, DOI: 10.1016/j.jalgebra.2010.07.001.

> **Current verdict: open audit.** The paper's conclusion has not yet been independently certified here. The reconstructed maximal minor is nonzero, but two literal representation-theoretic sentences in Lemma 19 are now refuted. The possible rescue route is a different linear combination or isotypic projection, which remains open.

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

The representation audit has also produced two negative certificates:

- the certified nonzero pivot minor has dominant weight beginning with `141`, so
  it cannot lie in the claimed irreducible whose highest weight begins with
  `133`;
- deleting one row and one column gives a certified nonzero `89 x 89` minor
  with dominant weight beginning with `140`, while all 15 degree-89 predecessor
  candidates begin at most with `132`.

A separate nonzero `90 x 90` minor has the claimed torus weight, but it is not
itself a highest-weight vector: five tested simple raising derivatives are
nonzero modulo `1000003`.

The 15 degree-89 candidates are now also serialized as
`results/certificates/lemma19_minimality_relevant_modules.json`. This file
answers the narrow question “which Schur modules \(V=S_\lambda W\) could
contribute to the target through \(V\otimes S_\mu W\)?” at the
Littlewood-Richardson/tensor level. It does not construct the actual embedded
copies inside `Sym^89(S_mu)` and does not test their `J_8` membership; those
are still the decisive open minimality checks.

The next support-level gate is now also recorded:
`results/certificates/lemma19_weight_supports.json` contains nonzero
exact-torus-weight `89 x 89` minor witnesses for all 15 candidates. These are
support witnesses only; highest-weight annihilation and `J_8` membership remain
open.

## Repository map

- `PLAN.md`, `JOURNAL.md`, `MEMORY.md`, `STATUS.json`: resumable project control.
- `research-vault/`: Obsidian-compatible knowledge base.
- `book/`: Jupyter Book source.
- `artifacts/`: SageMath, Magma, Singular, Oscar.jl, and Rust implementations.
- `results/`: machine-readable certificates and execution logs.
- `tools/slurm/`: WMI SLURM synchronization and batch entrypoints.
- `sources/`: bibliographic metadata and source integrity records; the copyrighted PDF is excluded.

## Reproducibility entrypoints

Authoritative local entrypoints:

```bash
./scripts/validate-all.sh
./scripts/validate-book.sh
```

## Licensing

Code is MIT-licensed. Original documentation in this repository is CC BY 4.0. The source article remains under its publisher's copyright and is not covered by either license.
