// Project Euler: Problem 7

extern crate euler;

use euler::util::primes;

fn main() {

    let n = 10000;

    let p: u64 = primes().nth(n).unwrap();

    println!("The {}th prime is {}", n+1,  p);
}
