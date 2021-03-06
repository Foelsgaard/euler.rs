extern crate num;

use self::num::Num;
use self::num::Integer;

pub struct Primes<N: Num + Copy + PartialOrd> {
    primes: Vec<N>,
}

pub fn primes<N: Num + Copy + PartialOrd>() -> Primes<N> {
    Primes{primes: vec![]}
}

impl<N: Num + Copy + PartialOrd> Iterator for Primes<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {


        let &current = self.primes.last().unwrap_or(&N::one());

        let mut n = current;
        let mut prime_found = false;
        
        while !prime_found {
            n = n + N::one();

            prime_found = true;
            for &p in self.primes.iter() {
                if n % p == N::zero() {
                    prime_found = false;
                    break;
                }
                if p * p > n {
                    break;
                }
            }
            
        }

        self.primes.push(n);

        Some(n)

    }
}

pub fn lcm<N: Num + Copy>(a: N, b: N) -> N {
    a / gcd(a, b) * b
}

pub fn gcd<N: Num + Copy>(a: N, b: N) -> N {

    let mut n = a;
    let mut m = b;

    while !m.is_zero() {
        let t = m;
        m = n % m;
        n = t;
    }

    return n;

}

pub struct Grid<N: Num + Copy> {
    height: N,
    width: N,
    index: N
}

impl<N: Num + Copy> Grid<N> {
    pub fn new(w: N, h: N) -> Grid<N>{
        Grid{height: h, width: w, index: N::zero()}
    }
}

impl<N: Num + Copy> Iterator for Grid<N> {
    type Item = (N, N);

    fn next(&mut self) -> Option<(N, N)> {
        if self.index == self.width * self.height {
            return None;
        } else {
            let x = self.index % self.width;
            let y = self.index / self.width;
            self.index = self.index + N::one();
            return Some((x,y));
        }

    }
}

pub struct Collatz<N: Num + Copy> {
    n: N
}

impl<N: Num + Copy> Collatz<N> {
    pub fn from(n: N) -> Collatz<N>{
        Collatz{n: n}
    }
}

impl<N: Num + Copy> Iterator for Collatz<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {

        let _0 = N::zero();
        let _1 = N::one();
        let _2 = _1 + _1;
        let _3 = _2 + _1;

        let current = self.n;

        if self.n % _2 == _0 {
            self.n = self.n / _2;
        } else {
            self.n = self.n * _3 + _1;
        }

        Some(current)
    }
}

pub fn n_choose_k<N: Num + Copy>(n: N, k: N) -> N {
    let _1 = N::one();

    let mut a = _1;
    let mut b = _1;

    let mut i = _1;

    // At each step the divisor and divident are divided by
    // their gcd to mitigate the risk of them overflowing.
    while i != k + _1 {
        a = a * (n + _1 - i);
        b = b * i;
        let gcd = gcd(a, b);

        a = a / gcd;
        b = b / gcd;

        i = i + _1;
    }
    
    a / b
}

pub struct Fibonacci<N> {
    current: N,
    next: N
}

impl<N: Num> Fibonacci<N> {
    pub fn new() -> Fibonacci<N> {
        Fibonacci{current: N::one(), next: N::one()}
    }

    pub fn from(a: N, b: N) -> Fibonacci<N> {
        Fibonacci{current: a, next: b}
    }
}

impl<N: Num + Clone> Iterator for Fibonacci<N> {
    type Item = N;

    fn next(&mut self) -> Option<N> {

        self.next = self.current.clone() + self.next.clone();
        self.current = self.next.clone() - self.current.clone();

        return Some(self.next.clone() - self.current.clone());
    }
}

pub fn modular_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut b = base % modulus;
    let mut e = exponent;

    while e > 0 {
        if e % 2 == 1 {
            result = result * b % modulus;
        }
        e >>= 1;
        b = b * b % modulus;
    }

    return result;
}

pub fn is_prime<N: Num + Integer + Clone>(n: N) -> bool {
    if n == N::one() {
        return false;
    }
    
    let mut m = N::one() + N::one();
    while m < n.clone() / (N::one() + N::one()) + N::one() {
        if n.is_multiple_of(&m) {
            return false;
        }
        m = m + N::one();
    }

    return true;
}

pub struct LexicographicPermutations {
    base: String,
    index: usize
}

impl LexicographicPermutations {

    pub fn from(s: &str) -> LexicographicPermutations {
        LexicographicPermutations::from_with_index(s, 0)
    }

    pub fn from_with_index(s: &str, i: usize) -> LexicographicPermutations {
        LexicographicPermutations{base: String::from(s), index: i}
    }

}

impl Iterator for LexicographicPermutations {

    type Item = String;

    fn next(&mut self) -> Option<String> {

        fn fac(n: usize) -> usize {
            match n {
                0 => 1,
                n => n * fac(n-1)
            }
        }

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
