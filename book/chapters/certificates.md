# Certificate design

The project uses canonical JSON so that five implementations can verify the same mathematical object.

## Matrix manifest

The manifest records:

- the ordered 90 row tuples $(a,j,i,k)$;
- the ordered 115 column tuples $(r,s,t)$;
- the ordered 45 coefficient variables;
- the centering convention;
- a SHA-256 digest of all entries in sparse form.

## Nonvanishing certificate

A certificate records:

- prime $p$;
- deterministic seed and generator algorithm;
- 45 specialized values;
- 90 pivot column indices;
- determinant residue modulo $p$;
- optional row-operation transcript digest.

A verifier regenerates $M$, selects the pivot minor, and recomputes the determinant. No expanded degree-90 polynomial is needed.

The current certificate uses prime `1000003`, seed `20100701`, the first generated assignment, and determinant residue `970351`. Its matrix payload SHA-256 is `dc4bb49eaab862c6f11ce83b9129fd9c16adc6b6d4e591fc4be8ea972d9176ac`. SageMath generates it and an independent Rust verifier accepts it.

## Representation certificate

For a selected determinant $f$, a highest-weight certificate must check both:

1. its diagonal torus weight is the claimed partition;
2. each simple raising operator $E_{i,i+1}$ annihilates $f$.

These operations should be implemented on the determinant representation or by polynomial identity testing with rigorous bounds, avoiding expansion whenever possible.

## Minimality certificate

The fifteen candidate degree-89 partitions and all relevant multiplicity-space maps must be serialized explicitly. The phrase "none of their embeddings" is not a certificate until the basis of every embedding space and the membership test are specified.

The partition enumeration itself is now certified. There are 102 partitions with nonzero LR coefficient into the target degree-90 partition. Tensor-product dominance removes 87, leaving exactly 15. Each survivor comes with an 88-step nonzero LR chain proving occurrence in the 89-fold tensor power. SageMath and an independently implemented Rust LR-tableau rule agree. The separate symmetric-power embedding and `J_8` exclusion remain open.
