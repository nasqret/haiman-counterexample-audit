#!/usr/bin/env python3
"""Find nonzero minor supports with the degree-89/90 target weights.

This is the first executable gate in the degree-89 attack plan.  It solves
binary weight equations for minor supports and then checks nonvanishing at the
recorded finite-field specialization.

Boundary: this script certifies torus-weight support witnesses.  A nonzero
minor of the right weight is not automatically a highest-weight vector, an
isotypic projection, or a J_8-membership/nonmembership certificate.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Iterable

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp

from analyze_minor_weight import (
    determinant_mod,
    relation_weight,
    selected_matrix,
    selected_minor_weight,
    variable_weight,
)


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "results/certificates/lemma19_matrix_manifest.json"
NONZERO_CERTIFICATE = ROOT / "results/certificates/lemma19_nonzero_minor.json"
MINIMALITY_MODULES = (
    ROOT / "results/certificates/lemma19_minimality_relevant_modules.json"
)
OUTPUT = ROOT / "results/certificates/lemma19_weight_supports.json"
BOREL_ORDER = (4, 5, 6, 7, 8, 1, 2, 3)


def add(left: tuple[int, ...], right: tuple[int, ...]) -> tuple[int, ...]:
    return tuple(a + b for a, b in zip(left, right))


def subtract(left: tuple[int, ...], right: tuple[int, ...]) -> tuple[int, ...]:
    return tuple(a - b for a, b in zip(left, right))


def orient_for_borel(partition: Iterable[int]) -> tuple[int, ...]:
    """Return the standard-coordinate weight for the fixed Borel order."""
    values = tuple(partition)
    # BOREL_ORDER is 4,5,6,7,8,1,2,3, so standard coordinates 1,2,3,4,5,6,7,8
    # receive the last three entries followed by the first five.
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


def determinant_or_zero(
    manifest: dict,
    assignments: dict[tuple[int, int, int], int],
    prime: int,
    selected_rows: list[int],
    selected_columns: list[int],
) -> int:
    try:
        return determinant_mod(
            selected_matrix(
                manifest,
                assignments,
                prime,
                selected_rows,
                selected_columns,
            ),
            prime,
        )
    except ValueError:
        return 0


class BinaryWeightSolver:
    def __init__(self, manifest: dict) -> None:
        self.column_weights = np.array(
            [variable_weight(tuple(column)) for column in manifest["columns"]],
            dtype=float,
        ).T
        self.column_count = len(manifest["columns"])

    def solve(
        self,
        *,
        size: int,
        target_weight_sum: tuple[int, ...],
        exclusions: list[tuple[int, ...]],
    ) -> tuple[int, ...] | None:
        rows = [np.ones(self.column_count)]
        lower = [size]
        upper = [size]
        for coordinate in range(8):
            rows.append(self.column_weights[coordinate])
            lower.append(target_weight_sum[coordinate])
            upper.append(target_weight_sum[coordinate])
        for excluded_columns in exclusions:
            row = np.zeros(self.column_count)
            row[list(excluded_columns)] = 1
            rows.append(row)
            lower.append(-np.inf)
            upper.append(size - 1)

        result = milp(
            c=np.zeros(self.column_count),
            integrality=np.ones(self.column_count),
            bounds=Bounds(0, 1),
            constraints=LinearConstraint(
                np.array(rows),
                np.array(lower),
                np.array(upper),
            ),
            options={"time_limit": 10},
        )
        if not result.success:
            return None
        chosen = tuple(
            index for index, value in enumerate(result.x) if value > 0.5
        )
        if len(chosen) != size:
            raise RuntimeError(
                f"MILP returned {len(chosen)} columns for requested size {size}"
            )
        return chosen


def find_nonzero_weight_minor(
    *,
    manifest: dict,
    assignments: dict[tuple[int, int, int], int],
    prime: int,
    solver: BinaryWeightSolver,
    selected_rows: list[int],
    target_minor_weight: tuple[int, ...],
    max_singular_exclusions: int = 32,
) -> dict:
    row_weight_sum = (0,) * 8
    for row in selected_rows:
        row_weight_sum = add(
            row_weight_sum,
            relation_weight(tuple(manifest["rows"][row])),
        )
    target_column_weight_sum = subtract(row_weight_sum, target_minor_weight)
    exclusions: list[tuple[int, ...]] = []
    for attempt in range(1, max_singular_exclusions + 2):
        columns = solver.solve(
            size=len(selected_rows),
            target_weight_sum=target_column_weight_sum,
            exclusions=exclusions,
        )
        if columns is None:
            return {
                "status": "no_more_weight_supports_found_by_milp",
                "attempts": attempt,
                "singular_supports_excluded": len(exclusions),
                "target_minor_weight": list(target_minor_weight),
                "target_column_weight_sum": list(target_column_weight_sum),
            }
        selected_columns = list(columns)
        determinant = determinant_or_zero(
            manifest,
            assignments,
            prime,
            selected_rows,
            selected_columns,
        )
        if determinant:
            actual_weight = selected_minor_weight(
                manifest,
                selected_rows,
                selected_columns,
            )
            if actual_weight != target_minor_weight:
                raise RuntimeError(
                    f"weight mismatch: expected {target_minor_weight}, got {actual_weight}"
                )
            return {
                "status": "nonzero_weight_support_found",
                "attempts": attempt,
                "singular_supports_excluded": len(exclusions),
                "target_minor_weight": list(target_minor_weight),
                "target_column_weight_sum": list(target_column_weight_sum),
                "selected_rows": selected_rows,
                "selected_columns": selected_columns,
                "determinant_mod_prime": determinant,
            }
        exclusions.append(columns)

    return {
        "status": "singular_exclusion_limit_reached",
        "attempts": max_singular_exclusions + 1,
        "singular_supports_excluded": len(exclusions),
        "target_minor_weight": list(target_minor_weight),
        "target_column_weight_sum": list(target_column_weight_sum),
    }


def write_json(path: Path, value: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(value, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def main() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    certificate = json.loads(NONZERO_CERTIFICATE.read_text(encoding="utf-8"))
    modules = json.loads(MINIMALITY_MODULES.read_text(encoding="utf-8"))
    prime = int(certificate["prime"])
    assignments = {
        tuple(record["variable"]): int(record["value"])
        for record in certificate["assignments"]
    }
    solver = BinaryWeightSolver(manifest)

    nu = tuple(modules["nu"])
    degree90_weight = orient_for_borel(nu)
    degree90_columns = certificate["claimed_weight_candidate"]["columns"]
    degree90_rows = list(range(90))
    degree90_determinant = determinant_or_zero(
        manifest,
        assignments,
        prime,
        degree90_rows,
        degree90_columns,
    )
    degree90_actual_weight = selected_minor_weight(
        manifest,
        degree90_rows,
        degree90_columns,
    )
    if degree90_determinant == 0 or degree90_actual_weight != degree90_weight:
        raise RuntimeError("recorded degree-90 claimed-weight witness no longer verifies")

    candidate_records = []
    for candidate in modules["candidate_modules"]:
        lam = tuple(candidate["degree_89_highest_weight_lambda"])
        target_weight = orient_for_borel(lam)
        witness = None
        failed_rows = []
        for deleted_row in range(90):
            selected_rows = [row for row in range(90) if row != deleted_row]
            trial = find_nonzero_weight_minor(
                manifest=manifest,
                assignments=assignments,
                prime=prime,
                solver=solver,
                selected_rows=selected_rows,
                target_minor_weight=target_weight,
            )
            if trial["status"] == "nonzero_weight_support_found":
                witness = trial
                witness["deleted_row"] = deleted_row
                break
            failed_rows.append({"deleted_row": deleted_row, **trial})
        if witness is None:
            candidate_records.append(
                {
                    "candidate_index": candidate["index"],
                    "lambda": list(lam),
                    "oriented_weight": list(target_weight),
                    "status": "no_nonzero_weight_support_found",
                    "failed_rows": failed_rows,
                }
            )
            continue
        candidate_records.append(
            {
                "candidate_index": candidate["index"],
                "lambda": list(lam),
                "oriented_weight": list(target_weight),
                "source_boundary": (
                    "This is a nonzero determinant of the correct torus weight. "
                    "It is not yet certified to be a highest-weight vector or "
                    "an embedded Schur submodule in Sym^89(S_mu)."
                ),
                **witness,
            }
        )

    payload = {
        "schema_version": 1,
        "source": "degree-89 attack plan, Phase A",
        "status": (
            "nonzero_weight_support_witnesses_found"
            if all(
                record["status"] == "nonzero_weight_support_found"
                for record in candidate_records
            )
            else "partial"
        ),
        "solver": "scipy.optimize.milp / HiGHS",
        "prime": prime,
        "borel_order": list(BOREL_ORDER),
        "degree90_target": {
            "nu": list(nu),
            "oriented_weight": list(degree90_weight),
            "selected_rows": degree90_rows,
            "selected_columns": degree90_columns,
            "determinant_mod_prime": degree90_determinant,
            "boundary": (
                "This is the previously certified nonzero weight-compatible "
                "maximal minor. Existing raising-operator checks show this "
                "particular determinant is not itself highest-weight."
            ),
        },
        "degree89_candidates": candidate_records,
        "computed_assertions": [
            (
                "For every one of the 15 certified tensor/LR candidates lambda, "
                "a nonzero 89 x 89 determinant of the Borel-oriented lambda "
                "weight was found at the recorded finite-field specialization."
            ),
            (
                "The witnesses prove that the exact torus weights are present "
                "in the reconstructed cofactor/minor span; they do not prove "
                "highest-weight annihilation, Schur-isotypic projection, or "
                "J_8 membership."
            ),
        ],
        "next_required_artifact": (
            "results/certificates/lemma19_degree89_highest_weight_spaces.json"
        ),
    }
    write_json(OUTPUT, payload)
    print(
        json.dumps(
            {
                "status": payload["status"],
                "degree89_witnesses": sum(
                    1
                    for record in candidate_records
                    if record["status"] == "nonzero_weight_support_found"
                ),
                "output": str(OUTPUT.relative_to(ROOT)),
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
