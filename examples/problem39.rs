// Project Euler: Problem 39

use std::iter::repeat;
use std::cmp::min;

fn main() {
    println!("Finding solutions..");
    
    let most_soluble_perimeter = (1..1001u64)
        .inspect(|p| {
            if p % 100 == 0 {
                println!("{}", p);
            }
        })
        .map(|p| {
            let solutions: Vec<(_, _)> = (1..p-1)
                .flat_map(|a| repeat(a).zip(1..min(a+1, p - a - 1)))
                .filter(|&(a, b)| {
                    let c = p - a - b;
                    a * a + b * b == c * c
                })
                .collect();

            (p, solutions)
                
        })
        .max_by(|&(_, ref s1), &(_, ref s2)| s1.len().cmp(&s2.len()));


    for (p, solutions) in most_soluble_perimeter {
        println!("The integer right triangle with the \
                  most solutions for perimeters less than or equal to \
                  one thousand has the perimeter {}", p);

        println!();
        println!("The following are its solutions:");

        for (a, b) in solutions {
            let c = p - a - b;
            println!("{}^2 + {}^2 = {}^2", a, b, c);
        }
    }
}
