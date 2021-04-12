mod problems;
mod utils;

// TODO: Figure out how to avoid this from exploding using macros
use problems::problem_1;
use problems::problem_2;
use problems::problem_3;
use problems::problem_4;
use problems::problem_5;
use problems::problem_6;
use problems::problem_7;
use problems::problem_8;
use problems::problem_9;

use problems::problem_10;
use problems::problem_11;
use problems::problem_12;
use problems::problem_13;
use problems::problem_14;
use problems::problem_15;

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

    // TODO: Figure out how to avoid this from exploding using macros
    match problem {
        1 => println!("{}", problem_1::solve(problem_1::Options::default())?),
        2 => println!("{}", problem_2::solve(problem_2::Options::default())?),
        3 => println!("{}", problem_3::solve(problem_3::Options::default())?),
        4 => println!("{}", problem_4::solve(problem_4::Options::default())?),
        5 => println!("{}", problem_5::solve(problem_5::Options::default())?),
        6 => println!("{}", problem_6::solve(problem_6::Options::default())?),
        7 => println!("{}", problem_7::solve(problem_7::Options::default())?),
        8 => println!("{}", problem_8::solve(problem_8::Options::default())?),
        9 => println!("{}", problem_9::solve(problem_9::Options::default())?),
        10 => println!("{}", problem_10::solve(problem_10::Options::default())?),
        11 => println!("{}", problem_11::solve(problem_11::Options::default())?),
        12 => println!("{}", problem_12::solve(problem_12::Options::default())?),
        13 => println!("{}", problem_13::solve(problem_13::Options::default())?),
        14 => println!("{}", problem_14::solve(problem_14::Options::default())?),
        15 => println!("{}", problem_15::solve(problem_15::Options::default())?),
        _ => return Err(format!("Unknown problem: {}", problem)),
    };

    Ok(())
}
