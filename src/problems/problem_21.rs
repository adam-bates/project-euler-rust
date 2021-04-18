use super::Result;
use std::collections::{HashMap, HashSet};

/*
Problem 21:
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.

Answer: 31626
*/

pub struct Options {
    exclusive_max: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            exclusive_max: 10_000,
        }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let proper_factors_map = &mut HashMap::new();
    let amicable_numbers = &mut HashSet::new();

    let mut sum = 0;
    for i in 1..options.exclusive_max {
        if amicable_numbers.contains(&i) {
            sum += i;
        } else {
            if let Some(amicable_pair) = get_amicable_pair(i, proper_factors_map) {
                sum += i;
                amicable_numbers.insert(amicable_pair.1);
            }
        }
    }

    Ok(sum)
}

fn get_amicable_pair(
    a: usize,
    proper_factors_map: &mut HashMap<usize, HashSet<usize>>,
) -> Option<(usize, usize)> {
    let b = sum_of_proper_divisors(a, proper_factors_map);

    if b != a && sum_of_proper_divisors(b, proper_factors_map) == a {
        Some((a, b))
    } else {
        None
    }
}

fn sum_of_proper_divisors(
    n: usize,
    proper_factors_map: &mut HashMap<usize, HashSet<usize>>,
) -> usize {
    if let Some(proper_factors) = proper_factors_map.get(&n) {
        return proper_factors.into_iter().sum();
    }

    let mut proper_factors = HashSet::new();
    proper_factors.insert(1);

    let max = (n as f32).sqrt().ceil() as usize;

    for i in 2..max {
        if n % i == 0 {
            proper_factors.insert(i);
            proper_factors.insert(n / i);
        }
    }

    proper_factors_map.insert(n, proper_factors.clone());
    let sum = proper_factors.into_iter().sum();

    sum
}
