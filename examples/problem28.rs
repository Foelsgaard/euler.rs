// Project Euler: Problem 28

fn main() {

    println!("The diagonal sum of the 1001x1001 number spiral is {}",
             diagonal_sum(1001));
}

/* This can be reduced further to avoid the recursive call,
 * but it's a bit tedious and error prone. The recursive
 * function is fast enough for now.
 */

fn diagonal_sum(side_length: u64) -> u64 {
    assert!(side_length % 2 == 1);

    match side_length {
        1 => 1,
        n => 4 * n * n - 6 * n + 6 + diagonal_sum(n - 2)
    }
    
}
