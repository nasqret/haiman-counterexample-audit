use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const N: usize = 5;
const ALPHABET: usize = 15;
const GRID: usize = ALPHABET * ALPHABET * ALPHABET;
const MODULUS: ModInt = 1_000_000_007;

type Weight = [u8; N];
type Key4 = [u8; 4];
type ModInt = u32;

#[derive(Clone, Debug)]
struct Column3 {
    key4: Key4,
    grid_index: usize,
}

#[derive(Clone, Debug)]
struct Column2 {
    key4: Key4,
    first: usize,
    second: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Shape {
    pi: [u8; 3],
    height3_columns: usize,
    has_final_height2_column: bool,
}

#[derive(Clone, Debug)]
struct Target {
    label: String,
    alpha: Weight,
    shape: Shape,
}

#[derive(Clone, Copy, Debug)]
enum DpMethod {
    DenseByWeight,
    SparseByColumn,
}

#[derive(Debug, Serialize)]
struct MultiplicityRecord {
    label: String,
    alpha: Vec<u8>,
    gl3_partition_pi: Vec<u8>,
    modulus: ModInt,
    shifted_weight_terms: usize,
    schur_multiplicity_mod_prime: ModInt,
    certified_nonzero_over_integers: bool,
}

#[derive(Debug, Serialize)]
struct ShapeRunRecord {
    gl3_partition_pi: Vec<u8>,
    modulus: ModInt,
    target_weights: usize,
    states_after_height3_columns: usize,
    height3_columns: usize,
    has_final_height2_column: bool,
}

#[derive(Debug, Serialize)]
struct Payload {
    schema_version: u64,
    source: String,
    status: String,
    modulus: ModInt,
    dp_method: String,
    method: String,
    character_formula: String,
    shape_runs: Vec<ShapeRunRecord>,
    multiplicities: Vec<MultiplicityRecord>,
    open_limitations: Vec<String>,
}

#[derive(Debug, Serialize)]
struct Certificate {
    payload_sha256: String,
    #[serde(flatten)]
    payload: Payload,
}

fn grid_index(a: usize, b: usize, c: usize) -> usize {
    (a * ALPHABET + b) * ALPHABET + c
}

fn add_mod(left: ModInt, right: ModInt) -> ModInt {
    let sum = left as u64 + right as u64;
    if sum >= MODULUS as u64 {
        (sum - MODULUS as u64) as ModInt
    } else {
        sum as ModInt
    }
}

fn sub_mod(left: ModInt, right: ModInt) -> ModInt {
    if left >= right {
        left - right
    } else {
        left + MODULUS - right
    }
}

fn add_weight(first: &Weight, second: &Weight) -> Weight {
    let mut out = [0u8; N];
    for index in 0..N {
        out[index] = first[index] + second[index];
    }
    out
}

fn key4(weight: &Weight) -> Key4 {
    [weight[0], weight[1], weight[2], weight[3]]
}

fn key4_add(first: &Key4, second: &Key4) -> Key4 {
    [
        first[0] + second[0],
        first[1] + second[1],
        first[2] + second[2],
        first[3] + second[3],
    ]
}

fn key4_to_weight(key: &Key4, total_degree: usize) -> Option<Weight> {
    let partial = key.iter().map(|value| *value as usize).sum::<usize>();
    if partial > total_degree {
        return None;
    }
    let fifth = total_degree - partial;
    if fifth > u8::MAX as usize {
        return None;
    }
    Some([key[0], key[1], key[2], key[3], fifth as u8])
}

fn within_bounds(
    key: &Key4,
    total_degree: usize,
    min_target: &Weight,
    max_target: &Weight,
    future_coordinate_max: usize,
) -> bool {
    let Some(weight) = key4_to_weight(key, total_degree) else {
        return false;
    };
    for index in 0..N {
        let value = weight[index] as usize;
        if value > max_target[index] as usize {
            return false;
        }
        if value + future_coordinate_max < min_target[index] as usize {
            return false;
        }
    }
    true
}

fn alphabet_weights() -> Vec<Weight> {
    let mut weights = Vec::new();
    for i in 0..N {
        for j in i..N {
            let mut weight = [0u8; N];
            weight[i] += 1;
            weight[j] += 1;
            weights.push(weight);
        }
    }
    assert_eq!(weights.len(), ALPHABET);
    weights
}

fn build_columns3(alphabet: &[Weight]) -> Vec<Column3> {
    let mut columns = Vec::new();
    for a in 0..ALPHABET {
        for b in (a + 1)..ALPHABET {
            for c in (b + 1)..ALPHABET {
                let weight = add_weight(&add_weight(&alphabet[a], &alphabet[b]), &alphabet[c]);
                columns.push(Column3 {
                    key4: key4(&weight),
                    grid_index: grid_index(a, b, c),
                });
            }
        }
    }
    assert_eq!(columns.len(), 455);
    columns
}

fn build_columns2(alphabet: &[Weight]) -> Vec<Column2> {
    let mut columns = Vec::new();
    for a in 0..ALPHABET {
        for b in (a + 1)..ALPHABET {
            let weight = add_weight(&alphabet[a], &alphabet[b]);
            columns.push(Column2 {
                key4: key4(&weight),
                first: a,
                second: b,
            });
        }
    }
    assert_eq!(columns.len(), 105);
    columns
}

fn prefix3(grid: &mut [ModInt]) {
    for a in 1..ALPHABET {
        for b in 0..ALPHABET {
            for c in 0..ALPHABET {
                let idx = grid_index(a, b, c);
                let prev = grid_index(a - 1, b, c);
                grid[idx] = add_mod(grid[idx], grid[prev]);
            }
        }
    }
    for a in 0..ALPHABET {
        for b in 1..ALPHABET {
            for c in 0..ALPHABET {
                let idx = grid_index(a, b, c);
                let prev = grid_index(a, b - 1, c);
                grid[idx] = add_mod(grid[idx], grid[prev]);
            }
        }
    }
    for a in 0..ALPHABET {
        for b in 0..ALPHABET {
            for c in 1..ALPHABET {
                let idx = grid_index(a, b, c);
                let prev = grid_index(a, b, c - 1);
                grid[idx] = add_mod(grid[idx], grid[prev]);
            }
        }
    }
}

fn update_cell(vector: &mut [ModInt], index: usize, value: ModInt) {
    vector[index] = add_mod(vector[index], value);
}

fn add_to_map(map: &mut HashMap<Key4, ModInt>, key: Key4, value: ModInt) {
    let entry = map.entry(key).or_insert(0);
    *entry = add_mod(*entry, value);
}

fn merge_map_into(source: &HashMap<Key4, ModInt>, target: &mut HashMap<Key4, ModInt>) {
    for (key, value) in source {
        add_to_map(target, *key, *value);
    }
}

fn aggregate_sparse_entries(state: &[HashMap<Key4, ModInt>]) -> usize {
    state.iter().map(HashMap::len).sum()
}

fn prefix_sparse_state(state: &[HashMap<Key4, ModInt>]) -> Vec<HashMap<Key4, ModInt>> {
    let mut prefix = state.to_vec();
    for a in 1..ALPHABET {
        for b in 0..ALPHABET {
            for c in 0..ALPHABET {
                let previous = prefix[grid_index(a - 1, b, c)].clone();
                let target = &mut prefix[grid_index(a, b, c)];
                merge_map_into(&previous, target);
            }
        }
    }
    for a in 0..ALPHABET {
        for b in 1..ALPHABET {
            for c in 0..ALPHABET {
                let previous = prefix[grid_index(a, b - 1, c)].clone();
                let target = &mut prefix[grid_index(a, b, c)];
                merge_map_into(&previous, target);
            }
        }
    }
    for a in 0..ALPHABET {
        for b in 0..ALPHABET {
            for c in 1..ALPHABET {
                let previous = prefix[grid_index(a, b, c - 1)].clone();
                let target = &mut prefix[grid_index(a, b, c)];
                merge_map_into(&previous, target);
            }
        }
    }
    prefix
}

fn initialize_height3_sparse(
    columns3: &[Column3],
    shape: &Shape,
    min_target: &Weight,
    max_target: &Weight,
) -> Vec<HashMap<Key4, ModInt>> {
    let mut state = vec![HashMap::new(); GRID];
    let final_height2_degree = if shape.has_final_height2_column { 4 } else { 0 };
    let future_coordinate_max =
        4 * (shape.height3_columns - 1) + if final_height2_degree == 0 { 0 } else { 3 };
    for column in columns3 {
        if !within_bounds(&column.key4, 6, min_target, max_target, future_coordinate_max) {
            continue;
        }
        add_to_map(&mut state[column.grid_index], column.key4, 1);
    }
    state
}

fn step_height3_sparse(
    state: Vec<HashMap<Key4, ModInt>>,
    columns3: &[Column3],
    completed_height3_columns: usize,
    shape: &Shape,
    min_target: &Weight,
    max_target: &Weight,
) -> Vec<HashMap<Key4, ModInt>> {
    let total_degree = 6 * completed_height3_columns;
    let remaining_height3 = shape.height3_columns - completed_height3_columns;
    let final_height2_degree = if shape.has_final_height2_column { 4 } else { 0 };
    let future_coordinate_max =
        4 * remaining_height3 + if final_height2_degree == 0 { 0 } else { 3 };
    let prefix = prefix_sparse_state(&state);
    let mut next = vec![HashMap::new(); GRID];
    for column in columns3 {
        let source = &prefix[column.grid_index];
        if source.is_empty() {
            continue;
        }
        let target = &mut next[column.grid_index];
        for (key, value) in source {
            let next_key = key4_add(key, &column.key4);
            if within_bounds(
                &next_key,
                total_degree,
                min_target,
                max_target,
                future_coordinate_max,
            ) {
                add_to_map(target, next_key, *value);
            }
        }
    }
    next
}

fn step_height3(
    state: HashMap<Key4, Vec<ModInt>>,
    columns3: &[Column3],
    completed_height3_columns: usize,
    shape: &Shape,
    min_target: &Weight,
    max_target: &Weight,
) -> HashMap<Key4, Vec<ModInt>> {
    let total_degree = 6 * completed_height3_columns;
    let remaining_height3 = shape.height3_columns - completed_height3_columns;
    let final_height2_degree = if shape.has_final_height2_column { 4 } else { 0 };
    let future_coordinate_max = 4 * remaining_height3
        + if final_height2_degree == 0 { 0 } else { 3 };
    let mut next: HashMap<Key4, Vec<ModInt>> = HashMap::new();
    let mut grid = vec![0 as ModInt; GRID];

    for (key, vector) in state {
        grid.fill(0);
        for (index, value) in vector.into_iter().enumerate() {
            if value != 0 {
                grid[columns3[index].grid_index] = value;
            }
        }
        prefix3(&mut grid);
        for (next_index, column) in columns3.iter().enumerate() {
            let value = grid[column.grid_index];
            if value == 0 {
                continue;
            }
            let next_key = key4_add(&key, &column.key4);
            if !within_bounds(
                &next_key,
                total_degree,
                min_target,
                max_target,
                future_coordinate_max,
            ) {
                continue;
            }
            let entry = next
                .entry(next_key)
                .or_insert_with(|| vec![0 as ModInt; columns3.len()]);
            update_cell(entry, next_index, value);
        }
    }
    next
}

fn initialize_height3(
    columns3: &[Column3],
    shape: &Shape,
    min_target: &Weight,
    max_target: &Weight,
) -> HashMap<Key4, Vec<ModInt>> {
    let mut state: HashMap<Key4, Vec<ModInt>> = HashMap::new();
    let final_height2_degree = if shape.has_final_height2_column { 4 } else { 0 };
    let future_coordinate_max = 4 * (shape.height3_columns - 1)
        + if final_height2_degree == 0 { 0 } else { 3 };
    for (index, column) in columns3.iter().enumerate() {
        if !within_bounds(&column.key4, 6, min_target, max_target, future_coordinate_max) {
            continue;
        }
        let entry = state
            .entry(column.key4)
            .or_insert_with(|| vec![0 as ModInt; columns3.len()]);
        update_cell(entry, index, 1);
    }
    state
}

fn target_bounds(targets: &BTreeSet<Weight>) -> (Weight, Weight) {
    let mut min_target = [u8::MAX; N];
    let mut max_target = [0u8; N];
    for target in targets {
        for index in 0..N {
            min_target[index] = min_target[index].min(target[index]);
            max_target[index] = max_target[index].max(target[index]);
        }
    }
    (min_target, max_target)
}

fn compute_weight_multiplicities_dense(
    shape: &Shape,
    targets: &BTreeSet<Weight>,
    columns3: &[Column3],
    columns2: &[Column2],
) -> (BTreeMap<Weight, ModInt>, ShapeRunRecord) {
    let (min_target, max_target) = target_bounds(targets);
    let mut state = initialize_height3(columns3, shape, &min_target, &max_target);

    for completed in 2..=shape.height3_columns {
        state = step_height3(
            state,
            columns3,
            completed,
            shape,
            &min_target,
            &max_target,
        );
        eprintln!(
            "pi={:?}: completed height-3 column {}/{}; dense weight states={}",
            shape.pi,
            completed,
            shape.height3_columns,
            state.len()
        );
    }

    let mut multiplicities = BTreeMap::new();
    if shape.has_final_height2_column {
        let total_degree = 6 * shape.height3_columns + 4;
        let target_lookup: HashSet<Weight> = targets.iter().copied().collect();
        let mut grid = vec![0 as ModInt; GRID];
        for (key, vector) in state.iter() {
            grid.fill(0);
            for (index, value) in vector.iter().copied().enumerate() {
                if value != 0 {
                    grid[columns3[index].grid_index] = value;
                }
            }
            prefix3(&mut grid);
            for column in columns2 {
                let value = grid[grid_index(column.first, column.second, ALPHABET - 1)];
                if value == 0 {
                    continue;
                }
                let next_key = key4_add(key, &column.key4);
                let Some(weight) = key4_to_weight(&next_key, total_degree) else {
                    continue;
                };
                if target_lookup.contains(&weight) {
                    let entry = multiplicities.entry(weight).or_insert(0);
                    *entry = add_mod(*entry, value);
                }
            }
        }
    } else {
        let total_degree = 6 * shape.height3_columns;
        for (key, vector) in state.iter() {
            let Some(weight) = key4_to_weight(key, total_degree) else {
                continue;
            };
            if targets.contains(&weight) {
                let mut value = 0 as ModInt;
                for cell in vector {
                    value = add_mod(value, *cell);
                }
                multiplicities.insert(weight, value);
            }
        }
    }

    let run = ShapeRunRecord {
        gl3_partition_pi: shape.pi.iter().copied().collect(),
        modulus: MODULUS,
        target_weights: targets.len(),
        states_after_height3_columns: state.len(),
        height3_columns: shape.height3_columns,
        has_final_height2_column: shape.has_final_height2_column,
    };
    (multiplicities, run)
}

fn compute_weight_multiplicities_sparse(
    shape: &Shape,
    targets: &BTreeSet<Weight>,
    columns3: &[Column3],
    columns2: &[Column2],
) -> (BTreeMap<Weight, ModInt>, ShapeRunRecord) {
    let (min_target, max_target) = target_bounds(targets);
    let mut state = initialize_height3_sparse(columns3, shape, &min_target, &max_target);

    for completed in 2..=shape.height3_columns {
        state = step_height3_sparse(
            state,
            columns3,
            completed,
            shape,
            &min_target,
            &max_target,
        );
        eprintln!(
            "pi={:?}: completed height-3 column {}/{}; sparse entries={}",
            shape.pi,
            completed,
            shape.height3_columns,
            aggregate_sparse_entries(&state)
        );
    }

    let mut multiplicities = BTreeMap::new();
    if shape.has_final_height2_column {
        let total_degree = 6 * shape.height3_columns + 4;
        let target_lookup: HashSet<Weight> = targets.iter().copied().collect();
        let prefix = prefix_sparse_state(&state);
        for column in columns2 {
            let source = &prefix[grid_index(column.first, column.second, ALPHABET - 1)];
            for (key, value) in source {
                let next_key = key4_add(key, &column.key4);
                let Some(weight) = key4_to_weight(&next_key, total_degree) else {
                    continue;
                };
                if target_lookup.contains(&weight) {
                    let entry = multiplicities.entry(weight).or_insert(0);
                    *entry = add_mod(*entry, *value);
                }
            }
        }
    } else {
        let total_degree = 6 * shape.height3_columns;
        for map in state.iter() {
            for (key, value) in map {
                let Some(weight) = key4_to_weight(key, total_degree) else {
                    continue;
                };
                if targets.contains(&weight) {
                    let entry = multiplicities.entry(weight).or_insert(0);
                    *entry = add_mod(*entry, *value);
                }
            }
        }
    }

    let run = ShapeRunRecord {
        gl3_partition_pi: shape.pi.iter().copied().collect(),
        modulus: MODULUS,
        target_weights: targets.len(),
        states_after_height3_columns: aggregate_sparse_entries(&state),
        height3_columns: shape.height3_columns,
        has_final_height2_column: shape.has_final_height2_column,
    };
    (multiplicities, run)
}

fn compute_weight_multiplicities(
    shape: &Shape,
    targets: &BTreeSet<Weight>,
    columns3: &[Column3],
    columns2: &[Column2],
    method: DpMethod,
) -> (BTreeMap<Weight, ModInt>, ShapeRunRecord) {
    match method {
        DpMethod::DenseByWeight => {
            compute_weight_multiplicities_dense(shape, targets, columns3, columns2)
        }
        DpMethod::SparseByColumn => {
            compute_weight_multiplicities_sparse(shape, targets, columns3, columns2)
        }
    }
}

fn permutation_sign(permutation: &[usize; N]) -> i32 {
    let mut inversions = 0usize;
    for i in 0..N {
        for j in (i + 1)..N {
            if permutation[i] > permutation[j] {
                inversions += 1;
            }
        }
    }
    if inversions % 2 == 0 {
        1
    } else {
        -1
    }
}

fn permutations5() -> Vec<([usize; N], i32)> {
    let mut out = Vec::new();
    for a in 0..N {
        for b in 0..N {
            if b == a {
                continue;
            }
            for c in 0..N {
                if c == a || c == b {
                    continue;
                }
                for d in 0..N {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 0..N {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let permutation = [a, b, c, d, e];
                        let sign = permutation_sign(&permutation);
                        out.push((permutation, sign));
                    }
                }
            }
        }
    }
    assert_eq!(out.len(), 120);
    out
}

fn shifted_weights(alpha: Weight, permutations: &[([usize; N], i32)]) -> Vec<(Weight, i32)> {
    let rho = [4i16, 3, 2, 1, 0];
    let mut out = Vec::new();
    for (permutation, sign) in permutations {
        let mut shifted = [0u8; N];
        let mut valid = true;
        for index in 0..N {
            let value = alpha[index] as i16 + rho[index] - rho[permutation[index]];
            if value < 0 || value > u8::MAX as i16 {
                valid = false;
                break;
            }
            shifted[index] = value as u8;
        }
        if valid {
            out.push((shifted, *sign));
        }
    }
    out
}

fn schur_multiplicity_mod(
    alpha: Weight,
    weight_multiplicities: &BTreeMap<Weight, ModInt>,
    permutations: &[([usize; N], i32)],
) -> ModInt {
    let mut value = 0 as ModInt;
    for (weight, sign) in shifted_weights(alpha, permutations) {
        let term = weight_multiplicities.get(&weight).copied().unwrap_or(0);
        if sign > 0 {
            value = add_mod(value, term);
        } else {
            value = sub_mod(value, term);
        }
    }
    value
}

fn parse_weight(value: &Value, key: &str) -> Result<Weight, Box<dyn Error>> {
    let array = value[key]
        .as_array()
        .ok_or_else(|| format!("missing array field {key}"))?;
    if array.len() != N {
        return Err(format!("expected {key} to have length {N}").into());
    }
    let mut out = [0u8; N];
    for (index, entry) in array.iter().enumerate() {
        out[index] = entry
            .as_u64()
            .ok_or_else(|| format!("{key}[{index}] is not a nonnegative integer"))?
            .try_into()
            .map_err(|_| format!("{key}[{index}] does not fit in u8"))?;
    }
    Ok(out)
}

fn parse_shape(value: &Value) -> Result<Shape, Box<dyn Error>> {
    let array = value["gl3_partition_pi"]
        .as_array()
        .ok_or("missing gl3_partition_pi")?;
    if array.len() != 3 {
        return Err("gl3_partition_pi must have length 3".into());
    }
    let mut pi = [0u8; 3];
    for (index, entry) in array.iter().enumerate() {
        pi[index] = entry
            .as_u64()
            .ok_or("gl3_partition_pi entry is not a nonnegative integer")?
            .try_into()
            .map_err(|_| "gl3_partition_pi entry does not fit in u8")?;
    }
    if pi[0] != pi[1] {
        return Err(format!("unsupported non-two-row-rectangle prefix pi={pi:?}").into());
    }
    let has_final_height2_column = pi[1] > pi[2];
    if pi[1] - pi[2] > 1 {
        return Err(format!("unsupported shape pi={pi:?}").into());
    }
    Ok(Shape {
        pi,
        height3_columns: pi[2] as usize,
        has_final_height2_column,
    })
}

fn parse_targets(path: &Path) -> Result<Vec<Target>, Box<dyn Error>> {
    let value: Value = serde_json::from_str(&fs::read_to_string(path)?)?;
    let mut targets = Vec::new();

    let degree90 = &value["degree90_target"];
    targets.push(Target {
        label: "degree90_target".to_string(),
        alpha: parse_weight(degree90, "gl5_target_alpha")?,
        shape: parse_shape(degree90)?,
    });

    let degree89 = value["degree89_targets"]
        .as_array()
        .ok_or("missing degree89_targets")?;
    for entry in degree89 {
        let index = entry["candidate_index"]
            .as_u64()
            .ok_or("candidate_index is missing")?;
        targets.push(Target {
            label: format!("degree89_candidate_{index}"),
            alpha: parse_weight(entry, "gl5_target_alpha")?,
            shape: parse_shape(entry)?,
        });
    }
    Ok(targets)
}

fn canonical_json<T: Serialize>(value: &T) -> Result<String, Box<dyn Error>> {
    Ok(serde_json::to_string(value)?)
}

fn certificate_digest(payload: &Payload) -> Result<String, Box<dyn Error>> {
    let canonical = canonical_json(payload)?;
    let digest = Sha256::digest(canonical.as_bytes());
    Ok(format!("{digest:x}"))
}

fn default_targets_path() -> PathBuf {
    PathBuf::from("results/certificates/lemma19_symmetric_multiplicity_targets.json")
}

fn default_output_path() -> PathBuf {
    PathBuf::from("results/certificates/lemma19_symmetric_multiplicities.json")
}

fn parse_method(args: &[String]) -> Result<DpMethod, Box<dyn Error>> {
    for arg in args {
        if arg == "--method=sparse" {
            return Ok(DpMethod::SparseByColumn);
        }
        if arg == "--method=dense" {
            return Ok(DpMethod::DenseByWeight);
        }
    }
    Ok(DpMethod::DenseByWeight)
}

fn method_label(method: DpMethod) -> &'static str {
    match method {
        DpMethod::DenseByWeight => "dense_by_weight",
        DpMethod::SparseByColumn => "sparse_by_column",
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let write_output = args.iter().any(|arg| arg == "--write");
    let method = parse_method(&args)?;
    let targets_path = default_targets_path();
    let output_path = default_output_path();
    let targets = parse_targets(&targets_path)?;
    let permutations = permutations5();

    let alphabet = alphabet_weights();
    let columns3 = build_columns3(&alphabet);
    let columns2 = build_columns2(&alphabet);

    let mut grouped: BTreeMap<[u8; 3], (Shape, Vec<Target>)> = BTreeMap::new();
    for target in targets {
        grouped
            .entry(target.shape.pi)
            .or_insert_with(|| (target.shape.clone(), Vec::new()))
            .1
            .push(target);
    }

    let mut shape_runs = Vec::new();
    let mut records = Vec::new();
    for (_pi, (shape, shape_targets)) in grouped {
        let mut shifted_target_set = BTreeSet::new();
        for target in &shape_targets {
            for (weight, _sign) in shifted_weights(target.alpha, &permutations) {
                shifted_target_set.insert(weight);
            }
        }
        let (weight_multiplicities, shape_run) =
            compute_weight_multiplicities(
                &shape,
                &shifted_target_set,
                &columns3,
                &columns2,
                method,
            );
        shape_runs.push(shape_run);

        for target in shape_targets {
            let multiplicity = schur_multiplicity_mod(
                target.alpha,
                &weight_multiplicities,
                &permutations,
            );
            records.push(MultiplicityRecord {
                label: target.label,
                alpha: target.alpha.iter().copied().collect(),
                gl3_partition_pi: target.shape.pi.iter().copied().collect(),
                modulus: MODULUS,
                shifted_weight_terms: 120,
                schur_multiplicity_mod_prime: multiplicity,
                certified_nonzero_over_integers: multiplicity != 0,
            });
        }
    }

    records.sort_by(|left, right| left.label.cmp(&right.label));
    let all_nonzero = records
        .iter()
        .all(|record| record.certified_nonzero_over_integers);
    let payload = Payload {
        schema_version: 1,
        source: "artifacts/rust/src/bin/plethysm_multiplicity.rs".to_string(),
        status: if all_nonzero {
            "all_target_multiplicities_nonzero_mod_prime"
        } else {
            "some_target_multiplicities_zero_mod_prime_or_unresolved"
        }
        .to_string(),
        modulus: MODULUS,
        dp_method: method_label(method).to_string(),
        method: "Semistandard-column dynamic programming for the weights of s_pi(Sym^2 V), followed by Weyl-character multiplicity extraction.".to_string(),
        character_formula: "multiplicity(alpha)=sum_{sigma in S_5} sign(sigma) weight_multiplicity(alpha+rho-sigma(rho)), rho=(4,3,2,1,0)".to_string(),
        shape_runs,
        multiplicities: records,
        open_limitations: vec![
            "Residues are modulo one prime; a nonzero residue rigorously proves nonzero integer multiplicity, but the exact integer multiplicities are not computed here.".to_string(),
            "This does not construct explicit highest-weight vectors inside Sym^89(S_mu W) or test J_8 membership.".to_string(),
        ],
    };
    let certificate = Certificate {
        payload_sha256: certificate_digest(&payload)?,
        payload,
    };
    let rendered = serde_json::to_string_pretty(&certificate)? + "\n";
    if write_output {
        fs::write(&output_path, rendered)?;
        println!("{}", output_path.display());
    } else {
        print!("{rendered}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn coefficient_mod(pi: [u8; 3], alpha: Weight) -> ModInt {
        let alphabet = alphabet_weights();
        let columns3 = build_columns3(&alphabet);
        let columns2 = build_columns2(&alphabet);
        let permutations = permutations5();
        let shape = Shape {
            pi,
            height3_columns: pi[2] as usize,
            has_final_height2_column: pi[1] > pi[2],
        };
        let targets = shifted_weights(alpha, &permutations)
            .into_iter()
            .map(|(weight, _sign)| weight)
            .collect::<BTreeSet<_>>();
        let (weight_multiplicities, _run) = compute_weight_multiplicities(
            &shape,
            &targets,
            &columns3,
            &columns2,
            DpMethod::DenseByWeight,
        );
        schur_multiplicity_mod(alpha, &weight_multiplicities, &permutations)
    }

    #[test]
    fn exterior_cube_of_symmetric_square_examples() {
        // Sage check:
        // s[1,1,1][s[2]] = s[3,3] + s[4,1,1].
        assert_eq!(coefficient_mod([1, 1, 1], [3, 3, 0, 0, 0]), 1);
        assert_eq!(coefficient_mod([1, 1, 1], [4, 1, 1, 0, 0]), 1);
        assert_eq!(coefficient_mod([1, 1, 1], [5, 1, 0, 0, 0]), 0);
    }

    #[test]
    fn shape_221_sage_examples() {
        // Sage check:
        // s[2,2,1][s[2]] contains s[4,4,2] and s[7,2,1],
        // but not s[5,5].
        assert_eq!(coefficient_mod([2, 2, 1], [4, 4, 2, 0, 0]), 1);
        assert_eq!(coefficient_mod([2, 2, 1], [7, 2, 1, 0, 0]), 1);
        assert_eq!(coefficient_mod([2, 2, 1], [5, 5, 0, 0, 0]), 0);
    }
}
