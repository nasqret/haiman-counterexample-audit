#!/usr/bin/env python3
"""Analyze the torus weight of the certified maximal minor."""

from __future__ import annotations

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "results/certificates/lemma19_matrix_manifest.json"
CERTIFICATE = ROOT / "results/certificates/lemma19_nonzero_minor.json"
OUTPUT = ROOT / "results/certificates/lemma19_pivot_weight_audit.json"
EXPECTED = (133, 130, 126, 122, 119, 60, 60, 60)


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


def main() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    certificate = json.loads(CERTIFICATE.read_text(encoding="utf-8"))
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
    oriented_claim = (60, 60, 60, 133, 130, 126, 122, 119)
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
                    row_weight_sum, oriented_claim
                ),
                "consequence": (
                    "This certified nonzero minor cannot itself lie in the claimed "
                    "irreducible module. Therefore the paper's literal assertion that "
                    "every maximal minor lies there is false; existence of another "
                    "minor or projection remains a separate question."
                ),
            }
    assert not compatible
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT.write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    print(json.dumps(report, sort_keys=True))


if __name__ == "__main__":
    main()
