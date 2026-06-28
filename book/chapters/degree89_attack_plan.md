# Degree-89 minimality attack plan

This plan attacks the questions raised around the files
`artifacts/sage/enumerate_predecessors.sage` and
`artifacts/common/analyze_minor_weight.py`.

The current state is:

- the 15 degree-89 candidates \(V=S_\lambda W\) are certified only at the
  Littlewood--Richardson/tensor-product level;
- the certified nonzero \(89\times89\) minor refutes the printed phrase
  "each minor", but it is not itself a plausible nonminimality witness for
  the target \(S_\nu W\);
- the missing object is an actual embedded degree-89 Schur copy inside
  \(\operatorname{Sym}^{89}(S_\mu W)\), together with its relation to \(J_8\)
  and to the degree-90 target.

## Target statement

Let

$$
\mu=(3,1,1,1,1,1,1,0),
\qquad
\nu=(133,130,126,122,119,60,60,60).
$$

For every certified candidate \(\lambda\), decide whether there is a genuine
degree-89 source

$$
V_\lambda=S_\lambda W\subset \operatorname{Sym}^{89}(S_\mu W)
$$

such that multiplication by degree-one variables contributes to the target
degree-90 constituent

$$
S_\nu W\subset \operatorname{Sym}^{90}(S_\mu W).
$$

The decisive map is

$$
S_\mu W\otimes (J_8)_{89}\longrightarrow (J_8)_{90}.
$$

The target \(S_\nu W\) is minimal if its degree-90 copy is outside this image.
It is nonminimal if a degree-89 copy in \(J_8\) maps nontrivially to it.

## What a useful \(V\)-certificate must contain

A useful certificate cannot be only a highest weight.  For a candidate
\(\lambda\), it must contain:

1. the candidate \(\lambda\);
2. the symmetric-power multiplicity
   \([S_\lambda W:\operatorname{Sym}^{89}(S_\mu W)]\);
3. an explicit basis for the relevant multiplicity space, preferably in a
   compressed determinant/cofactor basis rather than the full monomial basis;
4. highest-weight checks for a declared Borel order;
5. an evaluation or rank certificate deciding whether the copy intersects
   \(J_8\);
6. the induced multiplication matrix into the \(S_\nu W\)-isotypic component
   of degree 90.

Weights alone are allowed only as filters.  Positive Schur membership must use
highest-weight annihilation or an explicit isotypic projection.

## Phase A: fix conventions and support enumeration

1. Freeze the Borel convention used for \(\nu\) and the 15 \(\lambda\)'s.
   Current computations use the order

   $$
   4<5<6<7<8<1<2<3.
   $$

2. Extend the row/column weight formula already used in
   `analyze_minor_weight.py`:

   $$
   \operatorname{wt}(\det M_{R,C})
   =
   \sum_{\rho\in R}\operatorname{wt}(\rho)
   -
   \sum_{\gamma\in C}\operatorname{wt}(\gamma).
   $$

3. Enumerate or witness, by dynamic programming or integer programming, the
   row/column supports of:

   - \(90\times90\) minors with target weight in the Weyl orbit of \(\nu\);
   - \(89\times89\) minors with weights compatible with each candidate
     \(\lambda\).

4. Store verified support witnesses, and exact counts where feasible, in
   `results/certificates/lemma19_weight_supports.json`.

Decision gate: if no support exists for a candidate \(\lambda\), that
candidate cannot be produced from the relevant minor/cofactor span.

### Phase A checkpoint

This gate is now partially closed by
`results/certificates/lemma19_weight_supports.json`.

The artifact records:

- the previously certified nonzero \(90\times90\) minor of Borel-oriented
  \(\nu\)-weight;
- for every one of the 15 degree-89 candidates \(\lambda\), a nonzero
  \(89\times89\) determinant of Borel-oriented \(\lambda\)-weight.

This proves that the exact torus weights are present in the reconstructed
minor/cofactor span. It does not prove that any listed determinant is a
highest-weight vector, and it does not construct an embedded copy of
\(S_\lambda W\) inside \(\operatorname{Sym}^{89}(S_\mu W)\).

## Phase B: compute symmetric-power multiplicities

For the 15 candidate partitions, compute

$$
m_\lambda=[S_\lambda W:\operatorname{Sym}^{89}(S_\mu W)].
$$

Planned implementations:

- Sage symmetric functions as the reference implementation;
- an independent Rust or Magma check for the final 15 values, if feasible;
- JSON output in
  `results/certificates/lemma19_symmetric_multiplicities.json`.

### Phase B reduction checkpoint

The reduction target list is now recorded in
`results/certificates/lemma19_symmetric_multiplicity_targets.json`.

For every degree-89 candidate, the question reduces to a coefficient

$$
[S_\alpha V:S_{(30,30,29)}(\operatorname{Sym}^2 V)]
\qquad \dim V=5.
$$

For the degree-90 target, the analogous coefficient is

$$
[S_{(43,40,36,32,29)}V:S_{(30,30,30)}(\operatorname{Sym}^2 V)].
$$

Direct Sage plethysm expansion was tested and is not the intended route: it did
not finish promptly even for one coefficient.

A sharper exact reduction is implemented as a secondary check.  For
\(\pi\in\{(30,30,29),(30,30,30)\}\), Jacobi--Trudi gives

$$
s_\pi(\operatorname{Sym}^2 V)
  =
  \det\bigl(h_{\pi_i-i+j}(\operatorname{Sym}^2 V)\bigr)_{1\le i,j\le 3}.
$$

The symmetric-matrix Cauchy identity gives

$$
h_k(\operatorname{Sym}^2 V)
  =
  \operatorname{Sym}^k(\operatorname{Sym}^2 V)
  =
  \bigoplus_{\lambda\vdash k,\ \ell(\lambda)\le5} S_{2\lambda}V.
$$

Thus every target coefficient is an exact signed sum of ordinary
Littlewood--Richardson product coefficients among partitions of length at most
5.  The script `artifacts/sage/compute_symmetric_multiplicities.sage`
implements this theorem-level reduction.  It remains a secondary
implementation because its current direct-summation form did not return the
first target within 90 seconds on the local machine.

The accepted Phase B certificate is produced by the modular Rust route
`artifacts/rust/src/bin/plethysm_multiplicity.rs`.  It computes weight
multiplicities of \(s_\pi(\operatorname{Sym}^2V)\) by semistandard-column
dynamic programming and then applies

$$
m_\alpha
 =
 \sum_{\sigma\in S_5}\operatorname{sgn}(\sigma)\,
 M(\alpha+\rho-\sigma\rho),
\qquad \rho=(4,3,2,1,0),
$$

where \(M(\beta)\) is the target weight multiplicity.  A nonzero residue modulo
a prime rigorously proves \(m_\alpha>0\).  After changing the residue storage
from 64-bit to 32-bit arithmetic, the dense-by-weight DP completed locally in
about 389 seconds.  It produced
`results/certificates/lemma19_symmetric_multiplicities.json`.

The certificate status is
`all_target_multiplicities_nonzero_mod_prime`: all 15 degree-89 candidates and
the degree-90 target have nonzero residues modulo \(1000000007\).  This closes
the occurrence gate in the symmetric powers.  It does not construct explicit
highest-weight vectors in the minor/cofactor span and does not test
\(J_8\)-membership.

Decision gate:

- if \(m_\lambda=0\), then the tensor-level candidate does not actually occur
  in the symmetric degree-89 coordinate ring;
- if \(m_\lambda>0\), construct its multiplicity space in Phase C.

## Phase C: construct highest-weight spaces in compressed form

Avoid expanding degree-89 or degree-90 polynomials.  Work with spans of minors
and determinant evaluations.

For each candidate support set:

1. represent a candidate vector as a linear combination of weight-compatible
   minors;
2. apply the simple raising operators already implemented in
   `analyze_minor_weight.py`;
3. evaluate the determinant and derivative expressions over one or more
   finite fields;
4. solve the linear system imposing raising-operator annihilation.

Expected outputs:

- `results/certificates/lemma19_degree90_highest_weight_space.json`;
- `results/certificates/lemma19_degree89_highest_weight_spaces.json`.

Decision gate:

- a nonzero degree-90 highest-weight vector certifies that the maximal-minor
  span contains the required \(S_\nu W\) constituent;
- nonzero degree-89 highest-weight vectors give explicit \(V\)-candidates to
  test against \(J_8\).

## Phase D: test \(J_8\)-membership of degree-89 candidates

Since \(J_8\) is the reduced ideal of the principal component, a polynomial is
not in \(J_8\) if it evaluates nonzero at one point of the principal component.

For multiplicity spaces, use several radical configurations in the dense open
subset of \(P_8\).  Build the evaluation matrix on the candidate multiplicity
space.

Acceptance criteria:

- full column rank of this evaluation matrix proves that no nonzero vector in
  the tested multiplicity space lies in \(J_8\);
- a nonzero kernel gives a candidate degree-89 element of \(J_8\), but this
  needs exact confirmation because sampled vanishing is only heuristic until
  promoted to a rank certificate.

Expected output:

`results/certificates/lemma19_degree89_j8_membership.json`.

## Phase E: compute the multiplication image into \(S_\nu W\)

For any degree-89 subspace that survives Phase D as a possible \(J_8\)-source,
compute the multiplication map

$$
S_\mu W\otimes S_\lambda W\longrightarrow S_\nu W.
$$

Because the certified LR coefficient \(c^\nu_{\lambda,\mu}\) is 1 for every
listed \(\lambda\), each candidate has at most one target channel after the
choice of embedded copy.  The computation should therefore record a scalar or
small matrix on multiplicity spaces, not an expanded degree-90 polynomial.

Expected output:

`results/certificates/lemma19_multiplication_to_nu.json`.

Decision gate:

- if the \(S_\nu W\)-projection of the multiplication image contains the
  certified degree-90 vector, then the claimed generator is nonminimal;
- if the image is zero or misses the degree-90 target for every candidate,
  then Lee's minimality claim is repaired at this stage.

## Phase F: final certificates

There are two possible successful outcomes.

### Nonminimality certificate

Produce:

1. a candidate \(\lambda\);
2. an explicit degree-89 vector \(g\in (J_8)_{89}\cap S_\lambda W\);
3. a multiplication witness showing that \(S_\mu W\cdot g\) has nonzero
   projection to \(S_\nu W\);
4. finite-field rank/evaluation data, repeated over an independent prime or
   lifted to characteristic zero.

This would refute the minimality step in Lemma 19.

### Minimality certificate

Produce, for each of the 15 \(\lambda\)'s:

1. the symmetric-power multiplicity;
2. a basis of the relevant multiplicity space;
3. an exact rank/evaluation certificate proving that the corresponding
   subspace has zero intersection with \(J_8\), or a multiplication certificate
   proving that its image misses \(S_\nu W\).

This would repair the specific objection raised by the degree-89 discussion,
though the rest of Lee's theorem would still require the downstream geometric
audit.

## Implementation order

1. `lemma19_weight_supports.json`: produce nonzero exact-weight support
   witnesses for the candidate minor/cofactor supports. **Done for the
   Borel-oriented degree-90 target and all 15 degree-89 candidates.**
2. `lemma19_symmetric_multiplicities.json`: compute \(m_\lambda\) for the 15
   candidates. **Done modulo \(1000000007\), with nonzero residues for all 15
   degree-89 candidates and the degree-90 target.**
3. `lemma19_degree90_highest_weight_space.json`: prove the maximal-minor span
   actually contains the required \(S_\nu W\).
4. `lemma19_degree89_highest_weight_spaces.json`: construct candidate
   degree-89 highest-weight spaces.
5. `lemma19_degree89_j8_membership.json`: test the degree-89 candidates on the
   principal component.
6. `lemma19_multiplication_to_nu.json`: decide minimality versus
   nonminimality.

The early gates are intentionally cheap.  The expensive symmetric-power and
\(J_8\)-membership work should start only after support counts and multiplicity
counts show that a candidate can actually matter.
