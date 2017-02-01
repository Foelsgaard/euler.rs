// Project Euler: Problem 42

use std::fs::File;
use std::io::Read;
use std::io::Error;

fn main() {

    match load_words_file() {
        Ok(file) => {
            let triangle_words = file.split_terminator(',')
                .filter(|word| {
                    let score = word_score(word);
                    let d = isqrt(1 + 8 * score);
                    d * d == 1 + 8 * score && (d - 1) % 2 == 0
                });

            println!("There are {} triangle words in the file.",
                     triangle_words.count());
            
        },

        Err(e) => {
            println!("{}", e);
        }
    }
}


fn load_words_file() -> Result<String, Error> {
    let mut f = try!(File::open("resources/p022_names.txt"));
    
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    Ok(s)
}

fn word_score(word: &str) -> u64 {
    let alphabet: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    word.chars()
        .filter(|&c| c != '"')
        .flat_map(|c| {
            alphabet.chars()
                .position(|d| d == c)
                .map(|n| (n + 1) as u64)
        })
        .sum::<u64>()
}

fn isqrt(n: u64) -> u64 {

    if n == 0 {
        return 0;
    }
    
    let mut x = n;
    let mut x_prev = n;
    
    loop {
        x = (x + n/x)/2;

        if x == x_prev || x + 1 == x_prev || x == x_prev + 1 {
            break;
        }

        x_prev = x;
    }

    x
}
