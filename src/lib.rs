
struct SeivePrime {
    value: usize,
    last_multiple: Option<usize>,
}

pub struct Seive {
    primes: Vec<SeivePrime>,
    bools: Vec<bool>,
    start: usize,
}

impl Seive {
    fn value_from_index(idx: usize, seive_start: usize) -> usize {
        return 2 * idx + seive_start;
    }

    fn index_from_value(value: usize, seive_start: usize) -> Option<usize> {
        if value < seive_start || value % 2 == 0 {
            return None;
        }
        return Some((value - seive_start) / 2);
    }

    pub fn new() -> Seive {
        const SEIVE_SIZE: usize = 10000000;
        return Seive {
            primes: Vec::new(),
            bools: vec![true; SEIVE_SIZE],
            start: 3,
        };
    }
}

impl Iterator for Seive {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.primes.is_empty() {
            self.primes.push(SeivePrime {
                value: 2,
                last_multiple: None,
            });
            return Some(2);
        } else {
            let lower_bound =
                Seive::index_from_value(self.primes.last().unwrap().value, self.start).unwrap_or(0);
            let upper_bound = self.bools.len();
            for idx in lower_bound..upper_bound {
                if self.bools[idx] {
                    // Found a prime!
                    let new_prime = Seive::value_from_index(idx, self.start);
                    let last_multiple = 'outer: loop {
                        let mut last_valid_mult = 1;
                        for mult in 1.. {
                            let maybe_idx = Seive::index_from_value(new_prime * mult, self.start);
                            if maybe_idx.is_none() {
                                continue;
                            }
                            let idx = maybe_idx.unwrap();
                            if idx >= self.bools.len() {
                                break 'outer new_prime*last_valid_mult;
                            }
                            self.bools[idx] = false;
                            last_valid_mult = mult;
                        }
                    };

                    self.primes.push(SeivePrime {
                        value: new_prime,
                        last_multiple: Some(last_multiple)
                    });

                    return Some(new_prime);
                }
            }
        }

        return Some(0);
    }
}



pub fn compute_prime_factors(x: i64) -> std::collections::HashMap<i64, i64> {
    let mut val = x;
    let mut factors = std::collections::HashMap::new();
    for i in 1..(x + 1) {
        while val % i == 0 && i != 1 {
            let existing_count = *factors.get(&i).unwrap_or(&0);
            factors.insert(i, existing_count + 1);
            val /= i;
        }
        if val < i {
            break;
        }
    }
    return factors;
}
