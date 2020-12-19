use std::{error::Error, fs::read_to_string, path::PathBuf};

mod simulation;
use simulation::{ParseSimulationError, Simulation};

fn process(initial: &str, dimensions: usize) -> Result<usize, ParseSimulationError> {
    let mut simulation = Simulation::parse(initial, dimensions)?;
    for _ in 0..6 {
        simulation.update();
    }

    Ok(simulation.active_count())
}

fn run() -> Result<(), Box<dyn Error>> {
    let initial = {
        let path = ["data", "day17", "input.txt"].iter().collect::<PathBuf>();
        read_to_string(path)?
    };
    println!("Part 1: result = {}", process(&initial, 3)?);
    println!("Part 2: result = {}", process(&initial, 4)?);
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
