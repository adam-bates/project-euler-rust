use super::Result;

/*
Problem 20:
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!

Answer: 648
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
    // store n! as an array of its digits, allowing us to go above u128's max
    let mut digits: Vec<usize> = vec![1];

    // each loop multiplies by n
    for n in 2..(options.n + 1) {
        let mut carry = 0;

        digits.iter_mut().for_each(|digit| {
            *digit = (*digit * n) + carry;

            carry = if *digit < 10 {
                0
            } else {
                (*digit - (*digit % 10)) / 10
            };

            *digit %= 10;
        });

        if carry > 0 {
            digits.push(carry);

            // Handle carry >= 10
            {
                let mut carry = 0;

                digits.iter_mut().for_each(|digit| {
                    *digit = *digit + carry;

                    carry = if *digit < 10 {
                        0
                    } else {
                        (*digit - (*digit % 10)) / 10
                    };

                    *digit %= 10;
                });

                if carry > 0 {
                    digits.push(carry);
                }
            }
        }
    }

    let sum = digits.iter().sum();

    Ok(sum)
}
