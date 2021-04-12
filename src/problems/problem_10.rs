use super::{utils::is_prime, Result};

/*
Problem 10:
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.

Answer: 142913828922
*/

pub struct Options {
    n: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { n: 2_000_000 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    Ok((2..options.n).filter(|&n| is_prime(n)).sum())
}
