// Project Euler: Problem 30

fn main() {

    /* 354294 is an upper limit for how large numbers can be if
     * they are a sum of their digits to the fifth power.
     * Basically 6*9^5 = 354294. If you go larger than this you
     * would need more powers than are digits in the number.
     */
    let digit_power_sums = (2..354295)
        .filter(|&n| {
            let mut sum = 0;
            let mut m = n;
            while m > 0 {
                sum += pow(m % 10, 5);
                m /= 10;
            }

            sum == n
        });

    print!("The following numbers are the sums of the \
              fifth power of their digits: ");
    for n in digit_power_sums {
        print!("{} ", n);
    }
    println!();
}


fn pow(n: u64, e: u64) -> u64 {

    match e {
        0 => 1,
        e => n * pow(n, e-1)
    }

}
