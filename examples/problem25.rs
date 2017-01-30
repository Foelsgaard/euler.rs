// Project Euler: Problem 25

extern crate euler;
extern crate num;

use num::BigUint;

use euler::util::Fibonacci;

/* Another brute force use of BigUint.
 */

fn main() {

    let fib = Fibonacci::<BigUint>::new()
        .enumerate()
        .find(|&(_, ref n)| {
            n.to_string().len() == 1000
        });

    for (i, _) in fib {

        println!("The Fibonacci with an index of {} is \
                  the first one with a thousand digits.", i+1);
    }
}
