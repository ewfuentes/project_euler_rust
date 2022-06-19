fn compute_prime_factors(x: i64) -> Vec<i64> {
    let mut val = x;
    let mut factors = Vec::<i64>::new();
    let upper_bound = (x as f64).sqrt().ceil() as i64;
    for i in 2..upper_bound {
        if val % i == 0 {
            factors.push(i);
            val /= i;
        }
        if val < i {
            break;
        }
    }
    return factors;
}

fn main() {
    const N: i64 = 600851475143;
    let factors = compute_prime_factors(N);
    println!("Prime Factors: {:#?}", factors);
}
