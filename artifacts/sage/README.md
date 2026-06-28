# SageMath artifacts

Planned authoritative entrypoints:

- `generate_matrix.sage`: symbolic derivation, canonical manifest, and deterministic finite-field nonvanishing certificate.
- `enumerate_predecessors.sage`: the 102-to-15 LR/dominance enumeration,
  explicit tensor LR chains, and the compact list of degree-89
  minimality-relevant \(V=S_\lambda W\) candidates. This does not construct
  the actual copies in `Sym^89(S_mu)` or test `J_8` membership.
- `compute_symmetric_multiplicities.sage`: secondary exact Phase B
  implementation route. It uses Jacobi--Trudi and
  \(\operatorname{Sym}^k(\operatorname{Sym}^2 V)=
  \bigoplus_{\lambda\vdash k} S_{2\lambda}V\) to reduce the GL5 plethysm
  coefficients to ordinary Littlewood--Richardson products. It is not the
  accepted full-run certificate yet: the current direct LR summation did not
  return the first target within 90 seconds locally. The accepted Phase B
  nonvanishing certificate is currently the Rust modular weight-DP artifact.
- `verify_certificate.sage`: certificate-only verifier.

No Sage result is considered cross-checked until the Rust verifier accepts the same certificate.
