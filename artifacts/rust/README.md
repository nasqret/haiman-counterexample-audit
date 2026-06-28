# Rust certificate verifier

This crate is the small trusted verifier. It will:

1. regenerate row, column, and variable manifests;
2. regenerate the specialized matrix directly from the printed formulas;
3. select the recorded 90 pivot columns;
4. compute its determinant modulo the recorded prime;
5. compare it with the certificate.

The verifier does not depend on Sage, Magma, Singular, Julia, or network access.

Additional Phase B binary:

- `plethysm_multiplicity`: modular Phase B implementation for the
  GL5 coefficients listed in
  `results/certificates/lemma19_symmetric_multiplicity_targets.json`. It uses
  semistandard-column dynamic programming plus the Weyl character formula.
  The dense-by-weight implementation, after switching residues from `u64` to
  `u32`, produced
  `results/certificates/lemma19_symmetric_multiplicities.json` locally in about
  389 seconds. All target residues are nonzero modulo `1000000007`, which
  certifies nonzero integer multiplicities. This long computation is not part
  of default validation; validation checks the stored JSON and the small
  built-in plethysm regression tests.
