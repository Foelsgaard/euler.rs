// Project Euler: Problem 38

use std::str::FromStr;

fn main() {

    let largest_pandigital_multiple = (1..10000u64)
        .map(|n| {
            let mut m = 0;
            let mut string = String::new();

            while string.len() < 9 {
                m += 1;
                string.push_str((n * m).to_string().as_str());
            }

            (n, string, m)
        })
        .filter(|&(_, ref string, _)| {
            let digits = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

            let mut char_vec: Vec<char> = string.chars().collect();
            char_vec.sort();

            digits == char_vec
        })
        .flat_map(|(n, string, m)| {
            u64::from_str(string.as_str()).map(|p| (n, p, m))
        })
        .max_by(|&(_, p1, _), &(_, p2, _)| p1.cmp(&p2));

    for (n, p, m) in largest_pandigital_multiple {
        println!("The largest pandigital multiple is \
                  {}, the concatenation of the following:", p);

        for a in 1..m+1 {
            println!("{} * {} = {}", n, a, n * a);
        }
    }
}
