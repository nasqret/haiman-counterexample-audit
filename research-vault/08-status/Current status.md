# Current status

- Verdict: **open audit**
- Source: identified and integrity-hashed
- Critical pages: rendered and visually checked
- Local toolchain: SageMath, Singular, Oscar/Julia, Rust available
- Remote infrastructure: WMI operational at the 2026-06-27 snapshot
- Project jobs: WMI job 105520 completed with exit 0:0; output archived and hashed
- Public repository: https://github.com/nasqret/haiman-counterexample-audit
- Live book: https://nasqret.github.io/haiman-counterexample-audit/
- Matrix reconstruction: SageMath/Magma/Singular/Oscar/Rust agree on 1410 sparse entries
- Nonvanishing: certified by a rank-90 specialization and determinant residue
- Representation audit: two literal Lemma 19 membership sentences refuted as written
- Degree-89 candidates: 15 \(V=S_\lambda W\) modules certified at
  Littlewood--Richardson/tensor level
- Weight supports: nonzero exact-torus-weight `89 x 89` minor witnesses found
  for all 15 candidates
- Symmetric multiplicities: reduced to explicit GL5 plethysm coefficients;
  coefficient values still open
- Multiplicity engines: exact Sage Jacobi--Trudi/LR and modular Rust
  weight-DP implementations exist, but both are in-progress and not accepted
  as certificates
- Current work: solve the raising-operator linear systems, construct the
  symmetric-power copies, and decide whether a
  hidden projection or linear combination still gives the required Schur
  constituent
- Main blocker: no independent certificate yet for degree-90 minimality or for absence of the required projection

Machine-readable authority: `../../STATUS.json`.
