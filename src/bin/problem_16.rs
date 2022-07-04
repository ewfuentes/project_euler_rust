use itertools::Itertools;
use num_bigint::ToBigInt;

fn main() {
    let num = 2i64.to_bigint().unwrap().pow(1000);
    let digit_sum: u64 = num.to_radix_le(10).1.into_iter().map_into::<u64>().sum();
    println!("num: {}", digit_sum);
}
