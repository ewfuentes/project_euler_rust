
fn main() {
    let seive = project_euler::Seive::new();
    let prime_sum: usize = seive.take_while(|&x| x < 2_000_000).sum();
    println!("{}", prime_sum);
}
