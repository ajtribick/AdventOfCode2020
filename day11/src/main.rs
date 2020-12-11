use std::{error::Error, fs::read_to_string, path::PathBuf};

mod error;
mod seating;

use seating::SeatingPlan;

fn run() -> Result<(), Box<dyn Error>> {
    let path = ["data", "day11", "input.txt"].iter().collect::<PathBuf>();
    let mut plan = read_to_string(path)?.parse::<SeatingPlan>()?;
    while plan.update() {}
    println!("Part 1: occupied = {}", plan.occupied());
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
