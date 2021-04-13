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
    // if digits = 3, then 10^(3-1) = 10^2 = 100, which is the minimum 3-digit number
    let min = 10usize.pow(options.digits - 1);

    // if digits = 3, then 10^3 - 1 = 1000 - 1 = 999, which is the maximum 3-digit number
    let max = 10usize.pow(options.digits) - 1;

    let mut largest_palindrome = 0;

    // Loops through every unique combination of 2 numbers between [min, max] (inclusive)
    for i in ((min - 1)..(max + 1)).rev() {
        for j in (min..(i + 1)).rev() {
            let n = i * j;

            if is_palindrome(n) && n > largest_palindrome {
                largest_palindrome = n;
            }
        }
    }

    Ok(largest_palindrome)
}

// a number is considered a palindrome if and only if it is equal to it's reversed number
fn is_palindrome(n: usize) -> bool {
    n == reverse_number(n)
}

// reversing the number just inverts the position of the digits
// ie. revers_number(1234) = 4321
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
