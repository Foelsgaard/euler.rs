// Project Euler: Problem 12

extern crate euler;

use euler::util::primes;

fn main() {

    let highly_divisible_triangle = (1..)
        .map(|n| {
            
            let mut a = n;
            let mut b = n + 1;

            let mut factors = 1;
            let mut multiplicity = 0;
            
            for p in primes() {

                while a % p == 0 {
                    a /= p;

                    multiplicity += 1;
                }

                while b % p == 0 {
                    b /= p;

                    multiplicity += 1;
                }

                factors *= multiplicity;

                multiplicity = 1;

                if a == 1 && b == 1 {
                    break;
                }
            }

            (n, factors)
        })
        .find(|&(_, f)| f > 500);

    for (n, f) in highly_divisible_triangle {
        println!("The first triangle number with more than 500 factors is {} with {} factors.", n, f);
    }
}
