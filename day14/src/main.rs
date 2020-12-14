use std::{
    error::Error,
    fs::read_to_string,
    path::PathBuf,
};

mod error;
mod part1;

use part1::part1;

fn run() -> Result<(), Box<dyn Error>> {
    let path = ["data", "day14", "input.txt"].iter().collect::<PathBuf>();
    let code = read_to_string(path)?;

    part1(&code)?;

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
