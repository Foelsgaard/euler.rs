// Project Euler: Problem 1

use std::iter::Iterator;

fn main() {

    let sum: i64 = (0..1000)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .sum();

    println!("{}", sum);
}
