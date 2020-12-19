use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod fields;
mod part1;
mod part2;

use part1::part1;
use part2::part2;

fn run() -> Result<(), Box<dyn Error>> {
    let lines = {
        let path = ["data", "day04", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
    };

    part1(lines.iter());
    part2(lines.iter());

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
