# Degree-89 attack plan

This note tracks the plan for the questions raised about
`enumerate_predecessors.sage` and `analyze_minor_weight.py`.

Short version:

1. Freeze the Borel convention and weight-support enumeration.
2. Compute the symmetric-power multiplicities
   \([S_\lambda W:\operatorname{Sym}^{89}(S_\mu W)]\) for the 15 certified
   tensor-level candidates.
3. Construct highest-weight spaces in compressed determinant/cofactor bases,
   not in the full monomial basis.
4. Evaluate those multiplicity spaces on radical points of the principal
   component to decide intersection with \(J_8\).
5. Compute the multiplication image
   \(S_\mu W\otimes (J_8)_{89}\to (J_8)_{90}\) and project to \(S_\nu W\).

The decision criterion is:

- nonzero degree-89 \(J_8\)-source mapping to \(S_\nu W\) means nonminimality;
- zero intersection with \(J_8\), or zero/missing multiplication image for all
  15 candidates, repairs this part of Lemma 19.

Full plan: `../../book/chapters/degree89_attack_plan.md`.

Current Phase A result:

- `results/certificates/lemma19_weight_supports.json` contains nonzero
  exact-torus-weight minor witnesses for the degree-90 target and all 15
  degree-89 candidates.
- These witnesses are not highest-weight certificates. The next step is the
  raising-operator linear system on the exact-weight spans.

Current Phase B reduction:

- `results/certificates/lemma19_symmetric_multiplicity_targets.json` lists the
  GL5 plethysm coefficients whose values would give the symmetric-power
  multiplicities.
- The coefficients are still open; direct Sage plethysm expansion was too slow.
