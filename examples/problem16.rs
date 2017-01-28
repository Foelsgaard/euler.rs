// Project Euler: Problem 16

extern crate num;

use num::BigUint;
use num::pow::pow;
use num::Zero;

/* No finesse with this one. Just a large calculation.
 */

fn main() {
    let _10 = &BigUint::new(vec![10]);
    let mut n: BigUint = pow(BigUint::new(vec![2]),1000);

    let mut sum = BigUint::zero();

    while !n.is_zero() {
        sum = sum + &n % _10;
        n = n / _10;
    }

    println!("The sum of the digits of 2^1000 is {}", sum);

}
