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

## 2026-06-27 - Public reproducibility release

- Published the source-PDF-free repository at
  `https://github.com/nasqret/haiman-counterexample-audit`.
- Enabled workflow-based GitHub Pages at
  `https://nasqret.github.io/haiman-counterexample-audit/`.
- Validation run `28277277250` passed both the clean book build and locked Rust
  tests.
- Pages run `28277277253` built and deployed successfully.
- Live browser QA confirmed navigation, formulas, styling, and the current
  verdict; no page-load failure was observed.

## 2026-06-27 - Additional representation counter-certificates

- Added `lemma19_89_minor_weight_audit.json`. It deletes row 0 and the first
  pivot column from the certified full-rank pivot minor. The resulting
  `89 x 89` determinant is `421057 mod 1000003`.
- Its weight is `(60,59,59,140,123,126,119,115)`, with dominant reordering
  `(140,126,123,119,115,60,59,59)`.
- Every one of the 15 certified degree-89 predecessor partitions has first
  part at most 132, so this nonzero `89 x 89` minor is not a weight of any of
  those modules. This refutes the printed claim that each `89 x 89` minor lies
  in one of them.
- Added `lemma19_claimed_weight_raising_audit.json`. The previously found
  weight-compatible `90 x 90` minor has determinant `300834 mod 1000003`, but
  simple raising derivatives for the Borel order `(4,5,6,7,8,1,2,3)` are
  nonzero:
  `E45=685026`, `E56=176188`, `E67=140239`, `E78=78485`, and `E23=605583`.
- Rust now independently recomputes the 89-minor determinant/weight and the
  same raising-derivative residues.

Boundary: these certificates refute literal sentences in Lemma 19, but they do
not yet prove that the required Schur constituent is absent from the span of all
maximal minors.

## 2026-06-27 - Plethysm-coefficient route sizing

- Reduced the coarse necessary representation question to the coefficient of
  `S_(43,40,36,32,29)V` in `S_(30,30,30)(Sym^2 V)`.
- A first modular 4D NTT prototype returned nonzero residues, but it was
  rejected before use: transform length `64` causes cyclic wraparound, and
  degree sums such as `beta_i + 64` can alias into the queried coefficient.
- A one-color multigraph DP with caps `(47,43,38,33,29)` is feasible but large:
  table sizes for edge counts 28 through 32 are roughly 437k, 488k, 542k, 598k,
  and 655k degree states.
- A direct semistandard-tableau chain DP for shape `(30,30,30)` is cleaner, but
  the naive Python transition loop already reaches 2.7M ending states and 68M
  transitions at column 5.

Next implementation should use an optimized Rust tableau DP with componentwise
zeta transforms over the 455 strict column types, or a high-memory nonwrapping
convolution. The invalid cyclic NTT residue must not be cited.

## 2026-06-28 - Clarified degree-89 minimality candidates

- Addressed the concern that `artifacts/sage/enumerate_predecessors.sage` is
  only a Littlewood--Richardson/tensor-power computation.
- Added `results/certificates/lemma19_minimality_relevant_modules.json`, a
  compact list of the 15 degree-89 Schur modules \(V=S_\lambda W\) for which
  \(V\otimes S_\mu W\) can contain the target \(S_\nu W\).
- Each listed candidate has `c^nu_(lambda,mu)=1` and points back to the
  88-step LR-chain certificate in
  `lemma19_predecessor_partitions.json`.
- This is now documented as a necessary candidate list for minimality testing,
  not as a construction of actual copies inside `Sym^89(S_mu)` and not as a
  `J_8`-membership or exclusion test.
- Updated the Lemma 19 report and book chapter to state that the certified
  `89 x 89` minor refutes the literal phrase "each minor", but is not itself a
  plausible nonminimality witness for the target \(S_\nu W\).

Next decisive step: construct the symmetric-power multiplicity spaces for the
15 candidates, then compute the \(S_\nu W\)-projection of the multiplication
image from degree 89 into degree 90.

## 2026-06-28 - Degree-89 attack plan

- Added a dedicated Jupyter Book chapter,
  `book/chapters/degree89_attack_plan.md`, for the questions raised about the
  LR enumeration and the missing degree-89 submodules.
- The plan separates six gates:
  weight-support enumeration, symmetric-power multiplicities, compressed
  highest-weight construction, \(J_8\)-membership testing, multiplication into
  \(S_\nu W\), and final minimality/nonminimality certification.
- Added the short Obsidian note
  `research-vault/07-audit/Degree-89 attack plan.md`.
- Updated `PLAN.md` so the next implementation tasks are explicit:
  enumerate weight-compatible supports, compute symmetric multiplicities, then
  construct/test the actual embedded \(S_\lambda W\)-copies.

The main technical design choice is to avoid expanding degree-89 polynomials.
The planned computations use determinant/cofactor spans, finite-field
evaluations, raising-operator linear systems, and rank certificates.

## 2026-06-28 - Phase A weight-support witnesses

- Added `artifacts/common/enumerate_weight_supports.py`, using binary weight
  equations via `scipy.optimize.milp` and determinant checks at the recorded
  finite-field specialization.
- Generated `results/certificates/lemma19_weight_supports.json`.
- The artifact verifies the previously known nonzero degree-90 determinant of
  Borel-oriented \(\nu\)-weight.
- For each of the 15 degree-89 candidates \(S_\lambda W\), it finds a nonzero
  `89 x 89` determinant whose torus weight is the Borel-oriented \(\lambda\).
- All 15 witnesses use deleted row `0`; singular MILP supports were excluded
  when necessary before a nonzero determinant was found.
- Added non-SciPy verification to `artifacts/common/analyze_minor_weight.py`,
  so stored supports are checked by recomputing every determinant and weight.

Boundary: these are exact torus-weight support witnesses, not highest-weight
vectors. The next gate is the raising-operator linear system on the
exact-weight spans.

## 2026-06-28 - Phase B symmetric-multiplicity reduction

- Added `artifacts/common/reduce_symmetric_multiplicities.py`.
- Generated `results/certificates/lemma19_symmetric_multiplicity_targets.json`.
- The degree-89 multiplicity question for each candidate now reduces to a GL5
  plethysm coefficient
  \([S_\alpha V:S_{(30,30,29)}(\operatorname{Sym}^2 V)]\).
- The degree-90 target similarly reduces to
  \([S_{(43,40,36,32,29)}V:S_{(30,30,30)}(\operatorname{Sym}^2 V)]\).
- Direct Sage plethysm expansion was tested and interrupted after it failed to
  return promptly for one coefficient; it should not be used as the primary
  method.

Next implementation should compute the listed coefficients by a targeted
semistandard-tableau/character DP, probably in Rust or optimized Python with
aggressive pruning.

## 2026-06-28 - Phase B multiplicity-engine attempts

- Added `artifacts/sage/compute_symmetric_multiplicities.sage`.
- The Sage route uses theorem-level reductions rather than full plethysm:
  Jacobi--Trudi for \(s_\pi(\operatorname{Sym}^2V)\) and the symmetric-matrix
  Cauchy identity
  \[
  h_k(\operatorname{Sym}^2V)=
  \operatorname{Sym}^k(\operatorname{Sym}^2V)=
  \bigoplus_{\lambda\vdash k,\ell(\lambda)\le5} S_{2\lambda}V.
  \]
- This turns each coefficient into a signed sum of ordinary
  Littlewood--Richardson products in GL5. The current direct summation is still
  too broad: the first degree-90 target did not return within 90 seconds
  locally, even using `lrcalc.mult(..., maxrows=5)`.
- Added `artifacts/rust/src/bin/plethysm_multiplicity.rs`.
- The Rust route computes weight multiplicities modulo \(10^9+7\) by
  semistandard-column DP and applies the Weyl character formula. A nonzero
  residue would rigorously prove nonzero integer multiplicity.
- The current dense-state Rust DP reached about 700k bounded weight states at
  height-3 column 11/29 for \((30,30,29)\), then was interrupted as
  too slow/high-memory.

No `lemma19_symmetric_multiplicities.json` certificate has been accepted.
Next attempt should either prune the LR summation over intermediate
partitions \(\tau\subseteq\alpha\), replace the Rust state vectors with a
sparse/zeta representation and target-specific reachability, or profile a
bounded cluster run before submitting a long job.
