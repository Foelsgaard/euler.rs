#![feature(test)]

pub mod util;

extern crate test;

#[cfg(test)]
mod tests {

    use test::Bencher;
    use util::primes;
    
    #[bench]
    fn benchmark_primes(b: &mut Bencher) {

        b.iter(|| primes().nth(1000));
    }

}
