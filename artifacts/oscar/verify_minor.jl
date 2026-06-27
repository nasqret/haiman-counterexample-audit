#!/usr/bin/env julia

"""Independent Oscar.jl verifier for the nonzero maximal minor in Lemma 19.

The program regenerates the matrix from Lee's six row families and centered
coordinate formula. Only the certificate's 45 assignments and 90 pivot
columns are embedded below; no SageMath matrix entries are imported.
"""

using Oscar

const PRIME = 1_000_003
const ASSIGNMENTS = [
    862231, 902114, 920459, 686557, 444566,
    519439, 378921, 874935, 467970, 468016,
    584953, 370016, 309141, 405193, 969369,
    552990, 606144, 903829, 482783, 449854,
    848842, 588001, 920852, 92590, 620434,
    353822, 501592, 709658, 69440, 969953,
    150322, 731728, 390750, 10887, 336086,
    804335, 385418, 443899, 583240, 793944,
    743287, 43814, 574954, 720916, 178974,
]
const PIVOTS_ZERO_BASED = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9,
    10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
    30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 59, 60, 61,
    62, 63, 64, 65, 66, 67, 68, 69, 70, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
    87, 88, 89, 90, 91, 92, 93, 94, 95, 101,
]

coord(r::Int, s::Int, t::Int) = (r, min(s, t), max(s, t))

function coefficient_variables()
    [(r, s, t) for r in 1:3 for s in 4:8 for t in s:8]
end

function row_manifest()
    rows = NTuple{4, Int}[]
    for a in 1:3
        for j in 4:6, i in (j + 1):7, k in (i + 1):8
            push!(rows, (a, j, i, k))
        end
        for k in 4:6, j in (k + 1):7, i in (j + 1):8
            push!(rows, (a, j, i, k))
        end
        for k in 5:8
            push!(rows, (a, 4, 4, k))
        end
        for j in 5:8
            push!(rows, (a, j, 4, j))
        end
        push!(rows, (a, 5, 5, 6))
        push!(rows, (a, 6, 5, 6))
    end
    @assert length(rows) == length(Set(rows)) == 90
    rows
end

function column_manifest()
    columns = NTuple{3, Int}[
        (r, s, t) for r in 1:3 for s in 1:3 for t in 4:8
    ]
    append!(columns, [
        (r, s, t)
        for r in 4:8 for s in 4:8 for t in s:8
        if !(r == s == t)
    ])
    @assert length(columns) == length(Set(columns)) == 115
    columns
end

function derive_specialization()
    variables = coefficient_variables()
    rows = row_manifest()
    columns = column_manifest()
    @assert length(variables) == length(ASSIGNMENTS) == 45
    @assert length(PIVOTS_ZERO_BASED) == 90
    variable_index = Dict(value => index for (index, value) in enumerate(variables))
    column_index = Dict(value => index for (index, value) in enumerate(columns))

    entries = zeros(Int, 90, 115)
    translation_residual = zeros(Int, 90, 8 * 45)

    for (row_index, (a, j, i, k)) in enumerate(rows)
        for m in 1:8
            terms = (
                (1, coord(m, i, j), coord(a, k, m)),
                (-1, coord(m, k, j), coord(a, i, m)),
            )
            for (sign, left, right) in terms
                left_index = get(variable_index, left, 0)
                right_index = get(variable_index, right, 0)
                @assert (left_index > 0) != (right_index > 0)
                variable = left_index > 0 ? left_index : right_index
                complement = left_index > 0 ? right : left
                r, s, t = complement

                if r == s == t
                    translation_residual[row_index, (r - 1) * 45 + variable] += 2 * sign
                    continue
                end

                column = get(column_index, complement, 0)
                @assert column > 0
                entries[row_index, column] = mod(
                    entries[row_index, column] + sign * ASSIGNMENTS[variable],
                    PRIME,
                )
                if r == t
                    translation_residual[row_index, (s - 1) * 45 + variable] += sign
                end
                if r == s
                    translation_residual[row_index, (t - 1) * 45 + variable] += sign
                end
            end
        end
    end

    @assert iszero(translation_residual)
    @assert count(!iszero, entries) == 1410
    entries
end

function main()
    entries = derive_specialization()
    field = GF(PRIME)
    matrix_entries = [entries[i, j] for i in 1:90 for j in 1:115]
    specialized = matrix(field, 90, 115, matrix_entries)
    minor_entries = [
        specialized[i, PIVOTS_ZERO_BASED[j] + 1]
        for i in 1:90 for j in 1:90
    ]
    selected_minor = matrix(field, 90, 90, minor_entries)
    determinant_residue = det(selected_minor)

    @assert rank(specialized) == 90
    @assert determinant_residue == field(970351)

    println("Julia version: ", VERSION)
    println("Oscar version: ", pkgversion(Oscar))
    Oscar.versioninfo()
    println(
        "{\"implementation\":\"Oscar.jl\",\"shape\":[90,115]," *
        "\"nonzero_entries\":1410,\"prime\":1000003,\"rank\":90," *
        "\"determinant_mod_prime\":970351," *
        "\"translation_residual_zero\":true,\"status\":\"verified\"}",
    )
end

main()
