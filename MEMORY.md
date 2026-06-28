# Project memory

Last updated: 2026-06-28 (Europe/Warsaw)

## Objective

Decide, with independently checkable evidence, whether Lee's 2010 construction proves a counterexample to Haiman's Cohen-Macaulay conjecture. Prefer compact certificates over trust in a particular computer algebra system.

## Source of truth

- Local file: `Lee_principal_component_not_CM.pdf` (never publish without permission).
- SHA-256: `78bb80a2ba5194e30df5fb775d9997d479e258efe986199693035babf2d75c6d`.
- Title: *The singularities of the principal component of the Hilbert scheme of points*.
- Author: Kyungyong Lee.
- Journal: Journal of Algebra 324 (2010), 1347-1363.
- DOI: `10.1016/j.jalgebra.2010.07.001`.

## Exact target

Theorem A claims that the principal component \(P_8\subset\operatorname{Hilb}^9(\mathbb C^8)\) is not locally Cohen-Macaulay at \(\mathfrak m^2\). Corollary B propagates this to \(d\ge8,n\ge9\), including the corresponding isospectral Hilbert scheme.

## Critical dependency

Theorem A depends on a degree-90 minimal generator of \(J_8\). Lemma 19 gives only a sketch and delegates the decisive assertions to "exhaustive computations" without code:

1. construct a \(90\times115\) matrix \(M\) of linear forms;
2. show at least one \(90\times90\) minor is nonzero;
3. show maximal minors lie in \(J_8\) and in the Schur module \(S_{(133,130,126,122,119,60,60,60)}W\);
4. enumerate 15 degree-89 predecessor partitions;
5. show none of their embeddings in \(\operatorname{Sym}^{89}(S_{(3,1,1,1,1,1,1,0)}W)\) lies in \(J_8\).

These are separate proof obligations. A rank computation alone does not establish minimality.

## Certified milestone: nonzero maximal minor

On 2026-06-27, SageMath reconstructed the matrix directly from the printed relation and centering formula. Rust independently regenerated the same 1410 sparse entries and verified the certificate.

- Matrix shape: `90 x 115`.
- Coefficient variables: 45.
- Nonzero entries: 1410.
- Canonical payload SHA-256: `dc4bb49eaab862c6f11ce83b9129fd9c16adc6b6d4e591fc4be8ea972d9176ac`.
- Prime: `1000003`.
- Seed: `20100701`, successful trial: 0.
- Selected pivot columns: stored in `results/certificates/lemma19_nonzero_minor.json`.
- Determinant residue: `970351`.

Logical status: nonvanishing in characteristic zero is certified. Ideal membership
is proved by the dense-kernel argument below. Literal Schur-module membership
sentences in Lemma 19 are refuted as written, but the possible existence of a
hidden projection or linear combination is still pending.

## Certified milestone: the fifteen predecessor partitions

The degree-89 predecessor enumeration is now reproducible.

- Exactly 102 partitions `lambda` satisfy `c^nu_(lambda,mu) > 0`.
- The necessary tensor-product dominance inequalities exclude 87 of them.
- Exactly 15 remain, matching Lee's unexplained count.
- For every remaining partition, an explicit chain of 88 nonzero LR coefficients from `mu` to `lambda` certifies occurrence in `S_mu^(tensor 89)`.
- SageMath certificate: `results/certificates/lemma19_predecessor_partitions.json`.
- Minimality-candidate summary:
  `results/certificates/lemma19_minimality_relevant_modules.json`.
- Independent Rust LR-tableau verifier agrees on counts 102 and 15.
- Payload SHA-256: `57e87fae955efedd5211eab5fed48e154a2df1266b9414670a4f4def689e3c40`.
- Minimality-candidate payload SHA-256:
  `5b637f623bd05243e5f890eb0dc4324dcde3b0330ddd34aef19a0f82e7dbdbe2`.

These 15 partitions are exactly the currently certified tensor-level
candidates for a degree-89 Schur module `V=S_lambda(W)` such that
`V tensor S_mu(W)` can contain the target `S_nu(W)`. This does not establish
that the corresponding copies are constructed inside `Sym^89(S_mu)`, nor that
they avoid or lie in `J_8`; that is a separate, still-open membership
computation.

Phase A support checkpoint: `results/certificates/lemma19_weight_supports.json`
contains nonzero exact-torus-weight `89 x 89` minor witnesses for all 15
candidate `lambda`s, plus the Borel-oriented degree-90 target witness. The
payload SHA-256 is
`f09815d86f342add8245a5a00dd9b59b207a9fcc9e8426b63978e0463091ed92`.
These witnesses prove the relevant torus weights occur in the reconstructed
minor/cofactor span; they are not highest-weight or `J_8`-membership
certificates.

Phase B reduction checkpoint:
`results/certificates/lemma19_symmetric_multiplicity_targets.json` rewrites the
15 degree-89 symmetric-power multiplicities as coefficients
`[S_alpha(V):S_(30,30,29)(Sym^2 V)]` for `dim(V)=5`. The payload SHA-256 is
`9dbfbd5f8b66b380352437e1d9df62f6bc706031efc3cd9e31a20b7cced0e579`. The
coefficients are not computed yet; direct Sage plethysm expansion was too slow
for use as the primary route.

## Weight audit and ideal membership

The paper's literal statement that the determinant of any maximal minor lies in `S_nu` is false for the reconstructed matrix. The certified nonzero pivot minor has torus weight

`(60,60,60,141,125,128,120,116)`,

whose dominant reordering `(141,128,125,120,116,60,60,60)` is not dominated by `nu=(133,130,126,122,119,60,60,60)`. It therefore cannot be a weight of `S_nu`. Python and Rust independently verify the additive weight factorization.

This does not yet refute the existence of the required `S_nu` constituent. A different nonzero minor with Weyl-orbit weight `(60,60,60,133,130,126,122,119)` has been certified (determinant `300834 mod 1000003`), but weight compatibility alone is not pure Schur-module membership.

That weight-compatible minor is not itself a highest-weight vector for the
Borel order `(4,5,6,7,8,1,2,3)`: the raising derivatives have nonzero residues
`E45=685026`, `E56=176188`, `E67=140239`, `E78=78485`, and `E23=605583`
modulo `1000003`.

The literal `89 x 89` minor sentence is also false as written. Deleting row 0
and the first pivot column from the certified full-rank pivot minor gives a
nonzero `89 x 89` determinant `421057 mod 1000003` with dominant weight
`(140,126,123,119,115,60,59,59)`. Every one of the 15 certified degree-89
predecessor partitions has first part at most 132, so this minor is not a
weight of any of those modules.

Every maximal minor does lie in `J_8`: on `P_8`, the selected equations give `Mz=0`; the vector `z` is nonzero on a nonempty open subset (witnessed by the radical standard simplex), hence every maximal minor vanishes on a dense subset of the reduced irreducible component and therefore belongs to its defining ideal.

## Environment snapshot

- Local: SageMath 10.8, Singular 4.4.1, Julia 1.12.5, Oscar 1.7.3,
  Rust 1.96.0, Jupyter Book, Pandoc, LaTeX, and GitHub CLI available.
- Local Magma: unavailable.
- WMI gateway/SLURM: operational on 2026-06-27; VPN route used `utun19`.
- Magma V2.28-3 on `lts-faculty.wmi.amu.edu.pl` independently reproduced
  shape, support, translation cancellation, rank, and determinant.
- WMI SLURM job `105520` completed on `c2n2` with exit `0:0`, independently
  matching the canonical payload hash and determinant from commit `caed815`.

## Cross-CAS checkpoint

SageMath, Rust, Singular, Oscar.jl, and Magma independently regenerate the
specialized matrix from Lee's index formulas. All five report 1410 nonzero
entries, zero translation residual, rank 90, and determinant 970351 modulo
1000003. The immutable execution ledger is
`results/executions/2026-06-27-cross-cas.json`.

## Conventions

- Work over \(\mathbb Z\) or a declared finite field whenever possible.
- Every randomized search records seed, prime, assignments, pivot columns, software version, commit, and certificate hash.
- A nonzero specialization modulo a prime certifies that the corresponding integer polynomial is nonzero in characteristic zero.
- Finite-field evidence for ideal membership or representation membership is not silently promoted to characteristic-zero proof.
- Never alter or cancel unrelated WMI jobs.

## Public endpoints

- Repository: `https://github.com/nasqret/haiman-counterexample-audit`
- Documentation: `https://nasqret.github.io/haiman-counterexample-audit/`
- First successful validation run: `28277277250`.
- First successful Pages run: `28277277253`.
