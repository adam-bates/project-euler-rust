use super::Result;

/*
Problem 14:
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.

Answer: 837799
*/

pub struct Options {
    exclusive_max: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            exclusive_max: 1_000_000,
        }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let mut max_length = 0;
    let mut max_length_n = 0;

    for n in 2..options.exclusive_max {
        let length = get_chain_length(n);

        if length > max_length {
            max_length_n = n;
            max_length = length;
        }
    }

    Ok(max_length_n)
}

fn get_chain_length(n: usize) -> usize {
    let mut g = n;
    let mut chain_length = 1;

    while g != 1 {
        g = if g % 2 == 0 { g / 2 } else { (3 * g) + 1 };
        chain_length += 1;
    }

    chain_length
}
