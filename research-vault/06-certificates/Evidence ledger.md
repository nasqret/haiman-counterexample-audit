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
| Weight-compatible maximal minor is highest | refuted for selected minor | `lemma19_claimed_weight_raising_audit.json` + Rust | nonzero raising derivatives |
| Required Schur constituent exists in minors | pending | highest-weight/isotypic certificate | projection or linear combination still open |
| Fifteen predecessor partitions | certified | Sage LR chains + Rust tableau verifier | 102 LR; 15 tensor |
| Degree-89 \(V=S_\lambda W\) candidates for \(V\otimes S_\mu W\to S_\nu W\) | certified at tensor level | `lemma19_minimality_relevant_modules.json` | 15 candidates, all LR coefficient 1 |
| Exact torus-weight supports for the 15 degree-89 candidates | certified | `lemma19_weight_supports.json` | nonzero minor witness for every candidate |
| Symmetric-power multiplicity targets | reduced | `lemma19_symmetric_multiplicity_targets.json` | GL5 plethysm coefficients listed; values open |
| Symmetric-power multiplicity engines | in progress | Sage Jacobi--Trudi/LR script + Rust modular weight DP | not accepted; local runs too slow so far |
| Literal "each 89 x 89 minor in a predecessor module" | refuted | `lemma19_89_minor_weight_audit.json` + Rust | incompatible nonzero weight |
| Copies of those candidates inside \(\operatorname{Sym}^{89}(S_\mu W)\) | pending | plethysm/symmetrization certificate | not constructed |
| Degree-89 embedding exclusion | pending | rational-evaluation certificates | - |
| Proposition 6 codimension | pending | independent CAS computation | - |
| Theorem A | pending | final audit | - |

A failed or contradictory artifact is retained and labeled; it is not overwritten.

Environment versions, script hashes, and the remote Magma run are recorded in
`results/executions/2026-06-27-cross-cas.json`.

WMI SLURM job `105520` independently reproduced the canonical matrix payload
and determinant on compute node `c2n2`; its output and resource record are in
`results/cluster/`.
