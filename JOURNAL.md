# Research journal

## 2026-06-27 - Intake, source inspection, and audit decomposition

### Observations

- The supplied file is the 17-page published version of Lee's 2010 Journal of Algebra article.
- PDF pages 12-13 are journal pages 1358-1359 and contain Lemma 19.
- The lemma is explicitly labeled "Sketch of proof" and contains the missing computational core.
- The selected rows of the quadratic relations number 90; the complementary coordinate vector has 115 entries. These counts can be derived exactly from the printed index sets.
- The claimed degree-90 Schur module has partition `(133,130,126,122,119,60,60,60)`, of total size 810, exactly `90 * 9` for the degree-9 coordinate representation.

### Initial proof-obligation graph

1. Reconstruct the coordinate change from \(p\) to the 280-dimensional \(q\)-space.
2. Reconstruct the selected 90 relations and the 115 complementary coordinates.
3. Generate \(M\) algorithmically and compare \(Mz\) to the printed relations.
4. Find and certify a nonzero maximal minor without expanding the degree-90 determinant.
5. Prove that all maximal minors vanish on the reduced principal component.
6. Certify the highest weight / Schur-module assertion.
7. Enumerate and verify the 15 predecessor partitions.
8. Replace the undocumented "none of their embeddings are in \(J_8\)" check with explicit evaluation certificates or another complete argument.
9. Audit Lemma 5, Proposition 6, and the regularity implication separately.

### Infrastructure

- WMI VPN, gateway, controllers, queue, and nodes were checked read-only and were operational.
- No project job has been submitted.
- The source PDF was rendered and visually inspected at the critical pages.
- The PDF is excluded from version control pending redistribution permission.

### Open risks

- The paper does not list the 15 partitions, a selected nonzero minor, a random seed, or any software/version details.
- Proposition 6 also cites a Macaulay2 check for the codimension statement when \(2\le d\le8\); this is a second computational dependency.
- The representation-theoretic degree/weight bookkeeping needs exact normalization.

## 2026-06-27 - Exact matrix reconstruction and nonvanishing certificate

### Construction

- Implemented the p-to-q centering formula from Theorem 16, treating `q_(s,ss)` as the translation summand.
- Derived all matrix entries by collecting the selected quadratic relations; no entry was transcribed.
- All translation terms canceled exactly.
- The resulting sparse matrix has shape `90 x 115` and 1410 nonzero signed-variable entries.

### Certificate

- SageMath specialization used prime `1000003` and seed `20100701`.
- Trial 0 had rank 90.
- The recorded pivot minor has determinant `970351` modulo the prime.
- Rust independently regenerated the matrix, matched payload SHA-256 `dc4bb49eaab862c6f11ce83b9129fd9c16adc6b6d4e591fc4be8ea972d9176ac`, and recomputed the same determinant.

### Mathematical consequence

The selected determinant is a polynomial with integer coefficients. Since one specialization remains nonzero modulo a prime, it is not the zero polynomial over the integers or over characteristic zero.

### Boundary

This closes only the paper's assertion that some maximal minor is nonzero. It does not close the claims that the minor lies in the stated Schur module or is a minimal generator of `J_8`.

## 2026-06-27 - Reconstruction of the fifteen degree-89 predecessors

- Corrected the total polynomial weight to 810 = 90 times 9.
- Enumerated 102 partitions with nonzero `c^nu_(lambda,mu)`.
- Applied the tensor-product dominance bound `lambda <= 89 mu`; exactly 15 survive.
- Constructed, for each survivor, an 88-step LR chain from `mu` to `lambda`, proving actual tensor occurrence rather than assuming dominance is sufficient.
- Implemented an independent Rust LR rule using semistandard skew tableaux and the lattice-word condition.
- Rust reproduced all 102 LR candidates, all 15 tensor candidates, and verified every recorded chain.

The remaining minimality claim is sharper: for every relevant embedding into `Sym^89(S_mu)`, the corresponding copy must be shown not to lie in `J_8`.

## 2026-06-27 - Weight audit of the maximal-minor claim

- Derived the torus weight `1 - e_r + e_s + e_t` of each coordinate `p_(r,st)` from the `wedge^7 W tensor Sym^2 W` realization.
- Verified the row/column additive weight factorization of all 1410 matrix entries.
- The certified nonzero pivot minor is weight-homogeneous of weight `(60,60,60,141,125,128,120,116)`.
- Its dominant reordering starts with 141, exceeding the claimed highest weight's first part 133; it cannot lie in `S_(133,130,126,122,119,60,60,60)`.
- This refutes the word **any** in the printed assertion about maximal minors.
- A separate mixed-integer search found a full-rank minor of Weyl-orbit weight `(60,60,60,133,130,126,122,119)`, with determinant `300834 mod 1000003`. This is a necessary weight match, not yet a highest-weight or isotypic-membership proof.

### Ideal membership closed conceptually

On the principal component, `C=Mz=0`. The standard radical simplex `{0,e_1,...,e_8}` has nonzero centered coordinates (for example mixed centered coordinates equal `-1/2`), so `z != 0` on a nonempty open subset. Thus `rank(M)<90` there; all maximal minors vanish densely and lie in the reduced defining ideal `J_8`.

## 2026-06-27 - Five-language reconstruction checkpoint

- Added independent generators/verifiers in Singular, Oscar.jl, and Magma;
  none imports SageMath's matrix entries.
- Singular 4.4.1 and Oscar 1.7.3 passed locally. Oscar's pinned environment
  records Julia 1.12.5, GAP.jl 0.16.7, and Singular.jl 0.28.9.
- Magma V2.28-3 passed on `lts-faculty.wmi.amu.edu.pl`.
- SageMath, Magma, Singular, Oscar.jl, and Rust all agree on shape `90 x 115`,
  1410 nonzero entries, zero translation residual, rank 90, and determinant
  `970351 mod 1000003`.
- A portable standard-library verifier reproduces the canonical matrix payload
  hash and is prepared for one isolated WMI CPU batch cross-check.

This agreement closes the reconstruction/nonvanishing engineering question.
It does not close highest-weight membership or degree-89 exclusion.

## 2026-06-27 - WMI batch cross-check

- Submitted reviewed CPU job `105520` from commit
  `caed815f21f360fcd610dca31c25ca08cf167106` after confirming that no
  equivalent project job existed.
- Resource request: one CPU, 1 GB RAM, five-minute limit on partition `cpu`.
- Result: `COMPLETED`, exit code `0:0`, elapsed below one second, batch MaxRSS
  312 KB on `c2n2.cluster.wmi.amu.edu.pl`.
- Python 3.10.12 independently regenerated the canonical payload hash and
  determinant 970351 modulo 1000003.
- Output SHA-256:
  `8bffbfedf3346525d5b13c1ed13e64a6b02e4c3d72bca52ce3fd0bc6d3abf763`.
