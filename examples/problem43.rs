// Project Euler: Problem 43

extern crate euler;

use euler::util::LexicographicPermutations;

fn main() {

    println!("Finding sub-string divisible pandigital numbers..");
    let sub_string_divisibles =
        LexicographicPermutations::from("0123456789")
        .filter(|s| {
            let d: Vec<u32> = s.chars()
                .flat_map(|c| c.to_digit(10))
                .collect();

            d[1]*d[2]*d[3] % 2 == 0 &&
                d[2]*d[3]*d[4] % 3 == 0 &&
                d[3]*d[4]*d[5] % 5 == 0 &&
                d[4]*d[5]*d[6] % 7 == 0 &&
                d[5]*d[6]*d[7] % 11 == 0 &&
                d[6]*d[7]*d[8] % 13 == 0 &&
                d[7]*d[8]*d[9] % 17 == 0
        })
        .enumerate()
        .map(|(i, v)| {
            if i % 1000 == 0 {
                println!("{}", i);
            }
            v
        })
        .flat_map(|s| {
            u64::from_str_radix(s.as_str(), 10)
        });

    println!("The sum of all sub-string divisible numbers is {}.",
             sub_string_divisibles.sum::<u64>());

}
