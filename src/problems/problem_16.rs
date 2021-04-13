use super::Result;

/*
Problem 16:
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?

Answer: 1366
*/

pub struct Options {
    n: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { n: 1000 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    // store 2^n as an array of its digits, allowing us to go above u128's max
    let mut digits: Vec<usize> = vec![2];

    // each loop multiplies by 2
    for _ in 1..options.n {
        let mut carry = false;

        digits.iter_mut().for_each(|digit| {
            *digit *= 2;

            if carry {
                *digit += 1;
            }

            carry = *digit >= 10;
            if carry {
                *digit %= 10;
            }
        });

        if carry {
            digits.push(1);
        }
    }

    let sum = digits.iter().sum();

    Ok(sum)
}
