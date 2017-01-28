// Project Euler: Problem 2

fn main() {

    let ns = Fibonacci{current: 1, next: 2};

    let sum: i64 = ns
    	.take_while(|&n| n < 4000000)
        .filter(|&n| n % 2 == 0)
        .sum();

    println!("{}", sum);
}

struct Fibonacci {
    current: i64,
    next: i64
}

impl Iterator for Fibonacci {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {

        self.next = self.current + self.next;
        self.current = self.next - self.current;

        return Some(self.next - self.current);
    }
}
