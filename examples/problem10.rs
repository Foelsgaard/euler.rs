// Project Euler: Problem 10

extern crate euler;

use euler::util::primes;

fn main() {
    println!("The sum of all primes below 2 million is {}",
             primes::<u64>().take_while(|&p| p < 2000000).sum::<u64>());
}
