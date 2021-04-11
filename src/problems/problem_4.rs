use super::Result;

/*
Problem 4:
A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.

Answer: 906609
*/

pub struct Options {
    min: usize,
    max: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { min: 100, max: 999 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let mut largest_palindrome = 0;

    // Loops through every unique combination of 2 numbers between [min, max] (inclusive)
    for i in ((options.min - 1)..(options.max + 1)).rev() {
        for j in (options.min..(i + 1)).rev() {
            let n = i * j;

            if is_palindrome(n) && n > largest_palindrome {
                largest_palindrome = n;
            }
        }
    }

    Ok(largest_palindrome)
}

fn reverse_number(n: usize) -> usize {
    if n < 10 {
        return n;
    }

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
