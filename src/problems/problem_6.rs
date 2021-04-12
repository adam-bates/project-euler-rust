use super::Result;

/*
Problem 6:
The sum of the squares of the first ten natural numbers is,

    1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,

    (1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is

    3025 - 385 = 2640

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

Answer: 25164150
*/

pub struct Options {
    n: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { n: 100 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let square_of_sums = square_of_sums(options.n);
    let sum_of_squares = sum_of_squares(options.n);

    let diff = square_of_sums - sum_of_squares;

    Ok(diff)
}

fn square_of_sums(n: usize) -> usize {
    let sum = (n * (n + 1)) / 2;
    sum.pow(2)
}

fn sum_of_squares(n: usize) -> usize {
    (1..(n + 1)).map(|n| n.pow(2)).sum()
}
