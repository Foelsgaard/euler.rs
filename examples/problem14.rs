// Project Euler: Problem 14

extern crate euler;

use euler::util::Collatz;

fn main() {

    let longest_chain = (1u64..1000000)
        .flat_map(|n| Collatz::from(n)
                  .position(|m| m == 1)
                  .map(|i| (n, i)))
        .max_by(|&(_, a), &(_, b)| a.cmp(&b));

    for (n, i) in longest_chain {
        println!("The longest Collatz sequence with a starting \
                  point less than one million starts at {} with \
                  a length of {}.", n, i+1);

        println!();
        print!("The sequence is as follows: ");
        for m in Collatz::from(n) {
            print!("{} ", m);
            if m == 1 {
                break;
            }
        }
        println!();
    }
        
}
