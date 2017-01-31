// Project Euler: Problem 9

fn main() {

    for c in 1000/3+2..998 {
        for b in (1000-c)/2+1..1000 - c {
            let a = 1000 - c - b;

            if a*a + b*b == c*c  {
                println!("{}^2 + {}^2 = {}^2", a, b, c);
                println!("{} * {} * {} = {}", a, b, c, a*b*c);
            }
        }
    }
}
