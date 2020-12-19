use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

mod common;
mod part1;
mod part2;

fn run() -> Result<(), Box<dyn Error>> {
    let program = {
        let path = ["data", "day14", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        BufReader::new(file)
            .lines()
            .map(|line_err| {
                line_err
                    .map_err(Box::<dyn Error>::from)
                    .and_then(|line| line.parse().map_err(Box::<dyn Error>::from))
            })
            .collect::<Result<Vec<_>, _>>()?
    };

    let result1 = part1::execute_program(program.iter());
    println!("Part 1: result = {}", result1);

    let result2 = part2::execute_program(program.iter());
    println!("Part 2: result = {}", result2);

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
