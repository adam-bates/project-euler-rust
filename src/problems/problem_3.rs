use super::{utils::prime_factors, Result};

/*
Problem 3:
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?

Answer: 6857
*/

pub struct Options {
    n: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { n: 600_851_475_143 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let prime_factors = prime_factors(options.n);

    let max_prime_factor = *prime_factors
        .iter()
        .max()
        .ok_or_else(|| format!("Couldn't find max prime factor of: {}", options.n))?;

    Ok(max_prime_factor)
}
