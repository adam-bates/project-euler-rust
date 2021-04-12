mod problems;

use problems::problem_1;
use problems::problem_2;
use problems::problem_3;
use problems::problem_4;
use problems::problem_5;
use problems::problem_6;
use problems::problem_7;
use problems::problem_8;
use std::env;

pub type Result<Type = ()> = std::result::Result<Type, String>;

fn parse_problem_from_args(args: env::Args) -> Result<usize> {
    args.skip(1)
        .take(1)
        .find(|_| true)
        .ok_or_else(|| "Please pass in a problem to run. ie. cargo run --release -- 1".to_string())?
        .parse::<usize>()
        .or_else(|_| {
            Err("Please pass in a problem to run. ie: cargo run --release -- 1".to_string())
        })
}

fn main() -> Result {
    let problem = parse_problem_from_args(env::args())?;

    let result = match problem {
        1 => problem_1::solve(problem_1::Options::default()),
        2 => problem_2::solve(problem_2::Options::default()),
        3 => problem_3::solve(problem_3::Options::default()),
        4 => problem_4::solve(problem_4::Options::default()),
        5 => problem_5::solve(problem_5::Options::default()),
        6 => problem_6::solve(problem_6::Options::default()),
        7 => problem_7::solve(problem_7::Options::default()),
        8 => problem_8::solve(problem_8::Options::default()),
        _ => Err(format!("Unknown problem: {}", problem)),
    };

    println!("{}", result?);

    Ok(())
}
