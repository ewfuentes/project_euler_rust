struct TriangleNumbers {
    sum: i64,
    step: i64,
}

impl TriangleNumbers {
    fn new() -> TriangleNumbers {
        return TriangleNumbers { sum: 0, step: 0 };
    }
}

impl Iterator for TriangleNumbers {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.step += 1;
        self.sum += self.step;
        return Some(self.sum);
    }
}

fn get_num_divisors(val: i64) -> i64 {
    return project_euler::compute_prime_factors(val)
        .values()
        .map(|&x| x + 1)
        .product();
}

fn main() {
    let mut gen = TriangleNumbers::new();
    let valid_number = gen.find(|&x| get_num_divisors(x) > 500).unwrap();

    println!(
        "first triangle number with more than 500 divisors: {}",
        valid_number
    );
}
