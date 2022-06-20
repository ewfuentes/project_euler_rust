use std::collections::HashMap;

fn get_smallest_multiple(factors: HashMap<i64, i64>, val: i64) -> HashMap<i64, i64> {
    let mut out = factors.clone();
    let new_factors = project_euler::compute_prime_factors(val);
    for (prime, power) in new_factors.iter() {
        let existing_count = *factors.get(&prime).unwrap_or(&0);
        if existing_count < *power {
            out.insert(*prime, *power);
        }
    }
    return out;
}

fn main() {
    let smallest_multiple = (1..21).fold(HashMap::new(), get_smallest_multiple);
    let smallest_multiple: i64 = smallest_multiple
        .iter()
        .map(|prime_and_power| prime_and_power.0.pow(*prime_and_power.1 as u32))
        .product();

    println!("Smallest multiple: {:#?}", smallest_multiple);
}
