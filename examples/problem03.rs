// Project Euler: Problem 3

extern crate euler;

use euler::util::primes;

pub fn main() {

    let mut n: u64 = 600851475143;

    let mut largest_prime_factor: u64 = 0;
    
    for p in primes() {

        while n % p == 0 {
            n /= p;
            println!("Prime factor {}, remainder {}", p, n);
        }

        if n == 1 {
            largest_prime_factor = p;
            break;
        }
        
    }

    println!();
    println!("Largest prime factor is {}", largest_prime_factor);

}
