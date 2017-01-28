// Project Euler: Problem 20

extern crate num;

use num::BigUint;
use num::Zero;
use num::One;

/* Again just a large calculation. I wonder how large
 * the numbers have to be before it becomes impractical
 * to just use BigUint.
 */

fn main() {
    let _10 = &BigUint::new(vec![10]);
    let mut n: BigUint = fac(BigUint::new(vec![100]));

    let mut sum = BigUint::zero();

    while !n.is_zero() {
        sum = sum + &n % _10;
        n = n / _10;
    }

    println!("The sum of the digits of 100! is {}", sum);

}

fn fac(n: BigUint) -> BigUint {

    if n.is_zero() {
        BigUint::one()
    } else {
        n.clone() * fac(n - BigUint::one())
    }
}
