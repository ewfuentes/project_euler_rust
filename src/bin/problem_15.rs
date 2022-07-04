type WrapState = (usize, usize);
use std::hash::Hash;

fn options_from_state(state: &WrapState, max_rows: usize, max_cols: usize) -> Vec<WrapState> {
    let &(row, col) = state;
    let mut out = Vec::new();
    if row < max_rows - 1 {
        out.push((row + 1, col));
    }

    if col < max_cols - 1 {
        out.push((row, col + 1));
    }

    return out;
}

fn count_paths<T, U>(state: T, next_state_fn: &mut U, cache: &mut std::collections::HashMap<T, usize>) -> usize
where
    T: Eq + Hash,
    U: FnMut(&T) -> Vec<T>,
{
    if cache.contains_key(&state) {
        return *cache.get(&state).unwrap();
    }
    let next_states = next_state_fn(&state);
    if next_states.len() == 0 {
        return 1;
    }
    let total_paths = next_states.into_iter().map(|s| count_paths(s, next_state_fn, cache)).sum();
    cache.insert(state, total_paths);
    return total_paths;
}

fn main() {
    const MAX_ROWS: usize = 21;
    const MAX_COLS: usize = 21;
    let mut next_state_fn =
        |state: &WrapState| options_from_state(state, MAX_ROWS, MAX_COLS);
    let initial_state = (0, 0);
    let total_paths = count_paths(initial_state, &mut next_state_fn, &mut std::collections::HashMap::new());
    println!("Total Paths: {}", total_paths);
}
