# Plan and acceptance contract

Status values: `[ ]` pending, `[~]` in progress, `[x]` verified, `[!]` blocked or failed.

## Phase 0 - Project control

- [x] Identify and hash the supplied source.
- [x] Create memory, journal, plan, status file, Obsidian vault, and Jupyter Book scaffold.
- [x] Initialize local git and record reproducible checkpoints.
- [x] Create a public GitHub repository without redistributing the source PDF.
- [x] Enable and visually verify GitHub Pages from the validated book build.

## Phase 1 - Mathematical audit

- [~] Write a dependency graph from Haiman's conjecture to Lee's Theorem A.
- [ ] Verify all dimension and regularity conventions.
- [ ] Audit Lemma 5 as a standalone theorem.
- [ ] Audit Proposition 6, including the cited Macaulay2 codimension computation.
- [ ] Check every propagation step in Corollary B.

## Phase 2 - Exact reconstruction of Lemma 19

- [ ] Specify all 280 centered coordinates canonically.
- [x] Enumerate the 90 printed row indices exactly.
- [x] Enumerate the 115 printed column indices exactly.
- [x] Derive every entry of \(M\) from \(C(a;j,(i,k))\), not from hand transcription.
- [x] Symbolically verify \(Mz=C\) over \(\mathbb Z\) for the selected relations, including cancellation of translations.
- [x] Produce stable row/column manifests and hashes.

## Phase 3 - Degree-90 nonvanishing certificate

- [x] Find a full-rank specialization of \(M\) modulo a recorded prime.
- [x] Store the 45 input values and 90 pivot columns.
- [x] Store the determinant residue and independent matrix payload hash.
- [x] Verify the same certificate in SageMath, Magma, Singular, Oscar.jl, and Rust.
- [x] Explain why the modular specialization proves characteristic-zero nonvanishing.

## Phase 4 - Ideal and representation membership

- [x] Prove maximal minors vanish on a dense subset of \(P_8\), hence lie in reduced \(J_8\).
- [ ] Verify the rational parametrization used for membership tests.
- [x] Certify weights of selected maximal and `89 x 89` minors.
- [x] Apply/check simple raising operators for the selected weight-compatible determinant.
- [ ] Establish membership in \(S_{(133,130,126,122,119,60,60,60)}W\) in characteristic zero.

## Phase 5 - Minimality

- [x] Enumerate the claimed 15 partitions with reproducible LR code and independent Rust verification.
- [x] Serialize the 15 tensor-level \(V=S_\lambda W\) candidates whose product with \(S_\mu W\) can contain \(S_\nu W\).
- [x] Write the staged attack plan for the degree-89 symmetric-power and \(J_8\)-membership questions.
- [x] Produce nonzero exact-torus-weight `89 x 89` and `90 x 90` minor/cofactor support witnesses.
- [x] Reduce the degree-89 symmetric-power multiplicities to explicit GL5 plethysm coefficients.
- [x] Compute symmetric-power multiplicity nonvanishing for the 15 \(V=S_\lambda W\) candidates by modular Rust weight DP.
- [x] Record all relevant tensor-level and symmetric-power nonvanishing multiplicity certificates.
- [~] Keep exact Sage LR reduction as a secondary implementation; it is not yet optimized enough for the full target.
- [ ] Solve raising-operator linear systems on the exact-weight minor/cofactor spans.
- [ ] Construct every relevant symmetric-power embedding in degree 89.
- [ ] Prove/evaluate that no relevant copy lies in \(J_8\).
- [ ] Compute the multiplication image into the \(S_\nu W\)-isotypic component.
- [ ] Conclude that a degree-90 generator is minimal, or locate the failure.

## Phase 6 - Cross-CAS artifacts

- [x] SageMath implementation and tests.
- [x] Magma implementation and tests on `lts-faculty`.
- [x] Singular implementation and tests.
- [x] Oscar.jl implementation and tests.
- [x] Rust generator/verifier and tests.
- [x] Cross-language certificate agreement and execution ledger.

## Phase 7 - Compute and preserve evidence

- [x] Run fast local certificates.
- [x] Benchmark the portable verifier before sizing the SLURM job.
- [x] Submit one reviewed, isolated WMI CPU cross-check.
- [x] Record job ID, commit, inputs, resources, and output immediately.
- [x] Fetch and hash remote result for job 105520.

## Phase 8 - Documentation and verdict

- [x] Execute every notebook from a clean environment.
- [x] Build Jupyter Book HTML with warnings as errors.
- [x] Build PDF and visually inspect rendered pages.
- [ ] Publish the evidence ledger and full proof/counter-proof.
- [ ] Issue exactly one verdict: verified counterexample, refuted construction, or conditional/unresolved with a minimal remaining obligation.

## Completion criteria

The project is not complete merely when a CAS prints `true`. Completion requires deterministic source, immutable inputs, independent verifiers, a written argument connecting certificates to the theorem, successful book builds, and an explicit accounting of every unverified assertion.
