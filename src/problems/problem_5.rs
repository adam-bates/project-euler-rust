use super::Result;

/*
Problem 5:
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

Answer: 232792560
*/

pub struct Options {
    min: usize,
    max: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { min: 1, max: 20 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let min = options.min.max(2); // don't need to check division of 0 or 1
    let max = options.max;

    // The number to check must be divisible by every number in range,
    // so we'll iterate by the maximum amount to skip the maximum amount of work.
    // We also x2 if max is odd, as the number will have to be even since there are
    // even numbers in the range
    let iteration = if max % 2 == 0 { max } else { max * 2 };

    let mut check = max * (max - 1); // any number lower than this shouldn't be divisible by all numbers in the range
    while check <= std::usize::MAX - max {
        if ((min - 1)..(max + 1)).all(|m| check % m == 0) {
            break;
        }

        check += iteration;
    }

    Ok(check)
}
