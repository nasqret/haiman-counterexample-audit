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

Current certificates separate three facts that the paper states together:

- `lemma19_pivot_weight_audit.json`: the certified nonzero pivot minor is weight-homogeneous, but its dominant weight is incompatible with the claimed irreducible.
- `lemma19_claimed_weight_raising_audit.json`: a different nonzero maximal minor has the claimed torus weight, but nonzero raising derivatives show it is not itself a highest-weight vector.
- `lemma19_89_minor_weight_audit.json`: a certified nonzero $89\times89$ minor is not a weight of any of the 15 certified degree-89 predecessor modules.
- `lemma19_minimality_relevant_modules.json`: the 15 Schur-highest weights $\lambda$ for which $S_\lambda W\otimes S_\mu W$ can contain the target $S_\nu W$, with pointers to the LR-chain certificate.
- `lemma19_weight_supports.json`: nonzero exact-torus-weight minor witnesses for the Borel-oriented degree-90 target and for all 15 Borel-oriented degree-89 candidates.
- `lemma19_symmetric_multiplicity_targets.json`: the reduced GL5 plethysm coefficients whose values would give the symmetric-power multiplicities.

Thus the nonvanishing computation is real, but the printed representation membership statements cannot be accepted literally.

## Minimality certificate

The fifteen candidate degree-89 partitions and all relevant multiplicity-space maps must be serialized explicitly. The phrase "none of their embeddings" is not a certificate until the basis of every embedding space and the membership test are specified.

The partition enumeration itself is now certified. There are 102 partitions with nonzero LR coefficient into the target degree-90 partition. Tensor-product dominance removes 87, leaving exactly 15. Each survivor comes with an 88-step nonzero LR chain proving occurrence in the 89-fold tensor power. SageMath and an independently implemented Rust LR-tableau rule agree.

This answers the first part of the degree-89 question: these 15 are exactly the currently certified candidates for a Schur module $V=S_\lambda W$ such that $V\otimes S_\mu W$ can contribute to $S_\nu W$. It does **not** answer the harder question whether the corresponding copies occur inside the symmetric power in a way that lies in `J_8`. The separate symmetric-power embedding and `J_8` exclusion remain open.

The weight-support artifact closes a still narrower gate: the candidate torus weights are not vacuous in the reconstructed minor/cofactor span. It does not certify highest-weight annihilation. The next certificate must solve the raising-operator linear systems on these exact-weight spans.

The symmetric-multiplicity target artifact reduces the next representation problem to coefficients of \(S_\alpha V\) in \(S_\pi(\operatorname{Sym}^2 V)\) for \(\dim V=5\). It does not compute those coefficients.
