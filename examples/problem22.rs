// Project Euler: Problem 22

use std::fs::File;
use std::io::Read;
use std::io::Error;

fn main() {
 
    match gen_names_file_score() {

        Ok(n) => {
            println!("The total score of the names in the file is {}", n);
        }

        Err(e) => {
            println!("{}", e);
        }
    }

}

fn gen_names_file_score() -> Result<usize, Error> {
    let mut f = try!(File::open("resources/p022_names.txt"));
    
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    let mut names_sorted = s.split_terminator(',')
        .map(|ss| ss.chars()
             .filter(|&c| c != '"')
             .collect::<String>())
        .collect::<Vec<String>>();

    names_sorted.sort();

    Ok(names_sorted.iter()
       .enumerate()
       .map(|(i, ss)| string_score(ss)*(i+1))
       .sum::<usize>())
}

static ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn char_score(c: char) -> usize {
    c.to_lowercase()
        .flat_map(|c| {
            ALPHABET.chars()
                .position(|d| {
                    d == c
                })
        })
        .sum::<usize>() + 1
}

fn string_score(s: &String) -> usize {
    s.chars()
        .map(|c| char_score(c))
        .sum()
}
