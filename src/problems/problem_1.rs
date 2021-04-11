use super::Result;

/*
Problem 1:
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.

Answer: 233168
*/

pub struct Options {
    multiples: Vec<usize>,
    exclusive_max: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            multiples: vec![3, 5],
            exclusive_max: 1000,
        }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let sum: usize = (1..options.exclusive_max)
        .filter(|&n| options.multiples.iter().any(|&multiple| n % multiple == 0))
        .sum();

    Ok(sum)
}
