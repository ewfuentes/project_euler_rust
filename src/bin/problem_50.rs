fn main() {
    let prime_list: Vec<usize> = project_euler::Seive::new()
        .take_while(|&x| x < 1_000_000)
        .collect();

    let mut prime_sum = 0;
    let mut num_terms = 0;
    for i in 0..prime_list.len() {
        for j in i + 1..=prime_list.len() {
            let range_sum: usize = prime_list[i..j].iter().sum();

            if range_sum > 1_000_000 {
                break;
            }

            if prime_list.binary_search(&range_sum).is_ok() {
                if j - i > num_terms {
                    println!("New max: {} {}", range_sum, j - i);
                    num_terms = j - i;
                    prime_sum = range_sum;
                }
            }
        }
    }
    println!("max range sum: {} {}", prime_sum, num_terms);
}
