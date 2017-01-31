// Project Euler: Problem 5

extern crate euler;

use euler::util::lcm;

fn main() {

    let n = (1..20+1).fold(1, lcm);

    println!("{}", n);
}
