// Project Euler: Problem 27

extern crate euler;

use euler::util::is_prime;

use std::iter::repeat;

fn main() {

    println!("Finding quadratic prime formulas..");

    let mut i = 0;
    let qp = (-999..1000)
        .flat_map(|a| {
            repeat(a).zip(-1000..1001)
        })
        .inspect(|_| {
            if i % 100000 == 0 {
                println!("{} ", i);
            }
            i += 1;
        })
        .map(|(a, b)| {
            (a, b, QuadraticPrimes::from(a, b).count())
        })
        .max_by(|&(_, _, n1), &(_, _, n2)| n1.cmp(&n2));

    println!();

    for (a, b, n) in qp {

        let a_sign = if a < 0 { "-" } else { "+" };
        let b_sign = if b < 0 { "-" } else { "+" };

        println!("The formula n^2 {} {}n {} {}, 0 <= n < {}, \
                  gives the longest sequence of consecutive primes",
                 a_sign, a.abs(), b_sign, b.abs(), n);

        println!();
        println!("The sequence is as follows: ");

        for p in QuadraticPrimes::from(a, b) {
            print!("{} ", p);
        }

        println!();
    }

}


struct QuadraticPrimes {
    a: i64,
    b: i64,
    n: u64
}

impl QuadraticPrimes {

    fn from(a: i64, b:i64) -> QuadraticPrimes {

        QuadraticPrimes{a: a, b: b, n: 0}
    }
}

impl Iterator for QuadraticPrimes {

    type Item = u64;

    fn next(&mut self) -> Option<u64> {

        let p: i64 = (self.n * self.n) as i64 +
            self.a * (self.n as i64) + self.b;

        if p < 2 {
            return None;
        }

        self.n += 1;

        if is_prime(p) {
            return Some(p as u64);
        } else {
            return None;
        }
    }
}
