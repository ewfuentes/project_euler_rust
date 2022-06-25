fn main() {
    for a in 0..998 {
        for b in 0..(1000 - a) {
            let c: i64 = 1000 - a - b;
            if a > c || a > b || b > c {
                continue;
            }
            if a.pow(2) + b.pow(2) == c.pow(2) {
                println!("{} {} {} {}", a, b, c, a * b * c);
            }
        }
    }
}
