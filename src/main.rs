mod problems;

use problems::problem_1;

pub type Result<Type = ()> = std::result::Result<Type, String>;

fn main() -> Result {
    let problem = std::env::args()
        .skip(1)
        .take(1)
        .find(|_| true)
        .ok_or_else(|| "Please pass in a problem to run. ie. cargo run --release -- 1".to_string())?
        .parse::<usize>()
        .or_else(|_| {
            Err("Please pass in a problem to run. ie: cargo run --release -- 1".to_string())
        })?;

    let result = match problem {
        1 => problem_1::solve(problem_1::Options::default()),
        _ => Err(format!("Unknown problem: {}", problem)),
    };

    println!("{}", result?);

    Ok(())
}
