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
