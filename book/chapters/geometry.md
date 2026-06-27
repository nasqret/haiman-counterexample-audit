# The geometric and regularity branch

Even a perfect reconstruction of Lemma 19 does not automatically prove Theorem A.

## Lemma 5

The paper uses duality to show that a projective arithmetically Cohen-Macaulay variety $S$ satisfying its open-set hypotheses has regularity at most $\dim S+1$. The audit will check:

- equidimensionality and the dualizing-sheaf convention;
- extension across a codimension-two complement;
- the inference from rational curves to $H^0(\widetilde U,\omega_{\widetilde U})=0$;
- the translation between regularity index and Castelnuovo-Mumford regularity.

## Proposition 6

The proposed open subset parameterizes schemes whose radical contains at least $d$ distinct points. The paper claims smoothness, codimension-two complement, and coverage by proper rational curves.

The codimension statement is explicitly delegated to Macaulay2 for $2\le d\le8$. It will be reconstructed as its own computational artifact. The rational-curve family and the assertion that every point is in a $GL(d)$-translate of that family will be checked scheme-theoretically, not only set-theoretically.
