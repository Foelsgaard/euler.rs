// Project Euler: Problem 35

extern crate euler;

use euler::util::primes;

use std::collections::HashSet;

fn main() {
    let primes: HashSet<u64> = primes()
        .take_while(|&p| p < 1000000)
        .collect();
    

    let circular_primes = primes.iter()
        .filter(|&&p| {
            let e = p.to_string().len() as u64;
            let mut n = p / 10 + p % 10 * pow(10, e - 1);

            while n != p {
                if !primes.contains(&n) {
                    return false;
                }

                n = n / 10 + n % 10 * pow(10, e - 1);
            }
            return true;
        });

    println!("There are the {} circular primes below one million." ,
             circular_primes.count());
}

fn pow(n: u64, e: u64) -> u64 {
    match e {
        0 => 1,
        e => n * pow(n, e-1)
    }
}
