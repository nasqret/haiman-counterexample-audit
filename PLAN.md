# Plan and acceptance contract

Status values: `[ ]` pending, `[~]` in progress, `[x]` verified, `[!]` blocked or failed.

## Phase 0 - Project control

- [x] Identify and hash the supplied source.
- [x] Create memory, journal, plan, status file, Obsidian vault, and Jupyter Book scaffold.
- [x] Initialize local git and record the first checkpoint.
- [ ] Create a public GitHub repository without redistributing the source PDF.
- [ ] Enable GitHub Pages from the validated book build.

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
- [ ] Certify the weight of a selected determinant.
- [ ] Apply/check all highest-weight raising operators.
- [ ] Establish membership in \(S_{(133,130,126,122,119,60,60,60)}W\) in characteristic zero.

## Phase 5 - Minimality

- [x] Enumerate the claimed 15 partitions with reproducible LR code and independent Rust verification.
- [ ] Record all relevant multiplicities in tensor and symmetric powers.
- [ ] Construct every relevant embedding in degree 89.
- [ ] Prove/evaluate that no relevant copy lies in \(J_8\).
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
- [ ] Submit reviewed checkpointed CPU jobs only if local computation is insufficient.
- [ ] Record job IDs, commits, inputs, resources, and outputs immediately.
- [ ] Fetch and hash remote results.

## Phase 8 - Documentation and verdict

- [x] Execute every notebook from a clean environment.
- [x] Build Jupyter Book HTML with warnings as errors.
- [x] Build PDF and visually inspect rendered pages.
- [ ] Publish the evidence ledger and full proof/counter-proof.
- [ ] Issue exactly one verdict: verified counterexample, refuted construction, or conditional/unresolved with a minimal remaining obligation.

## Completion criteria

The project is not complete merely when a CAS prints `true`. Completion requires deterministic source, immutable inputs, independent verifiers, a written argument connecting certificates to the theorem, successful book builds, and an explicit accounting of every unverified assertion.
