fn main() {
    const N: i64 = 600851475143;
    let factors = project_euler::compute_prime_factors(N);
    println!("Prime Factors: {:#?}", factors);
}
