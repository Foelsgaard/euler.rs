// Project Euler: Problem 4

fn main() {

    let (a, b) = (0..999).zip(0..999)
        .filter(|&(n, m)| n * m == reverse_number(n * m))
        .max_by(|&(a, b), &(c, d)| (a * b).cmp(&(c * d)))
        .unwrap();

    println!("{} * {} = {}", a, b, a * b);
}


fn reverse_number(n: i64) -> i64 {

    fn go(n: i64, i: u32) -> i64 {

        match i {
            0 => n,
            _ => n / 10i64.pow(i-1) + 10 * go(n % 10i64.pow(i-1), i-1)
        }
    }

    go(n, digits(n))
}

fn digits(n: i64) -> u32 {

    match n {
        0 => 0,
        n => 1 + digits(n/10)
    }
}
