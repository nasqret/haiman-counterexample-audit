use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

type Coord = [u8; 3];
type Row = [u8; 4];

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Centered {
    Translation(u8),
    Z(Coord),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct EntryKey {
    row: usize,
    column: usize,
}

#[derive(Clone, Debug, Serialize)]
struct EntryRecord {
    row: usize,
    column: usize,
    sign: i32,
    variable: Coord,
}

#[derive(Debug, Deserialize)]
struct AssignmentRecord {
    variable: Coord,
    value: u64,
}

#[derive(Debug, Deserialize)]
struct ClaimedWeightCandidate {
    columns: Vec<usize>,
    weight: [i64; 8],
    determinant_mod_prime: u64,
}

#[derive(Debug, Deserialize)]
struct Certificate {
    schema_version: u64,
    matrix_payload_sha256: String,
    shape: [usize; 2],
    prime: u64,
    assignments: Vec<AssignmentRecord>,
    pivot_columns: Vec<usize>,
    determinant_mod_prime: u64,
    rank: usize,
    claimed_weight_candidate: ClaimedWeightCandidate,
}

#[derive(Debug, Deserialize)]
struct TensorLrChain {
    partitions_from_power_1_to_89: Vec<[i64; 8]>,
    step_lr_coefficients: Vec<u64>,
}

#[derive(Debug, Deserialize)]
struct TensorCandidate {
    lambda: [i64; 8],
    delta: [i64; 8],
    littlewood_richardson_coefficient: u64,
    dominance_slacks: [i64; 8],
    tensor_lr_chain: TensorLrChain,
}

#[derive(Debug, Deserialize)]
struct PredecessorCertificate {
    schema_version: u64,
    payload_sha256: String,
    mu: [i64; 8],
    nu: [i64; 8],
    tensor_power: i64,
    total_lr_candidates: usize,
    total_tensor_candidates: usize,
    tensor_candidates: Vec<TensorCandidate>,
}

fn coord(r: u8, s: u8, t: u8) -> Coord {
    [r, s.min(t), s.max(t)]
}

fn coefficient_variables() -> Vec<Coord> {
    let mut values = Vec::new();
    for r in 1..=3 {
        for s in 4..=8 {
            for t in s..=8 {
                values.push([r, s, t]);
            }
        }
    }
    assert_eq!(values.len(), 45);
    values
}

fn row_manifest() -> Vec<Row> {
    let mut rows = Vec::new();
    for a in 1..=3 {
        // 4 <= j < i < k <= 8.
        for j in 4..=8 {
            for i in (j + 1)..=8 {
                for k in (i + 1)..=8 {
                    rows.push([a, j, i, k]);
                }
            }
        }
        // 4 <= k < j < i <= 8.
        for k in 4..=8 {
            for j in (k + 1)..=8 {
                for i in (j + 1)..=8 {
                    rows.push([a, j, i, k]);
                }
            }
        }
        // Four boundary families printed by Lee.
        for k in 5..=8 {
            rows.push([a, 4, 4, k]);
        }
        for j in 5..=8 {
            rows.push([a, j, 4, j]);
        }
        rows.push([a, 5, 5, 6]);
        rows.push([a, 6, 5, 6]);
    }
    assert_eq!(rows.len(), 90);
    assert_eq!(rows.iter().copied().collect::<BTreeSet<_>>().len(), 90);
    rows
}

fn column_manifest() -> Vec<Coord> {
    let mut columns = Vec::new();
    for r in 1..=3 {
        for s in 1..=3 {
            for t in 4..=8 {
                columns.push([r, s, t]);
            }
        }
    }
    assert_eq!(columns.len(), 45);
    for r in 4..=8 {
        for s in 4..=8 {
            for t in s..=8 {
                if r == s && s == t {
                    continue;
                }
                columns.push([r, s, t]);
            }
        }
    }
    assert_eq!(columns.len(), 115);
    assert_eq!(columns.iter().copied().collect::<BTreeSet<_>>().len(), 115);
    columns
}

fn p_in_centered_coordinates(c: Coord) -> Vec<(Centered, i32)> {
    let [r, s, t] = c;
    if r == s && s == t {
        vec![(Centered::Translation(r), 2)]
    } else if r == t {
        vec![(Centered::Z(c), 1), (Centered::Translation(s), 1)]
    } else if r == s {
        vec![(Centered::Z(c), 1), (Centered::Translation(t), 1)]
    } else {
        vec![(Centered::Z(c), 1)]
    }
}

fn relation_terms([a, j, i, k]: Row) -> Vec<(i32, Coord, Coord)> {
    let mut terms = Vec::new();
    for m in 1..=8 {
        terms.push((1, coord(m, i, j), coord(a, k, m)));
        terms.push((-1, coord(m, k, j), coord(a, i, m)));
    }
    terms
}

fn add_coefficient<K: Ord + Clone>(map: &mut BTreeMap<K, i32>, key: K, value: i32) {
    let new_value = map.get(&key).copied().unwrap_or(0) + value;
    if new_value == 0 {
        map.remove(&key);
    } else {
        map.insert(key, new_value);
    }
}

fn derive_entries(
    variables: &[Coord],
    rows: &[Row],
    columns: &[Coord],
) -> Result<BTreeMap<EntryKey, (i32, Coord)>, Box<dyn Error>> {
    let variable_set: BTreeSet<Coord> = variables.iter().copied().collect();
    let column_index: BTreeMap<Coord, usize> = columns
        .iter()
        .copied()
        .enumerate()
        .map(|(index, value)| (value, index))
        .collect();
    let mut entries = BTreeMap::new();

    for (row_index, row) in rows.iter().copied().enumerate() {
        let mut collected: BTreeMap<(Centered, Coord), i32> = BTreeMap::new();
        for (sign, left, right) in relation_terms(row) {
            let left_is_variable = variable_set.contains(&left);
            let right_is_variable = variable_set.contains(&right);
            if left_is_variable == right_is_variable {
                return Err(format!(
                    "row {row:?}: expected exactly one coefficient variable in {left:?} * {right:?}"
                )
                .into());
            }
            let variable = if left_is_variable { left } else { right };
            let complement = if left_is_variable { right } else { left };
            for (centered, centered_coefficient) in p_in_centered_coordinates(complement) {
                add_coefficient(
                    &mut collected,
                    (centered, variable),
                    sign * centered_coefficient,
                );
            }
        }

        for ((centered, variable), sign) in collected {
            let coordinate = match centered {
                Centered::Translation(index) => {
                    return Err(format!(
                        "row {row:?}: uncancelled translation T_{index} with variable {variable:?} and coefficient {sign}"
                    )
                    .into())
                }
                Centered::Z(value) => value,
            };
            let column = *column_index.get(&coordinate).ok_or_else(|| {
                format!("row {row:?}: centered coordinate {coordinate:?} is not a printed column")
            })?;
            if sign != -1 && sign != 1 {
                return Err(format!(
                    "entry ({row_index},{column}) has forbidden coefficient {sign}"
                )
                .into());
            }
            let key = EntryKey {
                row: row_index,
                column,
            };
            if entries.insert(key.clone(), (sign, variable)).is_some() {
                return Err(format!("duplicate entry ({},{})", key.row, key.column).into());
            }
        }
    }
    Ok(entries)
}

fn entry_records(entries: &BTreeMap<EntryKey, (i32, Coord)>) -> Vec<EntryRecord> {
    entries
        .iter()
        .map(|(key, (sign, variable))| EntryRecord {
            row: key.row,
            column: key.column,
            sign: *sign,
            variable: *variable,
        })
        .collect()
}

fn variable_weight([r, s, t]: Coord) -> [i64; 8] {
    let mut weight = [1_i64; 8];
    weight[usize::from(r - 1)] -= 1;
    weight[usize::from(s - 1)] += 1;
    weight[usize::from(t - 1)] += 1;
    weight
}

fn relation_weight([a, j, i, k]: Row) -> [i64; 8] {
    let mut weight = [2_i64; 8];
    weight[usize::from(a - 1)] -= 1;
    weight[usize::from(i - 1)] += 1;
    weight[usize::from(j - 1)] += 1;
    weight[usize::from(k - 1)] += 1;
    weight
}

fn weight_add(left: [i64; 8], right: [i64; 8]) -> [i64; 8] {
    std::array::from_fn(|index| left[index] + right[index])
}

fn weight_subtract(left: [i64; 8], right: [i64; 8]) -> [i64; 8] {
    std::array::from_fn(|index| left[index] - right[index])
}

fn dominant_reordering(mut weight: [i64; 8]) -> [i64; 8] {
    weight.sort_by(|left, right| right.cmp(left));
    weight
}

fn is_dominated_by(weight: &[i64; 8], highest_weight: &[i64; 8]) -> bool {
    let mut left_prefix = 0;
    let mut right_prefix = 0;
    for index in 0..8 {
        left_prefix += weight[index];
        right_prefix += highest_weight[index];
        if left_prefix > right_prefix {
            return false;
        }
    }
    left_prefix == right_prefix
}

fn canonical_json(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => serde_json::to_string(value).expect("serialize string"),
        Value::Array(values) => format!(
            "[{}]",
            values
                .iter()
                .map(canonical_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
        Value::Object(values) => {
            let mut keys: Vec<&String> = values.keys().collect();
            keys.sort();
            let fields = keys
                .into_iter()
                .map(|key| {
                    format!(
                        "{}:{}",
                        serde_json::to_string(key).expect("serialize key"),
                        canonical_json(&values[key])
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            format!("{{{fields}}}")
        }
    }
}

fn payload(
    variables: &[Coord],
    rows: &[Row],
    columns: &[Coord],
    entries: &BTreeMap<EntryKey, (i32, Coord)>,
) -> Value {
    json!({
        "indexing": {
            "tuples": "one_based",
            "matrix_positions": "zero_based"
        },
        "coefficient_variables": variables,
        "rows": rows,
        "columns": columns,
        "entries": entry_records(entries),
    })
}

fn sha256_hex(text: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
fn mod_add(left: u64, right: u64, modulus: u64) -> u64 {
    ((left as u128 + right as u128) % modulus as u128) as u64
}

fn mod_sub(left: u64, right: u64, modulus: u64) -> u64 {
    ((left as u128 + modulus as u128 - right as u128) % modulus as u128) as u64
}

fn mod_mul(left: u64, right: u64, modulus: u64) -> u64 {
    ((left as u128 * right as u128) % modulus as u128) as u64
}

fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = mod_mul(result, base, modulus);
        }
        base = mod_mul(base, base, modulus);
        exponent >>= 1;
    }
    result
}

fn is_prime(value: u64) -> bool {
    if value < 2 {
        return false;
    }
    if value % 2 == 0 {
        return value == 2;
    }
    let mut divisor = 3;
    while divisor <= value / divisor {
        if value % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

fn determinant_mod(mut matrix: Vec<Vec<u64>>, prime: u64) -> Result<u64, Box<dyn Error>> {
    let n = matrix.len();
    if matrix.iter().any(|row| row.len() != n) {
        return Err("determinant matrix is not square".into());
    }
    let mut determinant = 1;
    for column in 0..n {
        let pivot = (column..n)
            .find(|row| matrix[*row][column] != 0)
            .ok_or("singular pivot minor")?;
        if pivot != column {
            matrix.swap(pivot, column);
            determinant = if determinant == 0 {
                0
            } else {
                prime - determinant
            };
        }
        let pivot_value = matrix[column][column];
        determinant = mod_mul(determinant, pivot_value, prime);
        let pivot_inverse = mod_pow(pivot_value, prime - 2, prime);
        for row in (column + 1)..n {
            if matrix[row][column] == 0 {
                continue;
            }
            let factor = mod_mul(matrix[row][column], pivot_inverse, prime);
            for index in column..n {
                let reduction = mod_mul(factor, matrix[column][index], prime);
                matrix[row][index] = mod_sub(matrix[row][index], reduction, prime);
            }
        }
    }
    Ok(determinant)
}

fn determinant_and_inverse_mod(
    matrix: Vec<Vec<u64>>,
    prime: u64,
) -> Result<(u64, Vec<Vec<u64>>), Box<dyn Error>> {
    let n = matrix.len();
    if matrix.iter().any(|row| row.len() != n) {
        return Err("inverse matrix is not square".into());
    }
    let mut augmented = vec![vec![0_u64; 2 * n]; n];
    for row in 0..n {
        for column in 0..n {
            augmented[row][column] = matrix[row][column] % prime;
        }
        augmented[row][n + row] = 1;
    }

    let mut determinant = 1;
    for column in 0..n {
        let pivot = (column..n)
            .find(|row| augmented[*row][column] != 0)
            .ok_or("singular matrix")?;
        if pivot != column {
            augmented.swap(pivot, column);
            determinant = if determinant == 0 {
                0
            } else {
                prime - determinant
            };
        }
        let pivot_value = augmented[column][column];
        determinant = mod_mul(determinant, pivot_value, prime);
        let pivot_inverse = mod_pow(pivot_value, prime - 2, prime);
        for index in 0..(2 * n) {
            augmented[column][index] = mod_mul(augmented[column][index], pivot_inverse, prime);
        }
        for row in 0..n {
            if row == column || augmented[row][column] == 0 {
                continue;
            }
            let factor = augmented[row][column];
            for index in 0..(2 * n) {
                let reduction = mod_mul(factor, augmented[column][index], prime);
                augmented[row][index] = mod_sub(augmented[row][index], reduction, prime);
            }
        }
    }

    let inverse = augmented.into_iter().map(|row| row[n..].to_vec()).collect();
    Ok((determinant, inverse))
}

fn submatrix(
    matrix: &[Vec<u64>],
    selected_rows: &[usize],
    selected_columns: &[usize],
) -> Vec<Vec<u64>> {
    selected_rows
        .iter()
        .map(|row| {
            selected_columns
                .iter()
                .map(|column| matrix[*row][*column])
                .collect()
        })
        .collect()
}

fn selected_minor_weight(
    rows: &[Row],
    columns: &[Coord],
    selected_rows: &[usize],
    selected_columns: &[usize],
) -> [i64; 8] {
    let row_weight_sum = selected_rows
        .iter()
        .map(|row| relation_weight(rows[*row]))
        .fold([0_i64; 8], weight_add);
    let column_weight_sum = selected_columns
        .iter()
        .map(|column| variable_weight(columns[*column]))
        .fold([0_i64; 8], weight_add);
    weight_subtract(row_weight_sum, column_weight_sum)
}

fn signed_mod(value: i64, prime: u64) -> u64 {
    if value >= 0 {
        (value as u64) % prime
    } else {
        let positive = ((-value) as u64) % prime;
        if positive == 0 {
            0
        } else {
            prime - positive
        }
    }
}

fn variable_action(
    [r, s, t]: Coord,
    (u, v): (u8, u8),
    variable_set: &BTreeSet<Coord>,
) -> Vec<(Coord, i64)> {
    let mut terms = BTreeMap::<Coord, i64>::new();
    let mut add_term = |target: Coord, coefficient: i64| {
        let canonical = coord(target[0], target[1], target[2]);
        if !variable_set.contains(&canonical) {
            return;
        }
        let value = terms.get(&canonical).copied().unwrap_or(0) + coefficient;
        if value == 0 {
            terms.remove(&canonical);
        } else {
            terms.insert(canonical, value);
        }
    };
    if r == u {
        add_term([v, s, t], -1);
    }
    if s == v {
        add_term([r, u, t], 1);
    }
    if t == v {
        add_term([r, s, u], 1);
    }
    terms.into_iter().collect()
}

fn trace_product_mod(left: &[Vec<u64>], right: &[Vec<u64>], prime: u64) -> u64 {
    let n = left.len();
    let mut total = 0;
    for i in 0..n {
        for k in 0..n {
            total = (total + mod_mul(left[i][k], right[k][i], prime)) % prime;
        }
    }
    total
}

fn derivative_value_mod(
    entries: &BTreeMap<EntryKey, (i32, Coord)>,
    variables: &BTreeSet<Coord>,
    assignments: &BTreeMap<Coord, u64>,
    selected_columns: &[usize],
    root: (u8, u8),
    determinant: u64,
    inverse: &[Vec<u64>],
    prime: u64,
) -> u64 {
    let column_index: BTreeMap<usize, usize> = selected_columns
        .iter()
        .copied()
        .enumerate()
        .map(|(index, column)| (column, index))
        .collect();
    let mut derivative = vec![vec![0_u64; selected_columns.len()]; 90];
    for (key, (sign, variable)) in entries {
        let Some(column) = column_index.get(&key.column) else {
            continue;
        };
        let mut value = 0;
        for (target, coefficient) in variable_action(*variable, root, variables) {
            let term = mod_mul(signed_mod(coefficient, prime), assignments[&target], prime);
            value = (value + term) % prime;
        }
        if *sign < 0 && value != 0 {
            value = prime - value;
        }
        derivative[key.row][*column] = value;
    }
    mod_mul(
        determinant,
        trace_product_mod(inverse, &derivative, prime),
        prime,
    )
}

fn partition_sum(partition: &[i64; 8]) -> i64 {
    partition.iter().sum()
}

fn is_partition(partition: &[i64; 8]) -> bool {
    partition.iter().all(|value| *value >= 0) && partition.windows(2).all(|pair| pair[0] >= pair[1])
}

fn lr_backtrack(
    position: usize,
    cells: &[(usize, i64)],
    content: &[u8; 8],
    used: &mut [u8; 8],
    values: &mut BTreeMap<(usize, i64), u8>,
) -> u64 {
    if position == cells.len() {
        return u64::from(used == content);
    }

    let (row, column) = cells[position];
    let right = values.get(&(row, column + 1)).copied();
    let above = row
        .checked_sub(1)
        .and_then(|previous_row| values.get(&(previous_row, column)).copied());
    let mut count = 0_u64;

    for value in 1..=8_u8 {
        let index = usize::from(value - 1);
        if used[index] >= content[index] {
            continue;
        }
        // Rows are weakly increasing from left to right. We fill right to left.
        if right.is_some_and(|right_value| value > right_value) {
            continue;
        }
        // Columns are strictly increasing from top to bottom.
        if above.is_some_and(|above_value| value <= above_value) {
            continue;
        }

        used[index] += 1;
        let lattice = (0..7).all(|slot| used[slot] >= used[slot + 1]);
        if lattice {
            values.insert((row, column), value);
            count += lr_backtrack(position + 1, cells, content, used, values);
            values.remove(&(row, column));
        }
        used[index] -= 1;
    }
    count
}

fn lr_coefficient(outer: &[i64; 8], inner: &[i64; 8], content: &[i64; 8]) -> u64 {
    if !is_partition(outer)
        || !is_partition(inner)
        || outer
            .iter()
            .zip(inner.iter())
            .any(|(outer_part, inner_part)| inner_part > outer_part)
        || partition_sum(outer) != partition_sum(inner) + partition_sum(content)
        || content
            .iter()
            .any(|value| *value < 0 || *value > i64::from(u8::MAX))
    {
        return 0;
    }

    let mut cells = Vec::new();
    for row in 0..8 {
        for column in ((inner[row] + 1)..=outer[row]).rev() {
            cells.push((row, column));
        }
    }
    if cells.len() != usize::try_from(partition_sum(content)).unwrap_or(usize::MAX) {
        return 0;
    }
    let content_u8: [u8; 8] = content.map(|value| u8::try_from(value).expect("small content"));
    lr_backtrack(0, &cells, &content_u8, &mut [0; 8], &mut BTreeMap::new())
}

fn dominance_slacks(partition: &[i64; 8], mu: &[i64; 8], power: i64) -> [i64; 8] {
    let mut result = [0_i64; 8];
    let mut partition_prefix = 0;
    let mut mu_prefix = 0;
    for index in 0..8 {
        partition_prefix += partition[index];
        mu_prefix += mu[index];
        result[index] = power * mu_prefix - partition_prefix;
    }
    result
}

fn weak_compositions_9_into_8() -> Vec<[i64; 8]> {
    fn visit(position: usize, remaining: i64, current: &mut [i64; 8], output: &mut Vec<[i64; 8]>) {
        if position == 7 {
            current[position] = remaining;
            output.push(*current);
            return;
        }
        for value in 0..=remaining {
            current[position] = value;
            visit(position + 1, remaining - value, current, output);
        }
    }

    let mut output = Vec::new();
    visit(0, 9, &mut [0; 8], &mut output);
    assert_eq!(output.len(), 11_440);
    output
}

fn verify_predecessor_certificate(
    certificate_path: &Path,
) -> Result<(usize, usize, String, i64), Box<dyn Error>> {
    let text = fs::read_to_string(certificate_path)?;
    let raw: Value = serde_json::from_str(&text)?;
    let certificate: PredecessorCertificate = serde_json::from_str(&text)?;
    let expected_mu = [3, 1, 1, 1, 1, 1, 1, 0];
    let expected_nu = [133, 130, 126, 122, 119, 60, 60, 60];
    if certificate.schema_version != 1
        || certificate.mu != expected_mu
        || certificate.nu != expected_nu
        || certificate.tensor_power != 89
        || certificate.total_lr_candidates != 102
        || certificate.total_tensor_candidates != 15
        || certificate.tensor_candidates.len() != 15
    {
        return Err("unsupported predecessor-certificate metadata".into());
    }

    let predecessor_payload = json!({
        "mu": raw.get("mu").ok_or("missing predecessor mu")?,
        "nu": raw.get("nu").ok_or("missing predecessor nu")?,
        "tensor_power": raw.get("tensor_power").ok_or("missing tensor power")?,
        "total_lr_candidates": raw
            .get("total_lr_candidates")
            .ok_or("missing LR count")?,
        "total_tensor_candidates": raw
            .get("total_tensor_candidates")
            .ok_or("missing tensor count")?,
        "tensor_candidates": raw
            .get("tensor_candidates")
            .ok_or("missing tensor candidates")?,
    });
    let digest = sha256_hex(&canonical_json(&predecessor_payload));
    if digest != certificate.payload_sha256 {
        return Err("predecessor payload SHA-256 mismatch".into());
    }

    let mut recorded_lambdas = BTreeSet::new();
    let mut max_predecessor_first_part = 0;
    for candidate in &certificate.tensor_candidates {
        max_predecessor_first_part = max_predecessor_first_part.max(candidate.lambda[0]);
        if !recorded_lambdas.insert(candidate.lambda) {
            return Err("duplicate predecessor partition".into());
        }
        let reconstructed =
            std::array::from_fn(|index| expected_nu[index] - candidate.delta[index]);
        if reconstructed != candidate.lambda {
            return Err("lambda/delta mismatch".into());
        }
        let coefficient = lr_coefficient(&expected_nu, &candidate.lambda, &expected_mu);
        if coefficient != candidate.littlewood_richardson_coefficient || coefficient == 0 {
            return Err(format!("LR coefficient mismatch for {:?}", candidate.lambda).into());
        }
        let slacks = dominance_slacks(&candidate.lambda, &expected_mu, 89);
        if slacks != candidate.dominance_slacks || slacks.iter().any(|value| *value < 0) {
            return Err(format!("dominance mismatch for {:?}", candidate.lambda).into());
        }

        let chain = &candidate.tensor_lr_chain;
        if chain.partitions_from_power_1_to_89.len() != 89
            || chain.step_lr_coefficients.len() != 88
            || chain.partitions_from_power_1_to_89.first() != Some(&expected_mu)
            || chain.partitions_from_power_1_to_89.last() != Some(&candidate.lambda)
        {
            return Err(format!("invalid tensor LR chain for {:?}", candidate.lambda).into());
        }
        for step in 1..89 {
            let previous = &chain.partitions_from_power_1_to_89[step - 1];
            let next = &chain.partitions_from_power_1_to_89[step];
            let step_coefficient = lr_coefficient(next, previous, &expected_mu);
            if step_coefficient != chain.step_lr_coefficients[step - 1] || step_coefficient == 0 {
                return Err(
                    format!("invalid tensor LR step {step} for {:?}", candidate.lambda).into(),
                );
            }
        }
    }

    let mut all_lr_candidates = BTreeSet::new();
    let mut all_tensor_candidates = BTreeSet::new();
    for delta in weak_compositions_9_into_8() {
        let lambda = std::array::from_fn(|index| expected_nu[index] - delta[index]);
        if !is_partition(&lambda) {
            continue;
        }
        if lr_coefficient(&expected_nu, &lambda, &expected_mu) == 0 {
            continue;
        }
        all_lr_candidates.insert(lambda);
        if dominance_slacks(&lambda, &expected_mu, 89)
            .iter()
            .all(|value| *value >= 0)
        {
            all_tensor_candidates.insert(lambda);
        }
    }
    if all_lr_candidates.len() != 102
        || all_tensor_candidates.len() != 15
        || all_tensor_candidates != recorded_lambdas
    {
        return Err(format!(
            "independent predecessor enumeration mismatch: LR={}, tensor={}",
            all_lr_candidates.len(),
            all_tensor_candidates.len()
        )
        .into());
    }

    Ok((
        all_lr_candidates.len(),
        all_tensor_candidates.len(),
        digest,
        max_predecessor_first_part,
    ))
}

fn parse_paths() -> Result<(PathBuf, PathBuf, PathBuf), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("cannot locate repository root")?;
    let mut manifest = root.join("results/certificates/lemma19_matrix_manifest.json");
    let mut certificate = root.join("results/certificates/lemma19_nonzero_minor.json");
    let mut predecessors = root.join("results/certificates/lemma19_predecessor_partitions.json");
    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--manifest" => manifest = PathBuf::from(args.next().ok_or("missing manifest path")?),
            "--certificate" => {
                certificate = PathBuf::from(args.next().ok_or("missing certificate path")?)
            }
            "--predecessors" => {
                predecessors = PathBuf::from(args.next().ok_or("missing predecessors path")?)
            }
            _ => return Err(format!("unknown argument: {argument}").into()),
        }
    }
    Ok((manifest, certificate, predecessors))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (manifest_path, certificate_path, predecessors_path) = parse_paths()?;
    let variables = coefficient_variables();
    let rows = row_manifest();
    let columns = column_manifest();
    let entries = derive_entries(&variables, &rows, &columns)?;
    if entries.len() != 1410 {
        return Err(format!("expected 1410 nonzero entries, got {}", entries.len()).into());
    }

    let generated_payload = payload(&variables, &rows, &columns, &entries);
    let generated_digest = sha256_hex(&canonical_json(&generated_payload));

    let manifest: Value = serde_json::from_str(&fs::read_to_string(&manifest_path)?)?;
    for key in [
        "indexing",
        "coefficient_variables",
        "rows",
        "columns",
        "entries",
    ] {
        if manifest.get(key) != generated_payload.get(key) {
            return Err(format!("manifest mismatch in field {key}").into());
        }
    }
    if manifest.get("payload_sha256").and_then(Value::as_str) != Some(&generated_digest) {
        return Err("manifest payload SHA-256 mismatch".into());
    }

    let certificate: Certificate = serde_json::from_str(&fs::read_to_string(&certificate_path)?)?;
    if certificate.schema_version != 1 || certificate.shape != [90, 115] || certificate.rank != 90 {
        return Err("unsupported certificate metadata".into());
    }
    if certificate.matrix_payload_sha256 != generated_digest {
        return Err("certificate references another matrix payload".into());
    }
    for (key, (_, variable)) in &entries {
        let expected_entry_weight = weight_subtract(
            relation_weight(rows[key.row]),
            variable_weight(columns[key.column]),
        );
        if variable_weight(*variable) != expected_entry_weight {
            return Err(format!(
                "weight factorization failed at ({},{})",
                key.row, key.column
            )
            .into());
        }
    }
    if !is_prime(certificate.prime) || certificate.prime == 2 {
        return Err("certificate modulus is not an odd prime".into());
    }
    if certificate.assignments.len() != variables.len() {
        return Err("assignment count mismatch".into());
    }
    let assignments: BTreeMap<Coord, u64> = certificate
        .assignments
        .iter()
        .map(|record| (record.variable, record.value))
        .collect();
    if assignments.len() != variables.len()
        || variables
            .iter()
            .any(|variable| !assignments.contains_key(variable))
    {
        return Err("assignment variables mismatch".into());
    }
    if assignments
        .values()
        .any(|value| *value >= certificate.prime)
    {
        return Err("assignment outside the finite field".into());
    }
    if certificate.pivot_columns.len() != 90
        || certificate
            .pivot_columns
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            != 90
        || certificate
            .pivot_columns
            .iter()
            .any(|column| *column >= 115)
    {
        return Err("invalid pivot-column list".into());
    }
    if certificate.claimed_weight_candidate.columns.len() != 90
        || certificate
            .claimed_weight_candidate
            .columns
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            != 90
        || certificate
            .claimed_weight_candidate
            .columns
            .iter()
            .any(|column| *column >= 115)
    {
        return Err("invalid claimed-weight column list".into());
    }
    let row_weight_sum = rows
        .iter()
        .copied()
        .map(relation_weight)
        .fold([0_i64; 8], weight_add);
    let selected_column_weight_sum = certificate
        .pivot_columns
        .iter()
        .map(|column| variable_weight(columns[*column]))
        .fold([0_i64; 8], weight_add);
    let pivot_minor_weight = weight_subtract(row_weight_sum, selected_column_weight_sum);
    let pivot_dominant_weight = dominant_reordering(pivot_minor_weight);
    let claimed_highest_weight = [133, 130, 126, 122, 119, 60, 60, 60];
    let pivot_weight_compatible = is_dominated_by(&pivot_dominant_weight, &claimed_highest_weight);

    let mut specialized = vec![vec![0_u64; 115]; 90];
    for (key, (sign, variable)) in &entries {
        let value = assignments[variable] % certificate.prime;
        specialized[key.row][key.column] = if *sign == 1 {
            value
        } else if value == 0 {
            0
        } else {
            certificate.prime - value
        };
    }
    let pivot_minor: Vec<Vec<u64>> = specialized
        .iter()
        .map(|row| {
            certificate
                .pivot_columns
                .iter()
                .map(|column| row[*column])
                .collect()
        })
        .collect();
    let determinant = determinant_mod(pivot_minor, certificate.prime)?;
    if determinant != certificate.determinant_mod_prime {
        return Err(format!(
            "determinant mismatch: recomputed {determinant}, certificate {}",
            certificate.determinant_mod_prime
        )
        .into());
    }
    let minor_89_rows: Vec<usize> = (1..90).collect();
    let minor_89_columns: Vec<usize> = certificate.pivot_columns.iter().copied().skip(1).collect();
    let minor_89 = submatrix(&specialized, &minor_89_rows, &minor_89_columns);
    let determinant_89 = determinant_mod(minor_89, certificate.prime)?;
    let minor_89_weight = selected_minor_weight(&rows, &columns, &minor_89_rows, &minor_89_columns);
    let minor_89_dominant_weight = dominant_reordering(minor_89_weight);
    if determinant_89 != 421_057 || minor_89_weight != [60, 59, 59, 140, 123, 126, 119, 115] {
        return Err("89-minor audit mismatch".into());
    }
    let claimed_column_weight_sum = certificate
        .claimed_weight_candidate
        .columns
        .iter()
        .map(|column| variable_weight(columns[*column]))
        .fold([0_i64; 8], weight_add);
    let claimed_candidate_weight = weight_subtract(row_weight_sum, claimed_column_weight_sum);
    if claimed_candidate_weight != certificate.claimed_weight_candidate.weight
        || dominant_reordering(claimed_candidate_weight) != claimed_highest_weight
    {
        return Err("claimed-weight minor has the wrong torus weight".into());
    }
    let claimed_minor: Vec<Vec<u64>> = specialized
        .iter()
        .map(|row| {
            certificate
                .claimed_weight_candidate
                .columns
                .iter()
                .map(|column| row[*column])
                .collect()
        })
        .collect();
    let (claimed_determinant, claimed_inverse) =
        determinant_and_inverse_mod(claimed_minor, certificate.prime)?;
    if claimed_determinant != certificate.claimed_weight_candidate.determinant_mod_prime
        || claimed_determinant == 0
    {
        return Err("claimed-weight determinant mismatch".into());
    }
    let variable_set: BTreeSet<Coord> = variables.iter().copied().collect();
    let tested_simple_roots = [
        (4_u8, 5_u8),
        (5_u8, 6_u8),
        (6_u8, 7_u8),
        (7_u8, 8_u8),
        (8_u8, 1_u8),
        (1_u8, 2_u8),
        (2_u8, 3_u8),
    ];
    let raising_values: Vec<u64> = tested_simple_roots
        .iter()
        .map(|root| {
            derivative_value_mod(
                &entries,
                &variable_set,
                &assignments,
                &certificate.claimed_weight_candidate.columns,
                *root,
                claimed_determinant,
                &claimed_inverse,
                certificate.prime,
            )
        })
        .collect();
    let expected_raising_values = [685_026, 176_188, 140_239, 78_485, 0, 0, 605_583];
    if raising_values != expected_raising_values {
        return Err(format!("raising derivative mismatch: {raising_values:?}").into());
    }
    let raising_records: Vec<Value> = tested_simple_roots
        .iter()
        .zip(raising_values.iter())
        .map(|((u, v), value)| json!({"root": [u, v], "derivative_mod_prime": value}))
        .collect();
    let (
        lr_candidate_count,
        tensor_candidate_count,
        predecessor_digest,
        max_predecessor_first_part,
    ) = verify_predecessor_certificate(&predecessors_path)?;
    if minor_89_dominant_weight[0] <= max_predecessor_first_part {
        return Err("89-minor dominance obstruction failed".into());
    }

    println!(
        "{}",
        json!({
            "status": "verified",
            "shape": [90, 115],
            "nonzero_entries": entries.len(),
            "payload_sha256": generated_digest,
            "prime": certificate.prime,
            "determinant_mod_prime": determinant,
            "predecessor_payload_sha256": predecessor_digest,
            "lr_predecessor_count": lr_candidate_count,
            "tensor_predecessor_count": tensor_candidate_count,
            "pivot_minor_weight": pivot_minor_weight,
            "pivot_minor_dominant_reordering": pivot_dominant_weight,
            "pivot_weight_compatible_with_claimed_irrep": pivot_weight_compatible,
            "claimed_weight_candidate": claimed_candidate_weight,
            "claimed_weight_candidate_determinant_mod_prime": claimed_determinant,
            "claimed_weight_candidate_boundary": "weight compatibility is necessary, not a proof of pure Schur-module membership",
            "minor_89_determinant_mod_prime": determinant_89,
            "minor_89_weight": minor_89_weight,
            "minor_89_dominant_reordering": minor_89_dominant_weight,
            "minor_89_max_predecessor_first_part": max_predecessor_first_part,
            "claimed_weight_raising_derivatives": raising_records,
            "claimed_weight_candidate_highest_vector": false,
            "implication": "the selected integer maximal-minor polynomial is nonzero in characteristic zero"
        })
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printed_counts_and_sparse_shape() {
        let variables = coefficient_variables();
        let rows = row_manifest();
        let columns = column_manifest();
        let entries = derive_entries(&variables, &rows, &columns).unwrap();
        assert_eq!((variables.len(), rows.len(), columns.len()), (45, 90, 115));
        assert_eq!(entries.len(), 1410);
    }

    #[test]
    fn modular_helpers() {
        let p = 1_000_003;
        assert_eq!(mod_add(p - 1, 2, p), 1);
        assert_eq!(mod_mul(123_456, mod_pow(123_456, p - 2, p), p), 1);
        assert!(is_prime(p));
    }

    #[test]
    fn littlewood_richardson_examples() {
        let mu = [3, 1, 1, 1, 1, 1, 1, 0];
        let nu = [133, 130, 126, 122, 119, 60, 60, 60];
        let multiplicity_one = [130, 129, 125, 121, 118, 60, 59, 59];
        let multiplicity_five = [131, 129, 125, 121, 118, 59, 59, 59];
        assert_eq!(lr_coefficient(&nu, &multiplicity_one, &mu), 1);
        assert_eq!(lr_coefficient(&nu, &multiplicity_five, &mu), 5);
        assert_eq!(weak_compositions_9_into_8().len(), 11_440);
    }
}
