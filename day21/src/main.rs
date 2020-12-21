use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod food;

use food::FoodProcessor;

fn run() -> Result<(), Box<dyn Error>> {
    let path = ["data", "day21", "input.txt"].iter().collect::<PathBuf>();
    let file = File::open(path)?;
    let processor = FoodProcessor::parse(BufReader::new(file).lines().filter_map(Result::ok))?;
    println!("Part 1: result = {}", processor.safe_count());
    println!("Part 2: result = {}", processor.map_allergens());
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
