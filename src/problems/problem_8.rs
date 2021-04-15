use super::Result;

/*
Problem 8:
The four adjacent digits in the 1000-digit number that have the greatest product are 9 × 9 × 8 × 9 = 5832.

73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450

Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. What is the value of this product?

Answer: 23514624000
*/

pub struct Options {
    n: String,
    digits: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            n: "73167176531330624919225119674426574742355349194934
            96983520312774506326239578318016984801869478851843
            85861560789112949495459501737958331952853208805511
            12540698747158523863050715693290963295227443043557
            66896648950445244523161731856403098711121722383113
            62229893423380308135336276614282806444486645238749
            30358907296290491560440772390713810515859307960866
            70172427121883998797908792274921901699720888093776
            65727333001053367881220235421809751254540594752243
            52584907711670556013604839586446706324415722155397
            53697817977846174064955149290862569321978468622482
            83972241375657056057490261407972968652414535100474
            82166370484403199890008895243450658541227588666881
            16427171479924442928230863465674813919123162824586
            17866458359124566529476545682848912883142607690042
            24219022671055626321111109370544217506941658960408
            07198403850962455444362981230987879927244284909188
            84580156166097919133875499200524063689912560717606
            05886116467109405077541002256983155200055935729725
            71636269561882670428252483600823257530420752963450"
                .to_string(),
            digits: 13,
        }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let max_product = options
        .n
        /*
        remove spaces and new lines to make one long number */
        .replace(" ", "")
        .replace("\r", "")
        .replace("\n", "")
        /*
        any zeros within the digits makes the whole product zero. Skip those checks */
        .split("0")
        /*
        filter out empty strings from the split
        or any strings of size less than options.digits, as they will include the split zero within their product */
        .filter(|&numbers| numbers.trim().len() >= options.digits)
        /*
        calculate for each string of numbers left and take the max */
        .map(|numbers| solve_without_zeros(numbers, options.digits))
        .max();

    max_product.ok_or_else(|| {
        format!(
            "Couldn't find max product of {} adjacent digits for: {}",
            options.digits, options.n
        )
    })
}

fn solve_without_zeros(str_of_digits: &str, num_of_digits: usize) -> usize {
    // Map str_of_digits to Vec<u8>
    let digits: Vec<u8> = str_of_digits
        .split("")
        .filter(|&c| !c.is_empty())
        .map(|c| c.parse::<u8>().unwrap())
        .collect();

    let mut product = 1usize;

    // initialize product with first num_of_digits in digits
    for i in 0..num_of_digits {
        product *= digits[i] as usize;
    }

    let mut max_product = product;

    // Iterate through the rest of digits, dividing out the previous, and multiplying in the next
    // This way we're not recalculating the same products over and over. Instead we keep a rolling product and watch for a max
    // This lets us visit each element in "digits" only once
    for i in 1..(digits.len() - num_of_digits + 1) {
        product /= digits[i - 1] as usize;
        product *= digits[i + num_of_digits - 1] as usize;

        if product > max_product {
            max_product = product;
        }
    }

    max_product
}
