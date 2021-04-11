use super::Result;

use std::collections::HashMap;

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

    let max_prime_factor = *prime_factors.iter().max().unwrap_or(&0);

    Ok(max_prime_factor)
}

fn is_prime(n: usize, primes_map: &mut HashMap<usize, bool>) -> bool {
    if let Some(&is_prime) = primes_map.get(&n) {
        return is_prime;
    }

    if n % 2 == 0 {
        return n == 2; // 2 is the only even prime
    }

    let max = (n as f64).sqrt().trunc() as usize;
    let max = if max % 2 == 0 { max - 1 } else { max };

    let mut i = max;
    while i >= 3 {
        if n % i == 0 {
            primes_map.insert(n, false);
            return false;
        }

        i -= 2;
    }

    primes_map.insert(n, true);
    true
}

fn prime_factors(n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }

    let mut primes_map = HashMap::new();

    if is_prime(n, &mut primes_map) {
        return vec![n];
    }

    let mut g = n;

    let mut prime_factors = vec![];

    let mut max = (n as f64).sqrt().ceil() as usize;

    while g % 2 == 0 {
        prime_factors.push(2);
        g = g / 2;
    }

    let mut prime = 3;
    while prime <= max {
        if is_prime(prime, &mut primes_map) {
            while g % prime == 0 {
                prime_factors.push(prime);
                g = g / prime;
            }

            if g == 1 {
                break;
            } else if is_prime(g, &mut primes_map) {
                prime_factors.push(g);
                break;
            } else {
                max = (g as f64).sqrt().ceil() as usize;
            }
        }

        prime += 2;
    }

    prime_factors
}
