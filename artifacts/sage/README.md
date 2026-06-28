# SageMath artifacts

Planned authoritative entrypoints:

- `generate_matrix.sage`: symbolic derivation, canonical manifest, and deterministic finite-field nonvanishing certificate.
- `enumerate_predecessors.sage`: the 102-to-15 LR/dominance enumeration,
  explicit tensor LR chains, and the compact list of degree-89
  minimality-relevant \(V=S_\lambda W\) candidates. This does not construct
  the actual copies in `Sym^89(S_mu)` or test `J_8` membership.
- `verify_certificate.sage`: certificate-only verifier.

No Sage result is considered cross-checked until the Rust verifier accepts the same certificate.
