use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug)]
enum Day5Error {
    NoData,
    NotFound,
}

impl fmt::Display for Day5Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Day5Error::NoData => write!(f, "No data"),
            Day5Error::NotFound => write!(f, "Result not found"),
        }
    }
}

impl Error for Day5Error {}

fn calculate_id(pass: &str) -> i32 {
    pass.chars().fold(0, |acc, c| {
        (acc << 1)
            + match c {
                'B' | 'R' => 1,
                _ => 0,
            }
    })
}

fn part1(lines: impl Iterator<Item = impl AsRef<str>>) -> Result<(), Day5Error> {
    let max_value = lines
        .map(|l| calculate_id(l.as_ref()))
        .max()
        .ok_or(Day5Error::NoData)?;
    println!("Part 1: maximum ID = {}", max_value);
    Ok(())
}

fn part2(lines: impl Iterator<Item = impl AsRef<str>>) -> Result<(), Day5Error> {
    let mut ids = lines.map(|l| calculate_id(l.as_ref())).collect::<Vec<_>>();
    ids.sort_unstable();
    let pair = ids
        .windows(2)
        .find(|&pair| pair[1] - pair[0] == 2)
        .ok_or(Day5Error::NotFound)?;
    println!("Part 2, found empty seat at {}", pair[0] + 1);
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = {
        let path = ["data", "day05", "input.txt"].iter().collect::<PathBuf>();
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

#[cfg(test)]
mod test {
    use super::calculate_id;

    const EXAMPLE_IDS: [(&str, i32); 4] = [
        ("FBFBBFFRLR", 357),
        ("BFFFBBFRRR", 567),
        ("FFFBBBFRRR", 119),
        ("BBFFBBFRLL", 820),
    ];

    #[test]
    fn parse_test() {
        for &(pass, expected_id) in &EXAMPLE_IDS {
            let actual_id = calculate_id(pass);
            assert_eq!(actual_id, expected_id);
        }
    }
}
