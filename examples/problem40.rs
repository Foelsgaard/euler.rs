// Project Euler: Problem 40

fn main() {

    let mut string = String::new();

    for n in 1..187500 {
        string.push_str(n.to_string().as_str());
    }

    let d1       = string.remove(0).to_digit(10).unwrap();
    let d10      = string.remove(10 - 2).to_digit(10).unwrap();
    let d100     = string.remove(100 - 3).to_digit(10).unwrap();
    let d1000    = string.remove(1000 - 4).to_digit(10).unwrap();
    let d10000   = string.remove(10000 - 5).to_digit(10).unwrap();
    let d100000  = string.remove(100000 - 6).to_digit(10).unwrap();
    let d1000000 = string.remove(1000000 - 7).to_digit(10).unwrap();


    print!("The product of the digits at positions 1, \
              10, 100, 1000, 10000, 100000, and 1000000 in \
              Champernowne's Constant is ");
    println!("{} * {} * {} * {} * {} * {} * {} = {}.",
             d1, d10, d100, d1000, d10000, d100000, d1000000,
             d1 * d10 * d100 * d1000 * d10000 *d100000 * d1000000);
    
}
