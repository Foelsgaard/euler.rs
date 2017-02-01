// Project Euler: Problem 41

extern crate euler;
extern crate num;

use euler::util::is_prime;

fn main() {

    println!("Searching for pandigital prime..");
    
    let largest_pandigital_prime =
        LexicographicPermutations::from("987654321")
        .chain(LexicographicPermutations::from("87654321"))
        .chain(LexicographicPermutations::from("7654321"))
        .chain(LexicographicPermutations::from("654321"))
        .chain(LexicographicPermutations::from("54321"))
        .chain(LexicographicPermutations::from("4321"))
        .chain(LexicographicPermutations::from("321"))
        .chain(LexicographicPermutations::from("21"))
        .flat_map(|s| {
            u64::from_str_radix(s.as_str(), 10)
        })
        .enumerate()
        .map(|(i, n)| {
            if i % 100000 == 0 {
                println!("{}", i);
            }
            n
        })
        .find(|&n| is_prime(n));

    for p in largest_pandigital_prime {
        println!("The largest pandigital prime is {}.", p);
    }
}


struct LexicographicPermutations {
    base: String,
    index: usize
}

impl LexicographicPermutations {

    fn from(s: &str) -> LexicographicPermutations {

        LexicographicPermutations{base: String::from(s), index: 0}

    }

}

impl Iterator for LexicographicPermutations {

    type Item = String;

    fn next(&mut self) -> Option<String> {

        if self.index == fac(self.base.len()) {
            return None;
        }

        let mut copy = self.base.clone();
        let mut next = String::new();

        while copy.len() > 0 {

            let i = self.index / fac(copy.len()-1) % copy.len();

            next.push(copy.remove(i));

        }

        self.index += 1;
        
        Some(next)
    }
}

fn fac(n: usize) -> usize {
    match n {
        0 => 1,
        n => n * fac(n-1)
    }
}
