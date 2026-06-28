# Auditing the claimed counterexample to Haiman's conjecture

Kyungyong Lee's 2010 article claims that the principal component of

$$
\operatorname{Hilb}^{9}(\mathbb C^8)
$$

is not locally Cohen-Macaulay at the point $\mathfrak m^2$. If correct, the result contradicts the Cohen-Macaulay conjecture attributed to Haiman and propagates to all $d\ge8$, $n\ge9$ in the form stated in the paper {cite:p}`lee2010`.

The decisive computation was not published. On pp. 1358-1359, Lemma 19 replaces it with the phrases "one can check" and "exhaustive computations show." This book reconstructs that computation, identifies every mathematical dependency, and records evidence at a level that does not require trusting a single CAS.

:::{admonition} Current verdict
:class: warning
**Open audit.** The source and missing code have been reconstructed far enough to certify nonvanishing and refute two literal Lemma 19 representation claims. The counterexample itself is not yet independently certified or refuted.
:::

The degree-89 Littlewood--Richardson search is now separated from the
minimality problem. The audit can list the 15 Schur modules
$V=S_\lambda W$ for which $V\otimes S_\mu W$ can feed the target
$S_\nu W$, but it has not yet constructed those copies inside
$\operatorname{Sym}^{89}(S_\mu W)$ or tested their $J_8$ membership.

## What counts as a successful verification?

A successful audit must provide:

1. a complete construction of the $90\times115$ matrix from the printed relations;
2. a compact certificate that a specified maximal minor is nonzero;
3. a proof that the minors lie in the reduced ideal $J_8$;
4. a characteristic-zero certificate for the claimed Schur-module membership;
5. a complete degree-89 exclusion establishing minimality;
6. an audit of the geometric and regularity arguments downstream of Lemma 19;
7. cross-checks in SageMath, Magma, Singular, Oscar.jl, and a small Rust verifier.

A fast numerical rank calculation solves only item 2.
