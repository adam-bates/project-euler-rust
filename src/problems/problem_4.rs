use super::Result;

/*
Problem 4:
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.

Answer: 906609
*/

pub struct Options {
    digits: u32,
}

impl Default for Options {
    fn default() -> Self {
        Self { digits: 3 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let min = 10usize.pow(options.digits - 1);
    let max = 10usize.pow(options.digits) - 1;

    let mut largest_palindrome = 0;

    let mut min_idx = min;

    // Loops through every unique combination of 2 numbers between [min, max] (inclusive)
    for i in ((min - 1)..(max + 1)).rev() {
        if i < min_idx {
            break;
        }

        for j in (min..(i + 1)).rev() {
            if j < min_idx {
                break;
            }

            let n = i * j;

            if is_palindrome(n) && n > largest_palindrome {
                largest_palindrome = n;
                min_idx = i.min(j);
            }
        }
    }

    Ok(largest_palindrome)
}

fn reverse_number(n: usize) -> usize {
    let mut g = n;
    let mut reversed = 0;

    while g > 0 {
        let digit = g % 10;
        reversed = (reversed * 10) + digit;
        g /= 10;
    }

    reversed
}

fn is_palindrome(n: usize) -> bool {
    n == reverse_number(n)
}
