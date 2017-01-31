// Project Euler: Problem 34

fn main() {
    let digit_factorials: Vec<u64> = (10..2540161u64)
        .filter(|&n| {
            let mut sum = 0u64;
            let mut m = n;

            while m > 0 {
                sum += fac(m % 10);
                m /= 10;
            }

            sum == n
        })
        .collect();

    println!("The following numbers are equal to the sum \
              of the factorials of their digits:");

    for n in digit_factorials.iter() {
        println!("{}", n);
    }
    
    println!("They sum up to {}.",
             digit_factorials.iter().sum::<u64>());
}


fn fac(n: u64) -> u64 {
    match n {
        0 => 1,
        n => n * fac(n-1)
    }
}
