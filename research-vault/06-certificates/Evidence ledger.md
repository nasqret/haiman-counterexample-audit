# Evidence ledger

| Obligation | Level | Artifact | Result |
|---|---|---|---|
| Source identity and hash | certified | `sources/source-manifest.json` | matched |
| 90 row count | reproduced | `book/notebooks/matrix_counts.ipynb` | 90 |
| 115 column count | reproduced | `book/notebooks/matrix_counts.ipynb` | 115 |
| \(Mz=C\) | cross-checked | SageMath, Magma, Singular, Oscar.jl, Rust | translations cancel; 1410 entries |
| Nonzero maximal minor | certified | `lemma19_nonzero_minor.json` + five implementations | det = 970351 mod 1000003 |
| Maximal minors in \(J_8\) | proved | dense-kernel argument + standard simplex witness | yes |
| Literal "any minor in claimed Schur module" | refuted | `lemma19_pivot_weight_audit.json` + Rust | incompatible nonzero weight |
| Required Schur constituent exists in minors | pending | highest-weight/isotypic certificate | weight-compatible minor only |
| Fifteen predecessor partitions | certified | Sage LR chains + Rust tableau verifier | 102 LR; 15 tensor |
| Degree-89 exclusion | pending | rational-evaluation certificates | - |
| Proposition 6 codimension | pending | independent CAS computation | - |
| Theorem A | pending | final audit | - |

A failed or contradictory artifact is retained and labeled; it is not overwritten.

Environment versions, script hashes, and the remote Magma run are recorded in
`results/executions/2026-06-27-cross-cas.json`.
