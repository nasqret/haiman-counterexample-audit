#!/usr/bin/env python3
"""Portable standard-library verifier used for the WMI batch cross-check.

This is not an additional source of matrix data: it regenerates the sparse
matrix from Lee's printed formula and reads only the assignment/pivot
certificate.  Its purpose is to make the cluster run independent of local CAS
installations.
"""

from __future__ import annotations

from hashlib import sha256
from itertools import combinations
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CERTIFICATE = ROOT / "results/certificates/lemma19_nonzero_minor.json"


def coord(r: int, s: int, t: int) -> tuple[int, int, int]:
    return r, min(s, t), max(s, t)


def variables() -> list[tuple[int, int, int]]:
    return [(r, s, t) for r in range(1, 4) for s in range(4, 9) for t in range(s, 9)]


def rows() -> list[tuple[int, int, int, int]]:
    result: list[tuple[int, int, int, int]] = []
    for a in range(1, 4):
        result.extend((a, j, i, k) for j, i, k in combinations(range(4, 9), 3))
        result.extend((a, j, i, k) for k, j, i in combinations(range(4, 9), 3))
        result.extend((a, 4, 4, k) for k in range(5, 9))
        result.extend((a, j, 4, j) for j in range(5, 9))
        result.append((a, 5, 5, 6))
        result.append((a, 6, 5, 6))
    assert len(result) == len(set(result)) == 90
    return result


def columns() -> list[tuple[int, int, int]]:
    result = [(r, s, t) for r in range(1, 4) for s in range(1, 4) for t in range(4, 9)]
    result.extend(
        (r, s, t)
        for r in range(4, 9)
        for s in range(4, 9)
        for t in range(s, 9)
        if not r == s == t
    )
    assert len(result) == len(set(result)) == 115
    return result


def add(mapping: dict, key, value: int) -> None:
    new_value = mapping.get(key, 0) + value
    if new_value:
        mapping[key] = new_value
    else:
        mapping.pop(key, None)


def centered_terms(value: tuple[int, int, int]):
    r, s, t = value
    if r == s == t:
        return [("T", r, 2)]
    result = [("z", value, 1)]
    if r == t:
        result.append(("T", s, 1))
    if r == s:
        result.append(("T", t, 1))
    return result


def derive():
    variable_manifest = variables()
    row_manifest = rows()
    column_manifest = columns()
    variable_set = set(variable_manifest)
    column_index = {value: index for index, value in enumerate(column_manifest)}
    entries: dict[tuple[int, int], tuple[int, tuple[int, int, int]]] = {}
    translation_residual: dict = {}

    for row_index, (a, j, i, k) in enumerate(row_manifest):
        collected: dict = {}
        for m in range(1, 9):
            terms = (
                (1, coord(m, i, j), coord(a, k, m)),
                (-1, coord(m, k, j), coord(a, i, m)),
            )
            for sign, left, right in terms:
                assert (left in variable_set) != (right in variable_set)
                variable = left if left in variable_set else right
                complement = right if left in variable_set else left
                for kind, centered, factor in centered_terms(complement):
                    add(collected, (kind, centered, variable), sign * factor)

        for (kind, centered, variable), sign in collected.items():
            if kind == "T":
                add(translation_residual, (row_index, centered, variable), sign)
            else:
                key = row_index, column_index[centered]
                assert key not in entries
                assert sign in (-1, 1)
                entries[key] = sign, variable

    assert not translation_residual
    assert len(entries) == 1410
    return variable_manifest, row_manifest, column_manifest, entries


def payload_digest(variable_manifest, row_manifest, column_manifest, entries) -> str:
    records = [
        {
            "row": row,
            "column": column,
            "sign": sign,
            "variable": list(variable),
        }
        for (row, column), (sign, variable) in sorted(entries.items())
    ]
    payload = {
        "indexing": {"tuples": "one_based", "matrix_positions": "zero_based"},
        "coefficient_variables": [list(value) for value in variable_manifest],
        "rows": [list(value) for value in row_manifest],
        "columns": [list(value) for value in column_manifest],
        "entries": records,
    }
    encoded = json.dumps(payload, sort_keys=True, separators=(",", ":"), ensure_ascii=True)
    return sha256(encoded.encode("ascii")).hexdigest()


def determinant_mod(matrix: list[list[int]], prime: int) -> int:
    values = [row[:] for row in matrix]
    determinant = 1
    size = len(values)
    for column in range(size):
        pivot = next((row for row in range(column, size) if values[row][column] % prime), None)
        if pivot is None:
            return 0
        if pivot != column:
            values[column], values[pivot] = values[pivot], values[column]
            determinant = -determinant
        pivot_value = values[column][column] % prime
        determinant = determinant * pivot_value % prime
        inverse = pow(pivot_value, -1, prime)
        for row in range(column + 1, size):
            factor = values[row][column] * inverse % prime
            if factor:
                for entry_column in range(column, size):
                    values[row][entry_column] = (
                        values[row][entry_column] - factor * values[column][entry_column]
                    ) % prime
    return determinant % prime


def main() -> None:
    certificate = json.loads(CERTIFICATE.read_text(encoding="utf-8"))
    variable_manifest, row_manifest, column_manifest, entries = derive()
    digest = payload_digest(variable_manifest, row_manifest, column_manifest, entries)
    assert digest == certificate["matrix_payload_sha256"]

    prime = certificate["prime"]
    assignments = {
        tuple(record["variable"]): record["value"] for record in certificate["assignments"]
    }
    specialized = [[0] * 115 for _ in range(90)]
    for (row, column), (sign, variable) in entries.items():
        specialized[row][column] = sign * assignments[variable] % prime
    selected = [
        [specialized[row][column] for column in certificate["pivot_columns"]]
        for row in range(90)
    ]
    determinant = determinant_mod(selected, prime)
    assert determinant == certificate["determinant_mod_prime"] == 970351
    print(
        json.dumps(
            {
                "implementation": "Python-stdlib-WMI",
                "matrix_payload_sha256": digest,
                "shape": [90, 115],
                "nonzero_entries": len(entries),
                "prime": prime,
                "rank": 90,
                "determinant_mod_prime": determinant,
                "translation_residual_zero": True,
                "status": "verified",
            },
            sort_keys=True,
        )
    )


if __name__ == "__main__":
    main()
