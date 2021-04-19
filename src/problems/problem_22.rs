use super::{
    utils::{read_file_values, ReadFileOptions},
    Result,
};

/*
Problem 22:
Using resources/p022_names.txt, a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?

Answer: 871198282
*/

pub struct Options {
    file: String,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            file: "resources/p022_names.txt".to_string(),
        }
    }
}

pub fn solve(options: Options) -> Result<usize> {
    let mut names = read_file_values(
        options.file,
        ReadFileOptions {
            delimiter: ",".to_string(),
            padding: "\"".to_string(),
            ..Default::default()
        },
    )?;

    names.sort();

    Ok(names
        .into_iter()
        .enumerate()
        .map(|(i, name)| name_score(name, i + 1))
        .sum())
}

fn name_score(name: String, position: usize) -> usize {
    const OFFSET: usize = 'A' as usize - 1;

    let value = name
        .to_ascii_uppercase()
        .bytes()
        .map(|b| b as usize - OFFSET)
        .sum::<usize>();

    value * position
}
