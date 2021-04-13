use std::collections::HashMap;

pub fn is_prime(n: usize) -> bool {
    if n % 2 == 0 {
        return n == 2; // 2 is the only even prime
    }

    // Only search as high as odd number close to sqrt(n)
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

pub fn is_prime_cached(n: usize, primes_map: &mut HashMap<usize, bool>) -> bool {
    if let Some(&is_prime) = primes_map.get(&n) {
        return is_prime;
    }

    if n % 2 == 0 {
        return n == 2; // 2 is the only even prime
    }

    let is_prime = is_prime(n);

    primes_map.insert(n, is_prime);
    is_prime
}

pub fn prime_factors(n: usize) -> Vec<usize> {
    prime_factors_cached(n, &mut HashMap::new(), &mut HashMap::new())
}

pub fn prime_factors_cached(
    n: usize,
    prime_factors_map: &mut HashMap<usize, Vec<usize>>,
    primes_map: &mut HashMap<usize, bool>,
) -> Vec<usize> {
    if n < 2 {
        return vec![];
    }

    if is_prime_cached(n, primes_map) {
        return vec![n];
    }

    if let Some(prime_factors) = prime_factors_map.get(&n) {
        return prime_factors.clone();
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
        if is_prime_cached(prime, primes_map) {
            while g % prime == 0 {
                prime_factors.push(prime);
                g /= prime;
            }

            if g == 1 {
                break;
            } else if is_prime_cached(g, primes_map) {
                prime_factors.push(g);
                break;
            } else {
                max = (g as f64).sqrt().ceil() as usize;
            }
        }

        prime += 2;
    }

    prime_factors_map.insert(n, prime_factors.clone());
    prime_factors
}
