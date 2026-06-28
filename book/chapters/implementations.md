# Independent implementations

The five implementations share a textual specification but not matrix-generation code.

| Environment | Version / host | Verified role |
|---|---|---|
| SageMath | 10.8, local | symbolic derivation, LR chains, exact finite-field linear algebra |
| Magma | V2.28-3, `lts-faculty` | independent matrix generation and determinant |
| Singular | 4.4.1, local | independent translation cancellation and determinant |
| Oscar.jl | 1.7.3 / Julia 1.12.5, local | independent exact finite-field matrix and determinant |
| Rust | 1.96.0, local | small matrix, determinant, weight, raising-operator, and LR verifier |

Agreement among systems catches transcription and API errors. It is not, by itself, a proof: every certificate is accompanied by the mathematical reason it implies the claimed statement.

All five implementations regenerate the matrix from the printed index
formulas. They agree on 1410 nonzero entries, zero translation residual, rank
90, and determinant 970351 modulo 1000003. The Rust verifier also recomputes
the new representation counter-certificates: the nonzero $89\times89$ minor
outside all 15 predecessor weights and the nonzero raising derivatives of the
selected weight-compatible maximal minor. The Sage/Rust LR code also certifies
the 15 tensor-level \(V=S_\lambda W\) candidates relevant to degree-90
minimality. This closes nonvanishing and the candidate enumeration, but not the
existence or absence of the required symmetric-power Schur-module projection.

Remote computations run through reviewed SLURM scripts or the faculty host as
appropriate. Job IDs, revisions, resources, and output hashes are recorded
before a result is cited.
