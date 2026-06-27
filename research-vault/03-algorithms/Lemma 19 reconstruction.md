# Lemma 19 reconstruction

## Printed construction

Set \(d=8\) and isolate the 45 symmetric coordinates
\[
A=\{p_{r,st}:1\le r\le3,\ 4\le s\le t\le8\}.
\]

Select 90 relations \(C(a;j,(i,k))\), with \(1\le a\le3\), and 30 index triples per \(a\). Every term factors as one coordinate from \(A\) times one centered complementary coordinate.

This yields
\[
C=M(A)z,
\]
where \(M\) is \(90\times115\), linear in the 45 coordinates \(A\), and \(z\) lists the complementary centered coordinates.

## Required certificates

- canonical ordered row and column manifests;
- a symbolic identity certificate for \(Mz=C\);
- a finite-field full-rank specialization and pivot minor;
- a characteristic-zero explanation of nonvanishing;
- an ideal-membership proof on the reduced principal component;
- a highest-weight or isotypic-projection certificate;
- a complete degree-89 predecessor/exclusion certificate.

Current negative evidence:

- a certified nonzero maximal minor has weight outside the claimed irreducible;
- a certified nonzero weight-compatible maximal minor is not itself a
  highest-weight vector;
- a certified nonzero `89 x 89` minor has weight outside all 15 predecessor
  modules.

See [[Matrix M specification]] and [[../06-certificates/Evidence ledger|Evidence ledger]].
