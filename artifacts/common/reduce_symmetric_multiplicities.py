#!/usr/bin/env python3
"""Reduce the degree-89/90 symmetric-power multiplicities to GL(5) plethysms.

For the Borel order 4<5<6<7<8<1<2<3, the relevant weight splits into a
GL(3) part and a GL(5) part.  Cauchy's formula for
Sym^m((det E) tensor E^* tensor Sym^2 V) reduces the multiplicity question to
coefficients of S_alpha(V) in S_pi(Sym^2 V), where dim(V)=5.

This script only performs the reduction.  It does not compute the plethysm
coefficients.
"""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[2]
MINIMALITY_MODULES = (
    ROOT / "results/certificates/lemma19_minimality_relevant_modules.json"
)
OUTPUT = ROOT / "results/certificates/lemma19_symmetric_multiplicity_targets.json"
BOREL_ORDER = (4, 5, 6, 7, 8, 1, 2, 3)


def canonical_json(value: object) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=True)


def orient_for_borel(partition: Iterable[int]) -> tuple[int, ...]:
    values = tuple(partition)
    return (
        values[5],
        values[6],
        values[7],
        values[0],
        values[1],
        values[2],
        values[3],
        values[4],
    )


def reduced_target(partition: Iterable[int], degree: int) -> dict:
    oriented = orient_for_borel(partition)
    gl3_weight = oriented[:3]
    gl5_weight = oriented[3:]
    # If det(E)^degree tensor S_pi(E^*) has highest GL(3) weight gl3_weight,
    # then gl3_weight = (degree-pi_3, degree-pi_2, degree-pi_1).
    pi = (
        degree - gl3_weight[2],
        degree - gl3_weight[1],
        degree - gl3_weight[0],
    )
    alpha = tuple(value - degree for value in gl5_weight)
    if any(value < 0 for value in pi) or pi != tuple(sorted(pi, reverse=True)):
        raise ValueError(("invalid GL3 partition", partition, degree, pi))
    if any(value < 0 for value in alpha) or alpha != tuple(sorted(alpha, reverse=True)):
        raise ValueError(("invalid GL5 target", partition, degree, alpha))
    if sum(alpha) != 2 * sum(pi):
        raise ValueError(("degree mismatch", partition, degree, pi, alpha))
    return {
        "original_partition": list(partition),
        "degree": degree,
        "borel_oriented_weight": list(oriented),
        "gl3_partition_pi": list(pi),
        "gl5_target_alpha": list(alpha),
        "coefficient_to_compute": (
            f"[S_{alpha}(V) : S_{pi}(Sym^2 V)] for dim(V)=5"
        ),
    }


def main() -> None:
    modules = json.loads(MINIMALITY_MODULES.read_text(encoding="utf-8"))
    degree90 = reduced_target(modules["nu"], 90)
    degree89 = [
        {
            "candidate_index": candidate["index"],
            **reduced_target(candidate["degree_89_highest_weight_lambda"], 89),
        }
        for candidate in modules["candidate_modules"]
    ]
    payload = {
        "schema_version": 1,
        "source": "degree-89 attack plan, Phase B reduction",
        "status": "reduced_targets_only_coefficients_not_computed",
        "borel_order": list(BOREL_ORDER),
        "formula": (
            "For degree m and Borel-oriented weight (e1,e2,e3,f1,...,f5), "
            "the GL3 partition is pi=(m-e3,m-e2,m-e1) and the GL5 target is "
            "alpha=(f1-m,...,f5-m). The required multiplicity is the "
            "coefficient of S_alpha(V) in S_pi(Sym^2 V), dim(V)=5."
        ),
        "degree90_target": degree90,
        "degree89_targets": degree89,
        "next_required_artifact": (
            "results/certificates/lemma19_symmetric_multiplicities.json"
        ),
    }
    digest = hashlib.sha256(canonical_json(payload).encode("ascii")).hexdigest()
    certificate = {
        "payload_sha256": digest,
        **payload,
    }
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(
        json.dumps(certificate, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    print(
        json.dumps(
            {
                "status": certificate["status"],
                "degree89_targets": len(degree89),
                "payload_sha256": digest,
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
