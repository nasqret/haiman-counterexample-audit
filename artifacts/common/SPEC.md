# Canonical computational specification

All indices are one-based in serialized artifacts.

## Symmetric coordinates

Normalize the final two indices:
\[
p(r,s,t)=p(r,\min(s,t),\max(s,t)).
\]

The coefficient variables are
\[
A=\{p(r,s,t):1\le r\le3,\ 4\le s\le t\le8\}
\]
in lexicographic order \((r,s,t)\).

## Row tuples

Rows are tuples \((a,j,i,k)\) in the family order listed in `research-vault/03-algorithms/Matrix M specification.md`. Within each family use lexicographic order of the displayed tuple.

## Centered coordinates

For each serialized column \((r,s,t)\),
\[
z(r,s,t)=p(r,s,t)-\frac{\delta_{r,t}}2p(s,s,s)-\frac{\delta_{r,s}}2p(t,t,t).
\]

The column blocks and their order are defined in the same vault note.

## Relation

For every selected row,
\[
C(a;j,(i,k))
=\sum_{m=1}^{8}
\left(p(m,i,j)p(a,k,m)-p(m,k,j)p(a,i,m)\right).
\]

Substitute the inverse centering relation, collect coefficients of the 115 \(z\)-coordinates, and define these coefficients as the corresponding row of \(M\). The residual polynomial must be zero.

## Sparse serialization

A symbolic matrix entry is a signed coefficient variable, represented as:

```json
{"sign": -1, "variable": [2, 4, 7]}
```

Zero is `null`. If derivation produces a coefficient other than \(0,\pm1\) or a sum of variables, validation must fail; do not silently generalize the format.

## Modular arithmetic

Certificates use an odd prime \(p\), with \(1/2\) interpreted as the inverse of 2 modulo \(p\). Pivot columns are zero-based in JSON so they can index arrays in every implementation; the manifest explicitly records this exception.
