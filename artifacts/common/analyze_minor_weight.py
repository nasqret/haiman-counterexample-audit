#!/usr/bin/env python3
"""Audit the representation-theoretic claims around Lee's Lemma 19 minors."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "results/certificates/lemma19_matrix_manifest.json"
CERTIFICATE = ROOT / "results/certificates/lemma19_nonzero_minor.json"
PREDECESSORS = ROOT / "results/certificates/lemma19_predecessor_partitions.json"
OUTPUT = ROOT / "results/certificates/lemma19_pivot_weight_audit.json"
OUTPUT_89 = ROOT / "results/certificates/lemma19_89_minor_weight_audit.json"
OUTPUT_RAISING = ROOT / "results/certificates/lemma19_claimed_weight_raising_audit.json"
EXPECTED = (133, 130, 126, 122, 119, 60, 60, 60)
ORIENTED_EXPECTED = (60, 60, 60, 133, 130, 126, 122, 119)
TESTED_BOREL_ORDER = (4, 5, 6, 7, 8, 1, 2, 3)
TESTED_SIMPLE_ROOTS = tuple(zip(TESTED_BOREL_ORDER, TESTED_BOREL_ORDER[1:]))


def add(left: tuple[int, ...], right: tuple[int, ...]) -> tuple[int, ...]:
    return tuple(a + b for a, b in zip(left, right))


def subtract(left: tuple[int, ...], right: tuple[int, ...]) -> tuple[int, ...]:
    return tuple(a - b for a, b in zip(left, right))


def dominated_by(weight: tuple[int, ...], highest: tuple[int, ...]) -> bool:
    return all(
        sum(weight[:length]) <= sum(highest[:length])
        for length in range(1, len(weight) + 1)
    ) and sum(weight) == sum(highest)


def variable_weight(variable: tuple[int, int, int]) -> tuple[int, ...]:
    r, s, t = variable
    result = [1] * 8
    result[r - 1] -= 1
    result[s - 1] += 1
    result[t - 1] += 1
    return tuple(result)


def relation_weight(row: tuple[int, int, int, int]) -> tuple[int, ...]:
    a, j, i, k = row
    result = [2] * 8
    result[a - 1] -= 1
    result[i - 1] += 1
    result[j - 1] += 1
    result[k - 1] += 1
    return tuple(result)


def selected_minor_weight(
    manifest: dict, selected_rows: list[int], selected_columns: list[int]
) -> tuple[int, ...]:
    row_weight_sum = (0,) * 8
    for row in selected_rows:
        row_weight_sum = add(row_weight_sum, relation_weight(tuple(manifest["rows"][row])))
    column_weight_sum = (0,) * 8
    for column in selected_columns:
        column_weight_sum = add(
            column_weight_sum,
            variable_weight(tuple(manifest["columns"][column])),
        )
    return subtract(row_weight_sum, column_weight_sum)


def selected_matrix(
    manifest: dict,
    assignments: dict[tuple[int, int, int], int],
    prime: int,
    selected_rows: list[int],
    selected_columns: list[int],
) -> list[list[int]]:
    row_index = {row: index for index, row in enumerate(selected_rows)}
    column_index = {column: index for index, column in enumerate(selected_columns)}
    matrix = [[0 for _ in selected_columns] for _ in selected_rows]
    for entry in manifest["entries"]:
        row = entry["row"]
        column = entry["column"]
        if row not in row_index or column not in column_index:
            continue
        value = assignments[tuple(entry["variable"])] % prime
        if entry["sign"] < 0 and value:
            value = prime - value
        matrix[row_index[row]][column_index[column]] = value
    return matrix


def determinant_and_inverse_mod(
    matrix: list[list[int]], prime: int
) -> tuple[int, list[list[int]]]:
    size = len(matrix)
    if any(len(row) != size for row in matrix):
        raise ValueError("matrix is not square")
    augmented = [
        [entry % prime for entry in row]
        + [1 if row_index == column_index else 0 for column_index in range(size)]
        for row_index, row in enumerate(matrix)
    ]
    determinant = 1
    width = 2 * size
    for column in range(size):
        pivot = next(
            (row for row in range(column, size) if augmented[row][column] % prime),
            None,
        )
        if pivot is None:
            raise ValueError("singular matrix")
        if pivot != column:
            augmented[column], augmented[pivot] = augmented[pivot], augmented[column]
            determinant = (-determinant) % prime
        pivot_value = augmented[column][column] % prime
        determinant = (determinant * pivot_value) % prime
        pivot_inverse = pow(pivot_value, prime - 2, prime)
        for index in range(width):
            augmented[column][index] = (augmented[column][index] * pivot_inverse) % prime
        for row in range(size):
            if row == column:
                continue
            factor = augmented[row][column] % prime
            if factor == 0:
                continue
            for index in range(width):
                augmented[row][index] = (
                    augmented[row][index] - factor * augmented[column][index]
                ) % prime
    inverse = [row[size:] for row in augmented]
    return determinant, inverse


def determinant_mod(matrix: list[list[int]], prime: int) -> int:
    determinant, _ = determinant_and_inverse_mod(matrix, prime)
    return determinant


def canonical_variable(variable: tuple[int, int, int]) -> tuple[int, int, int]:
    r, s, t = variable
    return (r, min(s, t), max(s, t))


def variable_action(
    variable: tuple[int, int, int],
    root: tuple[int, int],
    variable_set: set[tuple[int, int, int]],
) -> dict[tuple[int, int, int], int]:
    """Infinitesimal action on the coefficient variables.

    The tested roots stay inside the coefficient subspace A, so a nonzero
    derivative here is a conclusive non-highest-weight certificate.
    """

    u, v = root
    r, s, t = variable
    result: dict[tuple[int, int, int], int] = {}

    def add_target(raw: tuple[int, int, int], coefficient: int) -> None:
        target = canonical_variable(raw)
        if target not in variable_set:
            return
        result[target] = result.get(target, 0) + coefficient
        if result[target] == 0:
            del result[target]

    if r == u:
        add_target((v, s, t), -1)
    if s == v:
        add_target((r, u, t), 1)
    if t == v:
        add_target((r, s, u), 1)
    return result


def derivative_matrix(
    manifest: dict,
    assignments: dict[tuple[int, int, int], int],
    prime: int,
    selected_columns: list[int],
    root: tuple[int, int],
) -> list[list[int]]:
    variables = {tuple(variable) for variable in manifest["coefficient_variables"]}
    column_index = {column: index for index, column in enumerate(selected_columns)}
    matrix = [[0 for _ in selected_columns] for _ in range(90)]
    for entry in manifest["entries"]:
        column = entry["column"]
        if column not in column_index:
            continue
        derivative = 0
        for target, coefficient in variable_action(
            tuple(entry["variable"]), root, variables
        ).items():
            derivative += coefficient * assignments[target]
        derivative %= prime
        if entry["sign"] < 0 and derivative:
            derivative = prime - derivative
        matrix[entry["row"]][column_index[column]] = derivative
    return matrix


def trace_product_mod(
    left: list[list[int]], right: list[list[int]], prime: int
) -> int:
    size = len(left)
    total = 0
    for i in range(size):
        for k in range(size):
            total = (total + left[i][k] * right[k][i]) % prime
    return total


def perfect_matching(graph: list[list[int]], column_count: int) -> list[int]:
    matched_row = [-1] * column_count

    def augment(row: int, seen: set[int]) -> bool:
        for column in graph[row]:
            if column in seen:
                continue
            seen.add(column)
            if matched_row[column] == -1 or augment(matched_row[column], seen):
                matched_row[column] = row
                return True
        return False

    for row in range(len(graph)):
        if not augment(row, set()):
            raise RuntimeError(f"no perfect matching at row {row}")
    row_to_column = [-1] * len(graph)
    for column, row in enumerate(matched_row):
        if row != -1:
            row_to_column[row] = column
    assert all(column != -1 for column in row_to_column)
    return row_to_column


def write_json(path: Path, report: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def write_pivot_report(manifest: dict, certificate: dict) -> None:
    pivots = certificate["pivot_columns"]
    selected_index = {column: index for index, column in enumerate(pivots)}

    entries: dict[tuple[int, int], tuple[int, ...]] = {}
    graph = [[] for _ in range(90)]
    for entry in manifest["entries"]:
        if entry["column"] not in selected_index:
            continue
        row = entry["row"]
        column = selected_index[entry["column"]]
        weight = variable_weight(tuple(entry["variable"]))
        entries[(row, column)] = weight
        graph[row].append(column)

    matching = perfect_matching(graph, 90)
    total = (0,) * 8
    for row, column in enumerate(matching):
        total = add(total, entries[(row, column)])

    row_weight_sum = (0,) * 8
    for row in manifest["rows"]:
        row_weight_sum = add(row_weight_sum, relation_weight(tuple(row)))
    selected_column_weight_sum = (0,) * 8
    for column in pivots:
        selected_column_weight_sum = add(
            selected_column_weight_sum,
            variable_weight(tuple(manifest["columns"][column])),
        )
    assert subtract(row_weight_sum, selected_column_weight_sum) == total
    # Check additive row/column potentials. This proves that every nonzero
    # determinant monomial has the same torus weight, not just the matching
    # selected above.
    row_potential: dict[int, tuple[int, ...]] = {}
    column_potential: dict[int, tuple[int, ...]] = {}
    unseen_rows = set(range(90))
    while unseen_rows:
        start = min(unseen_rows)
        row_potential[start] = (0,) * 8
        pending: list[tuple[str, int]] = [("row", start)]
        while pending:
            side, index = pending.pop()
            if side == "row":
                unseen_rows.discard(index)
                for column in graph[index]:
                    proposed = subtract(entries[(index, column)], row_potential[index])
                    if column in column_potential:
                        assert column_potential[column] == proposed
                    else:
                        column_potential[column] = proposed
                        pending.append(("column", column))
            else:
                for row in range(90):
                    if (row, index) not in entries:
                        continue
                    proposed = subtract(entries[(row, index)], column_potential[index])
                    if row in row_potential:
                        assert row_potential[row] == proposed
                    else:
                        row_potential[row] = proposed
                        pending.append(("row", row))

    dominant = tuple(sorted(total, reverse=True))
    compatible = dominated_by(dominant, EXPECTED)
    report = {
                "status": "verified",
                "minor_weight": total,
                "claimed_highest_weight": EXPECTED,
                "dominant_reordering": dominant,
                "compatible_with_claimed_irrep": compatible,
                "perfect_matching_size": len(matching),
                "weight_homogeneous": True,
                "row_weight_sum": row_weight_sum,
                "selected_column_weight_sum": selected_column_weight_sum,
                "selected_column_sum_for_oriented_claim": subtract(
                    row_weight_sum, ORIENTED_EXPECTED
                ),
                "consequence": (
                    "This certified nonzero minor cannot itself lie in the claimed "
                    "irreducible module. Therefore the paper's literal assertion that "
                    "every maximal minor lies there is false; existence of another "
                    "minor or projection remains a separate question."
                ),
            }
    assert not compatible
    write_json(OUTPUT, report)
    print(json.dumps(report, sort_keys=True))


def write_89_minor_report(manifest: dict, certificate: dict, predecessors: dict) -> None:
    prime = int(certificate["prime"])
    assignments = {
        tuple(record["variable"]): int(record["value"])
        for record in certificate["assignments"]
    }
    selected_rows = list(range(1, 90))
    selected_columns = certificate["pivot_columns"][1:]
    matrix = selected_matrix(manifest, assignments, prime, selected_rows, selected_columns)
    determinant = determinant_mod(matrix, prime)
    weight = selected_minor_weight(manifest, selected_rows, selected_columns)
    dominant = tuple(sorted(weight, reverse=True))
    predecessor_weights = [
        tuple(candidate["lambda"]) for candidate in predecessors["tensor_candidates"]
    ]
    compatible = [
        weight for weight in predecessor_weights if dominated_by(dominant, weight)
    ]
    report = {
        "status": "verified",
        "source_minor": "delete row 0 and pivot-column position 0 from the certified full-rank 90 x 90 pivot minor",
        "prime": prime,
        "deleted_row": 0,
        "deleted_pivot_column_position": 0,
        "deleted_global_column": certificate["pivot_columns"][0],
        "selected_rows": selected_rows,
        "selected_columns": selected_columns,
        "determinant_mod_prime": determinant,
        "minor_weight": weight,
        "dominant_reordering": dominant,
        "degree": 89,
        "candidate_predecessor_count": len(predecessor_weights),
        "max_predecessor_first_part": max(weight[0] for weight in predecessor_weights),
        "compatible_predecessor_count": len(compatible),
        "compatible_predecessors": compatible,
        "consequence": (
            "This nonzero 89 x 89 minor is not a weight of any of the 15 "
            "degree-89 predecessor modules. Thus the printed sentence that "
            "each 89 x 89 minor belongs to one of those modules is false as "
            "written."
        ),
    }
    assert determinant == 421057
    assert compatible == []
    write_json(OUTPUT_89, report)
    print(json.dumps(report, sort_keys=True))


def write_raising_report(manifest: dict, certificate: dict) -> None:
    prime = int(certificate["prime"])
    assignments = {
        tuple(record["variable"]): int(record["value"])
        for record in certificate["assignments"]
    }
    columns = certificate["claimed_weight_candidate"]["columns"]
    matrix = selected_matrix(manifest, assignments, prime, list(range(90)), columns)
    determinant, inverse = determinant_and_inverse_mod(matrix, prime)
    assert determinant == certificate["claimed_weight_candidate"]["determinant_mod_prime"]
    values = []
    for root in TESTED_SIMPLE_ROOTS:
        derivative = derivative_matrix(manifest, assignments, prime, columns, root)
        value = (determinant * trace_product_mod(inverse, derivative, prime)) % prime
        values.append(
            {
                "root": root,
                "derivative_mod_prime": value,
                "nonzero": value != 0,
            }
        )
    nonzero_roots = [record["root"] for record in values if record["nonzero"]]
    report = {
        "status": "verified",
        "prime": prime,
        "columns": columns,
        "determinant_mod_prime": determinant,
        "minor_weight": tuple(certificate["claimed_weight_candidate"]["weight"]),
        "dominant_reordering": EXPECTED,
        "tested_borel_order": TESTED_BOREL_ORDER,
        "tested_simple_roots": values,
        "all_tested_roots_annihilate": len(nonzero_roots) == 0,
        "nonzero_derivative_roots": nonzero_roots,
        "consequence": (
            "The selected weight-compatible determinant is not itself a "
            "highest-weight vector for this Borel order. This does not rule "
            "out a nonzero highest-weight linear combination or projection in "
            "the span of all such minors."
        ),
    }
    assert nonzero_roots
    write_json(OUTPUT_RAISING, report)
    print(json.dumps(report, sort_keys=True))


def main() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    certificate = json.loads(CERTIFICATE.read_text(encoding="utf-8"))
    predecessors = json.loads(PREDECESSORS.read_text(encoding="utf-8"))
    write_pivot_report(manifest, certificate)
    write_89_minor_report(manifest, certificate, predecessors)
    write_raising_report(manifest, certificate)


if __name__ == "__main__":
    main()
