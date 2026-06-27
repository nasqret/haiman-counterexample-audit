# Reconstructing Lemma 19

Set

$$
A=\{p_{r,st}:1\le r\le3,\ 4\le s\le t\le8\},
$$

a set of $3\binom{6}{2}=45$ variables.

The article selects 90 relations $C(a;j,(i,k))$. Every monomial in these relations has one factor in $A$ and one outside $A$. After centering the complementary factor, the relations take the form

$$
C=M(A)z,
$$

where $M$ is a $90\times115$ matrix of signed variables from $A$.

## Why a full-rank specialization is useful

Choose integer values for the 45 variables and reduce modulo a prime $p$. If the resulting matrix has rank 90, Gaussian elimination returns 90 pivot columns and a nonzero determinant residue. The corresponding determinant polynomial is therefore nonzero over $\mathbb F_p$, hence its integer lift cannot be the zero polynomial over $\mathbb Q$.

This is a small, independently verifiable certificate of **nonvanishing**. It does not establish:

- that the polynomial vanishes on the principal component;
- that it belongs to the stated Schur module;
- that it is a minimal generator of $J_8$.

Those require separate arguments and certificates.

## Certified result

The reconstructed matrix has 1410 nonzero signed-variable entries and canonical payload hash

```text
dc4bb49eaab862c6f11ce83b9129fd9c16adc6b6d4e591fc4be8ea972d9176ac
```

At the 45 recorded values modulo $p=1{,}000{,}003$, the matrix has rank 90. SageMath selected a pivot minor with


$$
\det(M_{\mathrm{pivot}})=970351\pmod{1{,}000{,}003}.
$$


A separately written Rust program regenerates the matrix from the formulas, matches the payload hash, and recomputes this determinant. Therefore the selected maximal-minor polynomial is nonzero in characteristic zero.

## A literal assertion fails

The coordinate $p_{r,st}$ has torus weight


$$
(1,\ldots,1)-e_r+e_s+e_t.
$$


The certified nonzero pivot minor is weight-homogeneous of weight


$$
(60,60,60,141,125,128,120,116).
$$


Its dominant reordering begins with 141, whereas the claimed irreducible has highest weight beginning with 133. Hence this minor cannot lie in that irreducible. This disproves the paper's literal phrase that **any** maximal minor lies in the stated Schur module.

A different certified nonzero minor has Weyl-orbit weight


$$
(60,60,60,133,130,126,122,119)
$$


and determinant $300834\pmod{1000003}$. This repairs the weight obstruction but does not yet prove highest-weight annihilation or nonzero projection to the desired isotypic component.

In fact, this selected weight-compatible determinant is not itself a highest-weight vector for the Borel order

$$
4<5<6<7<8<1<2<3.
$$

Applying the corresponding simple raising operators and evaluating at the same certified specialization gives

| Operator | Derivative modulo $1000003$ |
|---|---:|
| $E_{45}$ | 685026 |
| $E_{56}$ | 176188 |
| $E_{67}$ | 140239 |
| $E_{78}$ | 78485 |
| $E_{81}$ | 0 |
| $E_{12}$ | 0 |
| $E_{23}$ | 605583 |

One nonzero residue is enough to prove that the derivative polynomial is nonzero over characteristic zero. Therefore this particular determinant is only a weight-compatible witness, not the highest-weight vector Lee's proof needs.

## A second literal assertion fails

The paper also says that each $89\times89$ minor belongs to one of the 15 degree-89 modules listed by the Littlewood--Richardson search. The certified invertible $90\times90$ pivot minor gives an immediate test case: delete row 0 and the first pivot column. The resulting $89\times89$ determinant is

$$
421057\pmod{1000003},
$$

so it is nonzero. Its torus weight is

$$
(60,59,59,140,123,126,119,115),
$$

with dominant reordering

$$
(140,126,123,119,115,60,59,59).
$$

Every one of the 15 certified degree-89 predecessor partitions has first part at most 132. Hence this nonzero $89\times89$ minor is not a weight of any of those modules. This refutes the printed sentence as written, though it does not by itself settle the minimality claim for the required degree-90 constituent.

## Why all maximal minors are in $J_8$

On $P_8$, the ideal-projector equations give $C=Mz=0$. The centered vector $z$ is nonzero on a nonempty open subset: the radical standard simplex $\{0,e_1,\ldots,e_8\}$ already supplies a witness. Therefore $M$ has a nontrivial kernel and rank at most 89 on a dense open subset of the reduced irreducible principal component. Every maximal minor vanishes densely, hence belongs to $J_8$.
