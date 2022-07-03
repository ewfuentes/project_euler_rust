use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

// fn families_of_length(len: usize) -> Vec<String> {
//     let chars = "*0123456789";
//     let indexer = (0..len)
//         .map(|_x| 0..chars.len())
//         .multi_cartesian_product()
//         .filter(|idx| idx.contains(&0));
//     return indexer
//         .map(|tup| {
//             tup.iter()
//                 .map(|&idx| chars.chars().nth(idx).unwrap())
//                 .collect()
//         })
//         .collect();
// }
//
// fn regex_str_from_family(family: &str) -> String {
//     let mut saw_first_wild = false;
//     let elements: Vec<String> = family
//         .chars()
//         .map(|c| {
//             if c == '*' {
//                 if saw_first_wild {
//                     r"(\1)".to_string()
//                 } else {
//                     saw_first_wild = true;
//                     r"(\d)".to_string()
//                 }
//             } else {
//                 c.to_string()
//             }
//         })
//         .collect();
//     return "^".to_string() + &elements.iter().join("") + "$";
// }
//
// fn find_matching_primes<'a>(prime_list: &'a Vec<String>, family: &str) -> Vec<&'a String> {
//     let family_regex = regex_str_from_family(&family);
//     let re = fancy_regex::Regex::new(&family_regex).unwrap();
//     return prime_list
//         .iter()
//         .filter(|prime| prime.len() == family.len() && re.is_match(prime).unwrap_or(false))
//         .collect();
// }

fn replace_at_idx(input: &str, idxs: &Vec<&usize>) -> String {
    let mut out = input.to_string();
    idxs.iter()
        .for_each(|&&x| out = out[..x].to_owned() + "*" + &out[x + 1..]);
    return out;
}

fn families_for_value(value: usize) -> HashSet<String> {
    let value = value.to_string();
    let indexes_from_char = value.to_string().chars().enumerate().fold(
        HashMap::new(),
        |mut accum: HashMap<_, Vec<usize>>, (idx, c)| {
            accum
                .entry(c)
                .and_modify(|e| e.push(idx))
                .or_insert(vec![idx]);
            return accum;
        },
    );

    return indexes_from_char
        .iter()
        .map(|(_, idxs)| {
            idxs.iter()
                .powerset()
                .map(|rep_idxs| replace_at_idx(&value, &rep_idxs))
        })
        .flatten()
        .collect();
}

// fn prime_families<'a>(
//     length: usize,
//     prime_list: &'a Vec<String>,
// ) -> std::collections::HashMap<String, Vec<&'a String>> {
//     return families_of_length(length)
//         .par_iter()
//         .map(|family| (family.clone(), find_matching_primes(prime_list, &family)))
//         .inspect(|x| println!("{} {}", x.0, x.1.len()))
//         .collect();
// }

fn main() {
    let primes_by_family = project_euler::Seive::new()
        .take_while(|&x| x < 10_000_000)
        .par_bridge()
        .map(|x| (x, families_for_value(x)))
        .collect::<Vec<(usize, HashSet<String>)>>()
        .into_iter()
        .fold(
            HashMap::new(),
            |mut accum: HashMap<_, Vec<usize>>, (prime, families)| {
                families
                    .into_iter()
                    .filter(|family| family.contains("*"))
                    .for_each(|family| accum.entry(family).or_default().push(prime));
                accum
            },
        )
        .into_iter()
        .sorted_by(|(_family_a, primes_a), (_family_b, primes_b)| {
            return primes_a.len().cmp(&primes_b.len());
        })
        .collect::<Vec<(String, Vec<usize>)>>();
    for (family, primes) in primes_by_family {
        println!("{} {} {:#?}", family, primes.len(), primes);
    }
}
