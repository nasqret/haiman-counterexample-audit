# Independent implementations

The five implementations share a textual specification but not matrix-generation code.

| Environment | Version / host | Verified role |
|---|---|---|
| SageMath | 10.8, local | symbolic derivation, LR chains, exact finite-field linear algebra |
| Magma | V2.28-3, `lts-faculty` | independent matrix generation and determinant |
| Singular | 4.4.1, local | independent translation cancellation and determinant |
| Oscar.jl | 1.7.3 / Julia 1.12.5, local | independent exact finite-field matrix and determinant |
| Rust | 1.96.0, local | small matrix, determinant, weight, and LR verifier |

Agreement among systems catches transcription and API errors. It is not, by itself, a proof: every certificate is accompanied by the mathematical reason it implies the claimed statement.

All five implementations regenerate the matrix from the printed index
formulas. They agree on 1410 nonzero entries, zero translation residual, rank
90, and determinant 970351 modulo 1000003. This closes nonvanishing, but not
the Schur-module or minimality claims.

Remote computations run through reviewed SLURM scripts or the faculty host as
appropriate. Job IDs, revisions, resources, and output hashes are recorded
before a result is cited.
