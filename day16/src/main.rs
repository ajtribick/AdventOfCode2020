use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod problem;

use problem::Problem;

fn run() -> Result<(), Box<dyn Error>> {
    let problem = {
        let path = ["data", "day16", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        Problem::parse(BufReader::new(file).lines().filter_map(Result::ok))?
    };

    println!("Part 1: rate = {}", problem.error_rate());

    let field_assignments = problem.assign_fields();
    let ticket = problem.your_ticket();
    let result = problem
        .fields()
        .iter()
        .enumerate()
        .filter(|(_, f)| f.name().starts_with("departure"))
        .map(|(i, _)| ticket[field_assignments[i]] as u64)
        .product::<u64>();

    println!("Part 2: result = {}", result);

    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            1
        }
    });
}
