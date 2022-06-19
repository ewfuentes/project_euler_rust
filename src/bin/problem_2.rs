struct FibGenerator {
    // The first state contains the value to output next the second state
    // contains the value to previous output value
    state: [i64; 2],
}

impl FibGenerator {
    fn new() -> FibGenerator {
        return FibGenerator { state: [1, 0] };
    }
}

impl Iterator for FibGenerator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        let out_val = self.state[0];
        self.state[0] = self.state.iter().sum();
        self.state[1] = out_val;
        return Some(out_val);
    }
}

fn main() {
    let fib_gen = FibGenerator::new();
    let even_sum: i64 = fib_gen
        .take_while(|&x| x < 4_000_000i64)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("Even Fibonacci Sum less than 4M: {}", even_sum);
}
