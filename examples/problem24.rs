// Project Euler: Problem 24

extern crate euler;

use euler::util::LexicographicPermutations;

fn main() {
    let mut lexs =
        LexicographicPermutations::from_with_index("0123456789",
                                                   1000000 - 1);

    for s in lexs.next() {
        
        println!("The millionth lexicographic permutation \
                  of the characters \"0123456789\" is \"{}\".", s);
        
    }
}
