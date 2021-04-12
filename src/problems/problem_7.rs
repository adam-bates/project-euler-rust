use super::Result;

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
    let mut primes_seen = 1; // Assume we've already seen "2"

    let mut n = 1; // +2 will start count at 3
    while primes_seen < options.n {
        n += 2;

        if is_prime(n) {
            primes_seen += 1;
        }
    }

    Ok(n)
}

fn is_prime(n: usize) -> bool {
    if n % 2 == 0 {
        return n == 2; // 2 is the only even prime
    }

    // Only search as high as odd number close to ceil(sqrt(n))
    let max = (n as f64).sqrt().trunc() as usize;
    let max = if max % 2 == 0 { max - 1 } else { max };

    let mut i = max;
    while i >= 3 {
        if n % i == 0 {
            return false;
        }

        i -= 2;
    }

    true
}
