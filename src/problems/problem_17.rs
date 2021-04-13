use super::Result;
use std::collections::HashMap;

/*
Problem 17:
If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters.
      The use of "and" when writing out numbers is in compliance with British usage.

Answer: 21124
*/

pub struct Options {
    from: u16,
    to: u16,
}

impl Default for Options {
    fn default() -> Self {
        Self { from: 1, to: 1000 }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let mut letters_map: HashMap<u16, u8> = HashMap::new();
    letters_map.insert(1, 3); // one
    letters_map.insert(2, 3); // two
    letters_map.insert(3, 5); // three
    letters_map.insert(4, 4); // four
    letters_map.insert(5, 4); // five
    letters_map.insert(6, 3); // six
    letters_map.insert(7, 5); // seven
    letters_map.insert(8, 5); // eight
    letters_map.insert(9, 4); // nine
    letters_map.insert(10, 3); // ten
    letters_map.insert(11, 6); // eleven
    letters_map.insert(12, 6); // twelve
    letters_map.insert(13, 8); // thirteen
    letters_map.insert(14, 8); // fourteen
    letters_map.insert(15, 7); // fifteen
    letters_map.insert(16, 7); // sixteen
    letters_map.insert(17, 9); // seventeen
    letters_map.insert(18, 8); // eighteen
    letters_map.insert(19, 8); // nineteen
    letters_map.insert(20, 6); // twenty
    letters_map.insert(30, 6); // thirty
    letters_map.insert(40, 5); // forty
    letters_map.insert(50, 5); // fifty
    letters_map.insert(60, 5); // sixty
    letters_map.insert(70, 7); // seventy
    letters_map.insert(80, 6); // eighty
    letters_map.insert(90, 6); // ninety

    // one // 3
    // forty-five // 9
    // ninety // 6
    // ninety-nine // 10
    // one hundred and five // 17
    // nine hundred and seventy-seven // 26
    // nine hundred and ninety // 20
    // nine hundred and ninety-nine // 24
    // one thousand // 11

    let mut sum = 0;

    for n in options.from..(options.to + 1) {
        let letters = count_letters(n, &mut letters_map) as usize;
        println!("{} has {} letters", n, letters);
        sum += letters;
    }

    Ok(sum)
}

fn count_letters(n: u16, letters_map: &mut HashMap<u16, u8>) -> u8 {
    let mut letters = 0;

    let ones = n % 10;
    let tens = (n - ones) % 100;
    let hundreds = (n - tens - ones) % 1000;

    if tens < 20 && (ones != 0 || tens != 0) {
        letters += *letters_map.get(&(n % 100)).unwrap(); // add ones/teens (ie. "nineteen")
    } else {
        if ones != 0 {
            letters += *letters_map.get(&ones).unwrap(); // add ones digit (ie. twenty-"one")
        }
        if tens != 0 {
            letters += *letters_map.get(&tens).unwrap(); // add tens (ie. "twenty"-one)
        }
    }

    if hundreds != 0 {
        letters += *letters_map.get(&(hundreds / 100)).unwrap(); // add hundreds (ie "four" hundred)
        letters += 7; // add 7 for "hundred" (ie. four "hundred")

        if ones != 0 || tens != 0 {
            letters += 3; // add 3 for "and" (ie. four hundred "and" twenty-one)
        }
    }

    if n >= 1000 {
        letters += count_letters(n / 1000, letters_map); // add thousands (ie. "one hundred and twenty-three" thousand)
        letters += 8; // add 8 for "thousand" (ie. one hundred and twenty-three "thousand")
    }

    letters
}
