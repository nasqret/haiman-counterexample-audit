#!/usr/bin/env sage
"""Reconstruct Lee's 90 x 115 matrix and emit a nonvanishing certificate.

The matrix is derived from the printed quadratic relation and the p-to-q
centering formula. No matrix entry is hard-coded.
"""

from itertools import combinations
from pathlib import Path
import argparse
import hashlib
import json
import random

from sage.all import GF, MixedIntegerLinearProgram, matrix


D = 8


def sym_coordinate(r, s, t):
    """Canonical p/q coordinate tuple, using symmetry in the last two slots."""
    return (r, min(s, t), max(s, t))


def coefficient_variables():
    return [
        (r, s, t)
        for r in range(1, 4)
        for s in range(4, 9)
        for t in range(s, 9)
    ]


def row_manifest():
    rows = []
    for a in range(1, 4):
        families = [
            [(a, j, i, k) for j, i, k in combinations(range(4, 9), 3)],
            [(a, j, i, k) for k, j, i in combinations(range(4, 9), 3)],
            [(a, 4, 4, k) for k in range(5, 9)],
            [(a, j, 4, j) for j in range(5, 9)],
            [(a, 5, 5, 6)],
            [(a, 6, 5, 6)],
        ]
        for family in families:
            rows.extend(family)
    assert len(rows) == len(set(rows)) == 90
    return rows


def column_manifest():
    first = [
        (r, s, t)
        for r in range(1, 4)
        for s in range(1, 4)
        for t in range(4, 9)
    ]
    second = [
        (r, s, t)
        for r in range(4, 9)
        for s in range(4, 9)
        for t in range(s, 9)
        if not (r == s == t)
    ]
    columns = first + second
    assert len(first) == 45
    assert len(second) == 70
    assert len(columns) == len(set(columns)) == 115
    return columns


def p_in_centered_coordinates(r, s, t):
    """Return p_(r,st) as a sparse linear expression in centered z and T.

    T_s denotes q_(s,ss), the translation summand in Theorem 16.
    z uses Lee's centered coordinate from p. 1359.
    """
    r, s, t = sym_coordinate(r, s, t)
    if r == s == t:
        return {("T", r): 2}
    if r == t:
        return {("z", (r, s, t)): 1, ("T", s): 1}
    if r == s:
        return {("z", (r, s, t)): 1, ("T", t): 1}
    return {("z", (r, s, t)): 1}


def relation_terms(a, j, i, k):
    """Terms of C(a;j,(i,k)) as (sign, first_p, second_p)."""
    terms = []
    for m in range(1, D + 1):
        terms.append((1, sym_coordinate(m, i, j), sym_coordinate(a, k, m)))
        terms.append((-1, sym_coordinate(m, k, j), sym_coordinate(a, i, m)))
    return terms


def add_to(mapping, key, value):
    mapping[key] = mapping.get(key, 0) + value
    if mapping[key] == 0:
        del mapping[key]


def derive_sparse_matrix():
    variables = coefficient_variables()
    variable_set = set(variables)
    rows = row_manifest()
    columns = column_manifest()
    column_index = {coordinate: index for index, coordinate in enumerate(columns)}

    entries = {}
    translation_residual = {}

    for row_index, row in enumerate(rows):
        a, j, i, k = row
        collected = {}
        for sign, left, right in relation_terms(a, j, i, k):
            left_is_a = left in variable_set
            right_is_a = right in variable_set
            assert left_is_a != right_is_a, (
                "Each selected monomial must contain exactly one coefficient variable",
                row,
                left,
                right,
            )
            coefficient_variable = left if left_is_a else right
            complement = right if left_is_a else left
            for centered_variable, centered_coefficient in p_in_centered_coordinates(*complement).items():
                key = (centered_variable, coefficient_variable)
                add_to(collected, key, sign * centered_coefficient)

        for (centered_variable, coefficient_variable), coefficient in collected.items():
            kind, coordinate = centered_variable
            if kind == "T":
                add_to(
                    translation_residual,
                    (row_index, coordinate, coefficient_variable),
                    coefficient,
                )
                continue
            assert coordinate in column_index, (row, coordinate)
            col_index = column_index[coordinate]
            entry_key = (row_index, col_index)
            current = entries.setdefault(entry_key, {})
            add_to(current, coefficient_variable, coefficient)

    assert translation_residual == {}, translation_residual
    for key, linear_form in entries.items():
        assert len(linear_form) == 1, (key, linear_form)
        coefficient = next(iter(linear_form.values()))
        assert coefficient in (-1, 1), (key, linear_form)

    return variables, rows, columns, entries


def sparse_records(entries):
    records = []
    for (row, column), linear_form in sorted(entries.items()):
        variable, sign = next(iter(linear_form.items()))
        records.append(
            {
                "row": row,
                "column": column,
                "sign": sign,
                "variable": list(variable),
            }
        )
    return records


def canonical_payload(variables, rows, columns, entries):
    return {
        "indexing": {
            "tuples": "one_based",
            "matrix_positions": "zero_based",
        },
        "coefficient_variables": [list(value) for value in variables],
        "rows": [list(value) for value in rows],
        "columns": [list(value) for value in columns],
        "entries": sparse_records(entries),
    }


def canonical_json(value):
    return json.dumps(
        value,
        sort_keys=True,
        separators=(",", ":"),
        ensure_ascii=True,
        default=int,
    )


def specialize(entries, assignments, prime, nrows, ncols):
    values = [[0 for _ in range(ncols)] for _ in range(nrows)]
    for (row, column), linear_form in entries.items():
        variable, sign = next(iter(linear_form.items()))
        values[row][column] = (sign * assignments[variable]) % prime
    return matrix(GF(prime), values)


def variable_weight(variable):
    r, s, t = variable
    weight = [1] * 8
    weight[r - 1] -= 1
    weight[s - 1] += 1
    weight[t - 1] += 1
    return weight


def relation_weight(row):
    a, j, i, k = row
    weight = [2] * 8
    weight[a - 1] -= 1
    weight[i - 1] += 1
    weight[j - 1] += 1
    weight[k - 1] += 1
    return weight


def find_claimed_weight_candidate(specialized, rows, columns, max_trials=200):
    """Find a nonzero minor with a Weyl permutation of Lee's claimed weight.

    Weight compatibility is necessary but does not prove that the determinant
    lies purely in the claimed irreducible Schur module.
    """
    oriented_weight = [60, 60, 60, 133, 130, 126, 122, 119]
    row_sum = [0] * 8
    for row in rows:
        weight = relation_weight(row)
        row_sum = [row_sum[index] + weight[index] for index in range(8)]
    target_column_sum = [
        row_sum[index] - oriented_weight[index]
        for index in range(8)
    ]
    column_weights = [variable_weight(column) for column in columns]

    program = MixedIntegerLinearProgram(maximization=False)
    selected = program.new_variable(binary=True)
    program.add_constraint(sum(selected[column] for column in range(115)) == 90)
    for coordinate in range(8):
        program.add_constraint(
            sum(
                column_weights[column][coordinate] * selected[column]
                for column in range(115)
            )
            == target_column_sum[coordinate]
        )
    program.set_objective(0)

    for trial in range(max_trials):
        program.solve()
        values = program.get_values(selected)
        chosen = [column for column in range(115) if values[column] > 0.5]
        assert len(chosen) == 90
        minor = specialized.matrix_from_columns(chosen)
        if minor.rank() == 90:
            return {
                "columns": chosen,
                "weight": oriented_weight,
                "trial": trial,
                "determinant_mod_prime": int(minor.determinant()),
                "evidence_boundary": (
                    "The determinant is nonzero and has a Weyl permutation of "
                    "the claimed highest weight. Highest-weight annihilation and "
                    "pure Schur-module membership are not yet certified."
                ),
            }
        program.add_constraint(sum(selected[column] for column in chosen) <= 89)
    raise RuntimeError("No full-rank claimed-weight minor found")


def find_certificate(entries, variables, prime, seed, max_trials):
    prime = int(prime)
    seed = int(seed)
    max_trials = int(max_trials)
    rng = random.Random(seed)
    for trial in range(max_trials):
        assignments = {
            variable: rng.randrange(1, prime)
            for variable in variables
        }
        specialized = specialize(entries, assignments, prime, 90, 115)
        rank = specialized.rank()
        if rank != 90:
            continue
        pivots = list(specialized.echelon_form().pivots())
        assert len(pivots) == 90
        determinant = specialized.matrix_from_columns(pivots).determinant()
        assert determinant != 0
        return {
            "prime": prime,
            "seed": seed,
            "trial": trial,
            "rng": "Python random.Random (MT19937); assignments are authoritative",
            "assignments": [
                {
                    "variable": list(variable),
                    "value": int(assignments[variable]),
                }
                for variable in variables
            ],
            "pivot_columns": pivots,
            "determinant_mod_prime": int(determinant),
            "rank": rank,
        }
    raise RuntimeError("No full-rank specialization found")


def write_json(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(value, indent=2, sort_keys=True, default=int) + "\n",
        encoding="utf-8",
    )


def main():
    root = Path(__file__).resolve().parents[2]
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--manifest",
        type=Path,
        default=root / "results/certificates/lemma19_matrix_manifest.json",
    )
    parser.add_argument(
        "--certificate",
        type=Path,
        default=root / "results/certificates/lemma19_nonzero_minor.json",
    )
    parser.add_argument("--prime", type=int, default=1000003)
    parser.add_argument("--seed", type=int, default=20100701)
    parser.add_argument("--max-trials", type=int, default=100)
    args = parser.parse_args()

    variables, rows, columns, entries = derive_sparse_matrix()
    payload = canonical_payload(variables, rows, columns, entries)
    digest = hashlib.sha256(canonical_json(payload).encode("ascii")).hexdigest()
    manifest = {
        "schema_version": 1,
        "source": "Lee 2010, Lemma 19, pp. 1358-1359",
        "shape": [len(rows), len(columns)],
        "nonzero_entries": len(entries),
        "payload_sha256": digest,
        **payload,
    }
    write_json(args.manifest, manifest)

    certificate = find_certificate(
        entries,
        variables,
        args.prime,
        args.seed,
        args.max_trials,
    )
    assignment_map = {
        tuple(record["variable"]): record["value"]
        for record in certificate["assignments"]
    }
    specialized = specialize(entries, assignment_map, args.prime, 90, 115)
    certificate["claimed_weight_candidate"] = find_claimed_weight_candidate(
        specialized,
        rows,
        columns,
    )
    certificate.update(
        {
            "schema_version": 1,
            "matrix_payload_sha256": digest,
            "shape": [len(rows), len(columns)],
            "mathematical_implication": (
                "A nonzero determinant after integer specialization and reduction "
                "modulo p proves that the selected maximal-minor polynomial is "
                "nonzero over Z and hence over characteristic zero."
            ),
        }
    )
    write_json(args.certificate, certificate)

    print(
        canonical_json(
            {
                "status": "ok",
                "shape": [len(rows), len(columns)],
                "coefficient_variables": len(variables),
                "nonzero_entries": len(entries),
                "payload_sha256": digest,
                "rank": certificate["rank"],
                "prime": certificate["prime"],
                "trial": certificate["trial"],
                "determinant_mod_prime": certificate["determinant_mod_prime"],
            }
        )
    )


if __name__ == "__main__":
    main()
