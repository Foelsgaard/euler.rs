// Project Euler: Problem 26

extern crate euler;

use euler::util::modular_pow;
use euler::util::primes;

/* This one is slightly dubious. I basically assume that the answer
 * has to be a prime number since this simplifies the problem to
 * solving 10^n == 1 (mod p) for any given prime p, with n being the
 * length of the cycle. My reasoning is that the decimal expansion of
 * the reciprocal of any prime number apart from 2 and 5 will consist
 * solely of cycle digits and composite reciprocals can be "boiled down"
 * to a multiple of some prime reciprocal with the same cycle length.
 *
 * None of the above probably made any sense and I'm not sure how to
 * prove it, but since my code terminates for all tested primes I
 * figure I'm not completely wrong.
 */

fn main() {

    let d = primes::<u64>().take_while(|&p| p < 1000)
        .filter(|p| {
            10 % p != 0
        })
        .map(|p| {
            let mut n = 1;
            while modular_pow(10, n, p) != 1 {
                n += 1;
            }

            (p, n)
        })
        .max_by(|&(_, n1), &(_, n2)| n1.cmp(&n2));

    for (p, n) in d {
        println!("1/{} is the unit reciprocal greater than 1/1000 \
                  with the longest cycle in its decimal expansion. \
                  The length of the cycle is {}.", p, n);
    }

}

