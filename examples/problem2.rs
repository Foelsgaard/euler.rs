// Project Euler: Problem 2

extern crate euler;
use euler::util::Fibonacci;

fn main() {

    let ns = Fibonacci::from(1, 2);

    let sum: i64 = ns
    	.take_while(|&n| n < 4000000)
        .filter(|&n| n % 2 == 0)
        .sum();

    println!("{}", sum);
}
