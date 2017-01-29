// Project Euler: Problem 23

use std::iter::repeat;

fn main() {

    println!("Generating abundant numbers..");

    let mut i = 0;

    let abundant: Vec<u64> = AbundantNumbers::new()
        .take_while(|&m| m < 28124)
        .inspect(|_| {
            if i % 1000 == 0 {
                println!("{}", i);
            }
            i += 1;
        })
        .collect();

    println!("Generating abundant sums..");

    i = 0;

    let mut abundant_sums: Vec<u64> = abundant
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            repeat(a).zip(abundant.iter().skip(i + 1))
        })
        .map(|(a,b)| a + b)
        .inspect(|_| {
            if i % 1000000 == 0 {
                println!("{}", i);
            }
            i += 1;
        })        
        .collect();

    println!("Deduplicating sums..");

    abundant_sums.sort();
    abundant_sums.dedup();


    println!("Summing numbers that are not abundant sums..");

    let mut sum = 0;
    let mut abundant_sums_iter = abundant_sums.iter();

    let mut a = abundant_sums_iter.next();
    for n in 1..28124 {

        match a {
            None => sum += n,
            Some(&m) => {
                if n == m {
                    a = abundant_sums_iter.next();
                } else {
                    sum += n;
                }
            }
        }
    }

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
