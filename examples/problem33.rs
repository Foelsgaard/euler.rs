// Project Euler: Problem 33

extern crate euler;

use std::str::FromStr;
use std::iter::repeat;

use euler::util::gcd;

fn main() {

    let digit_cancelling_fractions: Vec<(u64, u64)> = (1..)
        .flat_map(|b| (1..b).zip(repeat(b)))
        .map(|(a, b)| {
            let mut a2 = a;
            let mut b2 = b;

            while a2 % 10 == 0 && b2 % 10 == 0 {
                a2 /= 10;
                b2 /= 10;
            }
            (a2, b2)
        })
        .filter(|&(a, b)| {
            let a_str = a.to_string();
            let b_str = b.to_string();
            
            for (i1, c1) in a_str.chars().enumerate() {
                for (i2, c2) in b_str.chars().enumerate() {

                    if c1 == c2 {

                        let mut a_str_clone = a_str.clone();
                        let mut b_str_clone = b_str.clone();

                        a_str_clone.remove(i1);
                        b_str_clone.remove(i2);

                        let cr = u64::from_str(a_str_clone.as_str());
                        let dr = u64::from_str(b_str_clone.as_str());

                        match (cr, dr) {
                            (Ok(c), Ok(d)) => if a * d == c * b {
                                return true;
                            },
                            _ => ()
                        }

                    }
                }
            }

            return false;
        })
        .take(4)
        .collect();

    println!("These are the four digit cancelling fractions:");
    for &(a, b) in digit_cancelling_fractions.iter() {
        println!("{}/{}", a, b);
    }

    let (nom, denom) = digit_cancelling_fractions.iter()
        .fold((1,1), |(a, b), &(c, d)| {
            let gcd = gcd(a * c, b * d);
            (a * c / gcd, b * d / gcd)
        });

    println!("Their product in its lowest common terms is {}/{}.",
             nom, denom);
}
