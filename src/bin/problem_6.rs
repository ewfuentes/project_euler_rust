fn main() {
    let sum_of_squares: i64 = (1..101).map(|x: i64| x.pow(2)).sum();
    let square_of_sum: i64 = (1..101).sum::<i64>().pow(2);
    println!(
        "square of sum minus sum of squares delta: {}",
        square_of_sum - sum_of_squares
    );
}
