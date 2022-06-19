use itertools::Itertools;

fn is_palindrome(val: i64) -> bool {
    let val = val.to_string();
    return val == val.chars().rev().collect::<String>();
}

struct Palindrome {
    a: i64,
    b: i64,
}

impl PartialEq for Palindrome {
    fn eq(&self, other: &Palindrome) -> bool {
        return (self.a * self.b) == (other.a * other.b);
    }
}

impl PartialOrd for Palindrome {
    fn partial_cmp(&self, other: &Palindrome) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Palindrome {
    fn cmp(&self, other: &Palindrome) -> std::cmp::Ordering {
        return (self.a * self.b).cmp(&(other.a * other.b))
    }
}

impl std::fmt::Debug for Palindrome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Palindrome [{} {} {}]", self.a, self.b, self.a * self.b)
    }
}

impl Eq for Palindrome {}

fn main() {
    let palindromes = (100..1000)
        .cartesian_product(100..1000)
        .filter(|&x| x.1 > x.0 && is_palindrome(x.0 * x.1))
        .map(|x| Palindrome { a: x.0, b: x.1 })
        .sorted();

    println!("palindrome length: {}", palindromes.len());
    println!("{:#?}", palindromes);
}
