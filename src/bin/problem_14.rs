
use itertools::Itertools;

struct Collatz {
    state: Option<usize>,
    initial_seed: usize,
    has_terminated: bool,
}

impl Collatz {
    fn new(state: usize) -> Collatz {
        return Collatz{state: None, initial_seed: state, has_terminated: false};
    }
}

impl Iterator for Collatz {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_terminated {
            return None;
        }

        if self.state.is_some() {
            let mut state = self.state.unwrap();
            if state % 2 == 0 {
                state = state / 2;
            } else {
                state = 3 * state + 1;
            }
            self.state = Some(state);
        } else {
            self.state = Some(self.initial_seed);
        }

        if self.state.unwrap() == 1 {
            self.has_terminated = true;
        }

        return self.state;
    }
}

fn main() {
    const MAX_SEED: usize = 1_000_000;
    let mut collatz_lengths: Vec<Option<usize>> = vec![None; MAX_SEED];
    for start in 1..MAX_SEED {
        if collatz_lengths[start].is_some() {
            continue;
        }
        let collatz = Collatz::new(start).enumerate().collect_vec();
        let seq_length = collatz.len();
        for (idx, value) in collatz {
            if value >= MAX_SEED {
                continue;
            }
            if collatz_lengths[value].is_some() {
                break;
            }
            collatz_lengths[value] = Some(seq_length - idx - 1);
        }
    }

    let max_length_seed = collatz_lengths.iter().position_max().unwrap();

    println!("Max length seed: {} length: {:#?}", max_length_seed, collatz_lengths[max_length_seed].unwrap());
}
