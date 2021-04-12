use super::Result;

/*
Problem 15:
Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

x x x x x      x x x - -      x x x - -
-   -   x      -   x   -      -   x   -
- - - - x      - - x x x      - - x - -
-   -   x      -   -   x      -   x   -
- - - - X      - - - - X      - - x x X


x - - - -      x - - - -      x - - - -
x   -   -      x   -   -      x   -   -
x x x x x      x x x - -      x - - - -
-   -   x      -   x   -      x   -   -
- - - - X      - - x x X      x x x x X

How many such routes are there through a 20×20 grid?

Answer: 137846528820
*/

pub struct Options {
    n: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self { n: 20 }
    }
}

/*
For a grid of size (n, n), starting at the top left, moving to the bottom right, you can move either "right" or "down".
So there's 2 distinct types of moves.

There are 2n total moves to be made, with n number of moves to the "right", and n number of moves "down".

We can get the answer by getting the permutations of 2n moves, accounting for n duplicate moves twice.

ie. (2n)! / (n!)^2

We don't need to calculate 2n!, because we're dividing n! a couple times.

Instead we can calculate 2n * (2n-1) * (2n-2) * ... * (n + 2) * (n + 1) / n!

*/
pub fn solve(options: Options) -> Result<u128> {
    let moves = product_partial_factorial(options.n) / factorial(options.n);

    Ok(moves)
}

fn product_partial_factorial(n: usize) -> u128 {
    let mut partial_factorial = (n * 2) as u128;

    for i in (n + 1)..(n * 2) {
        partial_factorial *= i as u128;
    }

    partial_factorial
}

fn factorial(n: usize) -> u128 {
    let mut factorial = n as u128;

    for i in 2..n {
        factorial *= i as u128;
    }

    factorial
}
