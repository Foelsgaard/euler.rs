// Project Euler: Problem 29

use std::collections::HashSet;

fn main() {

    let mut powers = HashSet::new();

    for a in 2..101 {
        for b in 2..101 {
            powers.insert(a^b);
        }
    }

    println!("There are {} distinct powers a^b for \
              2 <= a <= 100 and 2 <= b <= 100.", powers.len());
}
