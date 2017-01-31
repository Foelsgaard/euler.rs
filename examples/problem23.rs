// Project Euler: Problem 23

use std::iter::repeat;
use std::collections::HashSet;

fn main() {

    println!("Generating abundant numbers..");

    let abundant: Vec<u64> = AbundantNumbers::new()
        .take_while(|&m| m < 28124)
        .enumerate()
        .map(|(i, n)| {
            if i % 1000 == 0 {
                println!("{}", i);
            }
            n
        })
        .collect();

    println!("Generating abundant sums..");

    let abundant_sums: HashSet<u64> = abundant
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            repeat(a).zip(abundant.iter().skip(i + 1))
        })
        .map(|(a,b)| a + b)
        .enumerate()
        .map(|(i, n)| {
            if i % 1000000 == 0 {
                println!("{}", i);
            }
            n
        })        
        .collect();

    println!("Summing numbers that are not abundant sums..");

    let sum: u64 =(1..28124)
        .filter(|n| !abundant_sums.contains(n))
        .sum();

    println!("The sum of all numbers that are not abundant sums is {}.", sum);
 
}


struct AbundantNumbers {
    current: u64
}

impl AbundantNumbers {
    fn new() -> AbundantNumbers {
        AbundantNumbers{current: 1}
    }
}

impl Iterator for AbundantNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let next_abundant = (self.current+1..)
            .find(|&n| d(n) > n);

        for n in next_abundant {
            self.current = n;
        }

        next_abundant
    }
}

fn d(n: u64) -> u64 {
    (1..n/2+1).filter(|m| {
        n % m == 0
    }).sum()
}
