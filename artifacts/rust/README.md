# Rust certificate verifier

This crate is the small trusted verifier. It will:

1. regenerate row, column, and variable manifests;
2. regenerate the specialized matrix directly from the printed formulas;
3. select the recorded 90 pivot columns;
4. compute its determinant modulo the recorded prime;
5. compare it with the certificate.

The verifier does not depend on Sage, Magma, Singular, Julia, or network access.
