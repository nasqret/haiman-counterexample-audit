#!/usr/bin/env sage
r"""
Exact GL(5) plethysm multiplicities for the Lemma 19 degree-89 audit.

The input target artifact reduces the problem to coefficients

    [S_alpha(V) : S_pi(Sym^2 V)],    dim(V)=5.

This script computes those coefficients without expanding the full plethysm.
It uses two standard identities:

1. Jacobi--Trudi for the outer Schur functor:

       s_pi(Sym^2 V) = det(h_{pi_i-i+j}(Sym^2 V)).

   In this audit pi is either (30,30,29) or (30,30,30), so this is a 3 x 3
   determinant.

2. The symmetric-matrix Cauchy identity:

       h_k(Sym^2 V) = Sym^k(Sym^2 V)
                    = direct_sum_{lambda partition k} S_{2 lambda}(V),

   with length(lambda) <= 5 in GL(5).

Therefore each target coefficient is a signed sum of ordinary
Littlewood--Richardson product coefficients among partitions of length at most
5.  No degree-89 polynomial, determinant, or full plethysm expansion is built.
"""

from __future__ import annotations

import hashlib
import itertools
import json
import sys
import time
from pathlib import Path

from sage.all import Partitions
from sage.libs.lrcalc import lrcalc


ROOT = Path(__file__).resolve().parents[2]
TARGETS = ROOT / "results/certificates/lemma19_symmetric_multiplicity_targets.json"
OUTPUT = ROOT / "results/certificates/lemma19_symmetric_multiplicities.json"
N = 5

def canonical_json(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def permutation_sign(perm):
    inversions = 0
    for i in range(len(perm)):
        for j in range(i + 1, len(perm)):
            if perm[i] > perm[j]:
                inversions += 1
    return -1 if inversions % 2 else 1


def even_partitions(k):
    return [
        tuple(2 * int(part) for part in partition)
        for partition in Partitions(k, max_length=N)
    ]


EVEN_PARTITIONS = {}
PAIR_PRODUCT_CACHE = {}


def even_partition_list(k):
    if k not in EVEN_PARTITIONS:
        EVEN_PARTITIONS[k] = even_partitions(k)
    return EVEN_PARTITIONS[k]


def normalized_partition_tuple(partition):
    return tuple(int(part) for part in partition if int(part) != 0)


def subset_of_alpha(partition_tuple, alpha):
    if len(partition_tuple) > len(alpha):
        return False
    for index in range(len(alpha)):
        left = partition_tuple[index] if index < len(partition_tuple) else 0
        if left > alpha[index]:
            return False
    return True


def pair_product_coefficients(left, right):
    """Return Schur expansion coefficients for s_left * s_right.

    The result is cached because Jacobi--Trudi terms reuse the same even
    partitions across the 16 target coefficients.
    """

    key = (tuple(left), tuple(right))
    if key not in PAIR_PRODUCT_CACHE:
        product = lrcalc.mult(list(left), list(right), maxrows=N)
        PAIR_PRODUCT_CACHE[key] = {
            normalized_partition_tuple(partition): int(coefficient)
            for partition, coefficient in product.items()
            if len(partition) <= N
        }
    return PAIR_PRODUCT_CACHE[key]


def product_h_coeff(alpha, ks):
    """Coefficient of S_alpha in h_{ks[0]} h_{ks[1]} h_{ks[2]} at Sym^2 V."""

    alpha = tuple(int(part) for part in alpha)
    parts0 = even_partition_list(ks[0])
    parts1 = even_partition_list(ks[1])
    parts2 = even_partition_list(ks[2])

    intermediate = {}
    for left in parts0:
        for right in parts1:
            for tau, coefficient in pair_product_coefficients(left, right).items():
                if subset_of_alpha(tau, alpha):
                    intermediate[tau] = intermediate.get(tau, 0) + coefficient

    total = 0
    for tau, tau_coefficient in intermediate.items():
        for third in parts2:
            coefficient = int(lrcalc.lrcoef_unsafe(list(alpha), list(tau), list(third)))
            if coefficient:
                total += tau_coefficient * coefficient
    return total, len(intermediate)


def jacobi_trudi_terms(pi):
    pi = tuple(int(part) for part in pi)
    terms = []
    for perm in itertools.permutations(range(3)):
        ks = []
        valid = True
        for i, j in enumerate(perm):
            k = pi[i] - (i + 1) + (j + 1)
            if k < 0:
                valid = False
                break
            ks.append(k)
        if valid:
            terms.append(
                {
                    "permutation": [int(value) + 1 for value in perm],
                    "sign": permutation_sign(perm),
                    "h_degrees": [int(value) for value in ks],
                }
            )
    return terms


def coefficient_for(pi, alpha):
    total = 0
    term_records = []
    started = time.time()
    for term in jacobi_trudi_terms(pi):
        coefficient, intermediate_count = product_h_coeff(alpha, tuple(term["h_degrees"]))
        signed = term["sign"] * coefficient
        total += signed
        term_records.append(
            {
                **term,
                "unsigned_lr_product_coefficient": int(coefficient),
                "signed_contribution": int(signed),
                "intermediate_partitions_after_first_product": int(intermediate_count),
            }
        )
    return {
        "coefficient": int(total),
        "jacobi_trudi_terms": term_records,
        "elapsed_seconds": round(time.time() - started, 3),
    }


def main():
    targets = json.loads(TARGETS.read_text(encoding="utf-8"))
    records = []

    all_targets = [
        {
            "label": "degree90_target",
            **targets["degree90_target"],
        }
    ]
    for target in targets["degree89_targets"]:
        all_targets.append(
            {
                "label": f"degree89_candidate_{target['candidate_index']}",
                **target,
            }
        )

    for index, target in enumerate(all_targets, start=1):
        pi = tuple(target["gl3_partition_pi"])
        alpha = tuple(target["gl5_target_alpha"])
        print(f"[{index}/{len(all_targets)}] pi={pi} alpha={alpha}", file=sys.stderr)
        result = coefficient_for(pi, alpha)
        records.append(
            {
                "label": target["label"],
                "candidate_index": target.get("candidate_index"),
                "degree": target["degree"],
                "original_partition": target["original_partition"],
                "gl3_partition_pi": list(pi),
                "gl5_target_alpha": list(alpha),
                "multiplicity": result["coefficient"],
                "certified_nonzero": result["coefficient"] > 0,
                "jacobi_trudi_terms": result["jacobi_trudi_terms"],
                "elapsed_seconds": result["elapsed_seconds"],
            }
        )
        print(
            f"    multiplicity={result['coefficient']} elapsed={result['elapsed_seconds']}s",
            file=sys.stderr,
        )

    payload = {
        "schema_version": 1,
        "source": "artifacts/sage/compute_symmetric_multiplicities.sage",
        "status": (
            "all_target_multiplicities_positive"
            if all(record["certified_nonzero"] for record in records)
            else "some_target_multiplicity_zero"
        ),
        "method": (
            "Exact Jacobi-Trudi reduction plus Sym^k(Sym^2 V)=sum_{lambda partition k} "
            "S_{2 lambda}(V), evaluated with ordinary Littlewood-Richardson products in Sage."
        ),
        "input_targets_sha256": hashlib.sha256(
            TARGETS.read_bytes()
        ).hexdigest(),
        "dimension": N,
        "degree90_target": records[0],
        "degree89_targets": records[1:],
        "open_limitations": [
            "This proves occurrence/nonoccurrence in the symmetric power but does not construct explicit highest-weight vectors.",
            "This does not test whether any degree-89 copy lies in J_8.",
            "The computation relies on Sage's Schur-basis Littlewood-Richardson multiplication; an independent Magma/Rust cross-check is still desirable.",
        ],
    }
    certificate = {
        "payload_sha256": hashlib.sha256(
            canonical_json(payload).encode("ascii")
        ).hexdigest(),
        **payload,
    }
    OUTPUT.write_text(
        json.dumps(certificate, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    print(
        json.dumps(
            {
                "status": certificate["status"],
                "payload_sha256": certificate["payload_sha256"],
                "degree89_targets": len(certificate["degree89_targets"]),
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
