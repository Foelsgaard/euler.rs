// Project Euler: Problem 32

use std::iter::repeat;
use std::collections::HashMap;

fn main() {

    let pandigital_products = (1..9877)
        .flat_map(|n| {
            let d = n.to_string().len();
            repeat(n).zip(1..pow(10, 5-d as u64))
        })
        .map(|(a, b)| (a, b, a * b))
        .filter(|&(a, b, c)| {
            let digits: Vec<char> =
                vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let mut s = a.to_string().chars()
                .chain(b.to_string().chars())
                .chain(c.to_string().chars())
                .collect::<Vec<char>>();
            s.sort();
            s == digits
        });

    let mut map: HashMap<u64, (u64, u64)> = HashMap::new();

    for (a, b, c) in pandigital_products {
        map.insert(c, (a, b));
    }

    println!("These are all the pandigital products:");

    for &(a, b) in map.values() {
        println!("{} * {} = {}", a, b, a * b);
    }

    println!();
    println!("The products sum up to {}.", map.keys().sum::<u64>());
}

fn pow(n:u64, e:u64) -> u64 {
    match e {
        0 => 1,
        e => n * pow(n, e - 1)
    }
}
