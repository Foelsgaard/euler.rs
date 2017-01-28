// Project Euler: Problem 21

fn main() {

    let amicable_numbers = (0u64..10000)
        .filter(|&n| d(d(n)) == n);

    println!("The sum of all amicable numbers less than 10000 is {}.",
             amicable_numbers.sum::<u64>());

}

fn d(n: u64) -> u64 {
    (1..n).filter(|m| {
        n % m == 0
    }).sum()
}
