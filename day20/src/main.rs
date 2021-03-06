use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[macro_use]
extern crate lazy_static;

mod grid;
mod tile;
mod utils;

use grid::Grid;

fn run() -> Result<(), Box<dyn Error>> {
    let grid = {
        let path = ["data", "day20", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        Grid::parse(BufReader::new(file).lines().filter_map(Result::ok))?
    };

    println!(
        "Part 1: result = {}",
        grid.corner_ids().iter().product::<u64>()
    );

    let mut merged = grid.merge_tiles();
    merged.remove_monsters();
    println!("Part 2: rougness = {}", merged.roughness());

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
