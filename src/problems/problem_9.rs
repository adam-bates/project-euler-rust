use super::Result;

/*
Problem 9:
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

Answer: 31875000
*/

pub struct Options {
    sum: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { sum: 1000 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    for a in 1..(options.sum / 3) {
        for b in (a + 1)..(options.sum / 2) {
            let c = options.sum - a - b;

            if a.pow(2) + b.pow(2) == c.pow(2) {
                return Ok(a * b * c);
            }
        }
    }

    Err(format!(
        "Couldn't find Pythagorean triplet which sum to {}",
        options.sum
    ))
}
