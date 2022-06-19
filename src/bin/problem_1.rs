fn is_3_or_5_multiple(val: &i64) -> bool {
    return val % 3 == 0 || val % 5 == 0;
}

fn main() {
    let sum: i64 = (1..1000).filter(is_3_or_5_multiple).sum();
    println!("Total sum: {}", sum);
}
