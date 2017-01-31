// Project Euler: Problem 6

fn main() {

    let n = 100;

    let sum_of_squares: i64 = (1..n+1).map(|n| n*n).sum();
    let sum: i64 = (1..n+1).sum();
    let square_of_sum: i64 = sum * sum;

    let difference = square_of_sum - sum_of_squares;

    println!("{} - {} = {}", square_of_sum, sum_of_squares, difference);

}
