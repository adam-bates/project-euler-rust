use super::{utils::is_prime, Result};

/*
Problem 7:
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?

Answer: 104743
*/

pub struct Options {
    n: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { n: 10_001 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    // Assume we've already seen "2", as we will only be looking at odd numbers below
    let mut primes_seen = 1;

    // n += 2 in the loop will start the count at 3
    let mut n = 1;

    while primes_seen < options.n {
        n += 2;

        if is_prime(n) {
            primes_seen += 1;
        }
    }

    Ok(n)
}
