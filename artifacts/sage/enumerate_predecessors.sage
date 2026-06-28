#!/usr/bin/env sage
"""Enumerate and certify the fifteen degree-89 predecessor partitions.

A candidate lambda must satisfy:
  c^NU_{lambda,MU} > 0,
  V_lambda occurs in V_MU^(tensor 89).

Tensor occurrence is certified by an explicit chain of 88 nonzero
Littlewood-Richardson coefficients from MU to lambda.

Important boundary:
  These are exactly the tensor/Littlewood-Richardson candidates for a
  degree-89 Schur module V = S_lambda(W) whose product with the degree-one
  coordinate module S_MU(W) can contain the target S_NU(W).  This file does
  not construct the actual embedded copies inside Sym^89(S_MU(W)), and it
  does not test whether any such embedded copy lies in J_8.
"""

from itertools import combinations
from pathlib import Path
import argparse
import hashlib
import json

from sage.all import IntegerVectors
from sage.libs.lrcalc.lrcalc import lrcoef


MU = (3, 1, 1, 1, 1, 1, 1, 0)
NU = (133, 130, 126, 122, 119, 60, 60, 60)
TENSOR_POWER = 89


def canonical_json(value):
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=True,
        default=int,
    )


def is_partition(value):
    return all(value[index] >= value[index + 1] for index in range(len(value) - 1))


def dominance_slacks(lam, tensor_power):
    return [
        tensor_power * sum(MU[:length]) - sum(lam[:length])
        for length in range(1, 9)
    ]


HORIZONTAL_DELTAS = [
    tuple(int(value) for value in vector)
    for vector in IntegerVectors(2, 5)
]
VERTICAL_DELTAS = []
for pair in combinations(range(3), 2):
    vector = [0, 0, 0]
    for index in pair:
        vector[index] = 1
    VERTICAL_DELTAS.append(tuple(vector))


def conjugate(partition):
    if not partition or partition[0] == 0:
        return ()
    return tuple(
        sum(1 for value in partition if value >= column)
        for column in range(1, partition[0] + 1)
    )


def dominates_twos(partition, count):
    """Kostka positivity condition partition >= (2^count)."""
    if sum(partition) != 2 * count:
        return False
    prefix = 0
    for length, value in enumerate(partition, start=1):
        prefix += value
        if prefix < 2 * min(length, count):
            return False
    return True


def tensor_lr_chain(lam):
    """Build an explicit MU^tensor(89) -> lambda LR path.

    All fifteen candidates saturate the five-row dominance facet. On that
    facet, subtracting one box from each of the first five rows leaves a
    horizontal two-strip problem; the last three rows give a vertical
    two-strip problem. Kostka dominance keeps the greedy removals extendable.
    Every resulting full GL(8) step is checked again with lrcalc.
    """
    beta = tuple(lam)
    backward_partitions = [beta]
    backward_coefficients = []

    for tensor_power in range(TENSOR_POWER, 1, -1):
        rho = tuple(beta[index] - tensor_power for index in range(5))
        sigma = tuple(beta[5:])
        options = []

        for horizontal in HORIZONTAL_DELTAS:
            rho_previous = tuple(
                rho[index] - horizontal[index]
                for index in range(5)
            )
            if min(rho_previous) < 0 or not is_partition(rho_previous):
                continue
            if not all(
                rho[index]
                >= rho_previous[index]
                >= (rho[index + 1] if index + 1 < 5 else 0)
                for index in range(5)
            ):
                continue
            if not dominates_twos(rho_previous, tensor_power - 1):
                continue

            for vertical in VERTICAL_DELTAS:
                sigma_previous = tuple(
                    sigma[index] - vertical[index]
                    for index in range(3)
                )
                if min(sigma_previous) < 0 or not is_partition(sigma_previous):
                    continue
                if not dominates_twos(
                    conjugate(sigma_previous),
                    tensor_power - 1,
                ):
                    continue

                alpha = tuple(
                    tensor_power - 1 + rho_previous[index]
                    for index in range(5)
                ) + sigma_previous
                if not is_partition(alpha):
                    continue
                coefficient = int(lrcoef(list(beta), list(alpha), list(MU)))
                if coefficient == 0:
                    continue
                score = sum(
                    abs(tensor_power * alpha[index] - (tensor_power - 1) * beta[index])
                    for index in range(8)
                )
                options.append((score, alpha, coefficient))

        if not options:
            raise ValueError(("no LR predecessor", tensor_power, beta))
        options.sort(key=lambda item: (item[0], item[1]))
        _, alpha, coefficient = options[0]
        backward_partitions.append(alpha)
        backward_coefficients.append(coefficient)
        beta = alpha

    assert beta == MU
    return {
        "partitions_from_power_1_to_89": [
            list(partition)
            for partition in reversed(backward_partitions)
        ],
        "step_lr_coefficients": list(reversed(backward_coefficients)),
    }


def enumerate_candidates():
    lr_candidates = []
    tensor_candidates = []
    for delta_vector in IntegerVectors(9, 8):
        delta = tuple(int(value) for value in delta_vector)
        lam = tuple(NU[index] - delta[index] for index in range(8))
        if min(lam) < 0 or not is_partition(lam):
            continue
        coefficient = int(lrcoef(list(NU), list(lam), list(MU)))
        if coefficient == 0:
            continue
        record = {
            "lambda": list(lam),
            "delta": list(delta),
            "littlewood_richardson_coefficient": coefficient,
            "dominance_slacks": dominance_slacks(lam, TENSOR_POWER),
        }
        lr_candidates.append(record)
        if min(record["dominance_slacks"]) < 0:
            continue
        record["tensor_lr_chain"] = tensor_lr_chain(lam)
        tensor_candidates.append(record)

    assert len(lr_candidates) == 102
    assert len(tensor_candidates) == 15
    assert all(
        record["littlewood_richardson_coefficient"] == 1
        for record in tensor_candidates
    )
    return lr_candidates, tensor_candidates


def minimality_relevant_payload(tensor_candidates):
    """Compact record answering which V can contribute to S_NU in degree 90.

    The answer is at the level of Schur highest weights and tensor-product
    occurrence.  It is a necessary list for minimality testing, not the missing
    symmetric-power embedding/J_8-membership computation.
    """
    modules = []
    for index, record in enumerate(tensor_candidates, start=1):
        modules.append(
            {
                "index": index,
                "degree_89_highest_weight_lambda": record["lambda"],
                "degree_one_variables_highest_weight_mu": list(MU),
                "target_degree_90_highest_weight_nu": list(NU),
                "littlewood_richardson_coefficient_c_nu_lambda_mu": record[
                    "littlewood_richardson_coefficient"
                ],
                "occurs_in_tensor_power_89": True,
                "tensor_power_certificate": {
                    "certificate_file": (
                        "results/certificates/"
                        "lemma19_predecessor_partitions.json"
                    ),
                    "json_pointer": (
                        f"/tensor_candidates/{index - 1}/tensor_lr_chain"
                    ),
                    "chain_length": len(
                        record["tensor_lr_chain"]["step_lr_coefficients"]
                    ),
                    "all_step_lr_coefficients_nonzero": True,
                },
                "symmetric_power_embedding_status": "open_not_constructed",
                "j8_membership_status": "open_not_tested",
                "minimality_relevance": (
                    "This is a Schur-highest-weight candidate V=S_lambda(W) "
                    "such that V tensor S_mu(W) can contain S_nu(W). It is "
                    "not yet an explicitly embedded copy in Sym^89(S_mu(W))."
                ),
            }
        )
    payload = {
        "mu": list(MU),
        "nu": list(NU),
        "tensor_power": TENSOR_POWER,
        "candidate_count": len(modules),
        "candidate_modules": modules,
    }
    return payload


def main():
    root = Path(__file__).resolve().parents[2]
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--output",
        type=Path,
        default=root / "results/certificates/lemma19_predecessor_partitions.json",
    )
    parser.add_argument(
        "--minimality-output",
        type=Path,
        default=(
            root
            / "results/certificates/lemma19_minimality_relevant_modules.json"
        ),
    )
    args = parser.parse_args()

    lr_candidates, tensor_candidates = enumerate_candidates()
    payload = {
        "mu": list(MU),
        "nu": list(NU),
        "tensor_power": TENSOR_POWER,
        "total_lr_candidates": len(lr_candidates),
        "total_tensor_candidates": len(tensor_candidates),
        "tensor_candidates": tensor_candidates,
    }
    digest = hashlib.sha256(canonical_json(payload).encode("ascii")).hexdigest()
    certificate = {
        "schema_version": 1,
        "source": "Lee 2010, Lemma 19, p. 1359",
        "payload_sha256": digest,
        "mathematical_implication": (
            "There are 102 LR predecessors of nu. Dominance excludes 87 "
            "from mu^tensor(89). For each of the remaining fifteen, an "
            "explicit chain of 88 nonzero LR coefficients certifies tensor occurrence."
        ),
        **payload,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(
        json.dumps(certificate, indent=2, sort_keys=True, default=int) + "\n",
        encoding="utf-8",
    )

    minimality_payload = minimality_relevant_payload(tensor_candidates)
    minimality_digest = hashlib.sha256(
        canonical_json(minimality_payload).encode("ascii")
    ).hexdigest()
    minimality_certificate = {
        "schema_version": 1,
        "source": "Lee 2010, Lemma 19, p. 1359",
        "payload_sha256": minimality_digest,
        "purpose": (
            "Compact list of the degree-89 Schur modules V=S_lambda(W) that "
            "can contribute to the target S_nu(W) after multiplication by "
            "the degree-one coordinate module S_mu(W), at the tensor/LR level."
        ),
        "computed_assertions": [
            (
                "There are exactly fifteen lambda with c^nu_{lambda,mu}>0 "
                "and certified occurrence in S_mu(W)^(tensor 89)."
            ),
            (
                "For every listed lambda, c^nu_{lambda,mu}=1 and the "
                "predecessor certificate supplies an 88-step LR chain."
            ),
        ],
        "theorem_boundary": (
            "These tensor/LR candidates are necessary for a degree-89 "
            "source of S_nu(W), but tensor occurrence does not construct a "
            "copy inside Sym^89(S_mu(W))."
        ),
        "open_questions": [
            (
                "Construct bases for the actual copies of these modules in "
                "Sym^89(S_mu(W))."
            ),
            (
                "Test whether those embedded copies lie in J_8 or whether "
                "their multiplication image contributes to the target "
                "degree-90 S_nu(W)."
            ),
        ],
        **minimality_payload,
    }
    args.minimality_output.parent.mkdir(parents=True, exist_ok=True)
    args.minimality_output.write_text(
        json.dumps(
            minimality_certificate,
            indent=2,
            sort_keys=True,
            default=int,
        )
        + "\n",
        encoding="utf-8",
    )
    print(
        canonical_json(
            {
                "status": "ok",
                "total_lr_candidates": len(lr_candidates),
                "total_tensor_candidates": len(tensor_candidates),
                "payload_sha256": digest,
                "minimality_payload_sha256": minimality_digest,
            }
        )
    )


if __name__ == "__main__":
    main()
