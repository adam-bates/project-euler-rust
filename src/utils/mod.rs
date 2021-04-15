use std::collections::HashMap;

pub fn is_prime(n: usize) -> bool {
    if n % 2 == 0 {
        return n == 2; // 2 is the only even prime
    }

    // Only search as high as odd number close to sqrt(n)
    let max = (n as f64).sqrt().trunc() as usize;
    let max = if max % 2 == 0 { max + 1 } else { max };

    let mut i = 3;
    while i <= max {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

pub fn is_prime_cached(n: usize, primes_map: &mut HashMap<usize, bool>) -> bool {
    if let Some(&is_prime) = primes_map.get(&n) {
        return is_prime;
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
        g /= 2;
    }

    let mut prime = 3;
    while prime <= max {
        // check divisible before expensive prime check
        if g % prime == 0 && is_prime_cached(prime, primes_map) {
            // do-while
            {
                prime_factors.push(prime);
                g /= prime;

                while g % prime == 0 {
                    prime_factors.push(prime);
                    g /= prime;
                }
            }

            if g == 1 {
                // Found all factors and added them to prime_factors vec
                break;
            } else if is_prime_cached(g, primes_map) {
                // Found all factors, still need to add self to prime_factors vec
                prime_factors.push(g);
                break;
            } else {
                // Still searching for prime_factors,

                // can narrow-down search though, after dividing out the found primes
                max = (g as f64).sqrt().ceil() as usize;
            }
        }

        prime += 2;
    }

    prime_factors_map.insert(n, prime_factors.clone());
    prime_factors
}
