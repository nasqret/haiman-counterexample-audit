# Rust certificate verifier

This crate is the small trusted verifier. It will:

1. regenerate row, column, and variable manifests;
2. regenerate the specialized matrix directly from the printed formulas;
3. select the recorded 90 pivot columns;
4. compute its determinant modulo the recorded prime;
5. compare it with the certificate.

The verifier does not depend on Sage, Magma, Singular, Julia, or network access.

Additional experimental binary:

- `plethysm_multiplicity`: in-progress modular Phase B implementation for the
  GL5 coefficients listed in
  `results/certificates/lemma19_symmetric_multiplicity_targets.json`. It uses
  semistandard-column dynamic programming plus the Weyl character formula.
  Nonzero residues would rigorously certify nonzero integer multiplicities,
  but the current dense-state version is not in default validation: on the
  local machine it reached about 700k bounded weight states at column 11 of 29
  for \((30,30,29)\) and was interrupted as too slow/high-memory.
