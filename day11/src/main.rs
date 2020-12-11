use std::{error::Error, fs::read_to_string, path::PathBuf};

mod error;
mod seating;

use seating::SeatingPlan;

fn part1(mut plan: SeatingPlan) {
    while plan.update() {}
    println!("Part 1: occupied = {}", plan.occupied());
}

fn part2(mut plan: SeatingPlan) {
    while plan.update2() {}
    println!("Part 2: occupied = {}", plan.occupied());
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = ["data", "day11", "input.txt"].iter().collect::<PathBuf>();
    let plan = read_to_string(path)?.parse::<SeatingPlan>()?;
    part1(plan.clone());
    part2(plan);
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
