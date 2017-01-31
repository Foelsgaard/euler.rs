// Project Euler: Problem 37

extern crate euler;

use euler::util::primes;
use euler::util::is_prime;

use std::collections::HashSet;

fn main() {

    println!("These are the truncatable primes:");
    let truncatable_primes = primes::<u64>()
        .skip(4)
        .filter(|p| {
            let mut n = p / 10;
            while n > 0 {
                if !is_prime(n) {
                    return false;
                }
                
                n /= 10;
            }
            
            let mut e = (p.to_string().len() - 1) as u64;

            n = p % pow(10, e);

            while n > 0 {
                if !is_prime(n) {
                    return false;
                }

                e -= 1;
                n = n % pow(10, e);
            }

            return true;
        })
        .inspect(|p| {
            println!("{}", p);
        })
        .take(11);

    println!("They sum up to {}.", truncatable_primes.sum::<u64>());
}

fn pow(n: u64, e: u64) -> u64 {
    match e {
        0 => 1,
        e => n * pow(n, e-1)
    }
}
