// Project Euler: Problem 24

fn main() {
    let mut lexs =
        LexicographicPermutations::from_with_index("0123456789",
                                                   1000000 - 1);

    for s in lexs.next() {
        
        println!("The millionth lexicographic permutation \
                  of the characters \"0123456789\" is \"{}\".", s);
        
    }
}

struct LexicographicPermutations {
    base: String,
    index: usize
}

impl LexicographicPermutations {

    fn from(s: &str) -> LexicographicPermutations {

        LexicographicPermutations::from_with_index(s, 0)

    }

    fn from_with_index(s: &str, i: usize) -> LexicographicPermutations {
        assert!(i <= fac(s.len()), "Index exceeds number of permutations");
        
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();

        let mut base = String::new();

        for c in chars {
            base.push(c);
        }
        LexicographicPermutations{base: base, index: i}
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
