use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod parser;

use parser::{parse, ParseError};

fn part1(lines: impl Iterator<Item = impl AsRef<str>>) -> Result<(), ParseError> {
    let mut result = 0;
    for line in lines {
        result += parse(line.as_ref(), false)?;
    }

    println!("Part 1: result = {}", result);
    Ok(())
}

fn part2(lines: impl Iterator<Item = impl AsRef<str>>) -> Result<(), ParseError> {
    let mut result = 0;
    for line in lines {
        result += parse(line.as_ref(), true)?;
    }

    println!("Part 2: result = {}", result);
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = {
        let path = ["data", "day18", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
    };

    part1(lines.iter())?;
    part2(lines.iter())?;

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
