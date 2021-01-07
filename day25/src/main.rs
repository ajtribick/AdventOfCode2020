use std::{error::Error, fmt, fs::read_to_string, path::PathBuf};

#[derive(Debug)]
struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error ({})", self.0)
    }
}

impl Error for ParseError {}

const SUBJECT_NUMBER: u64 = 7;
const ENCRYPTION_SIZE: u64 = 20201227;

fn loop_size(target: u64) -> u64 {
    let mut count = 0;
    let mut value = 1;
    while value != target {
        value = (value * SUBJECT_NUMBER) % ENCRYPTION_SIZE;
        count += 1;
    }

    count
}

fn find_key(first: u64, mut second: u64) -> u64 {
    let mut exponent = loop_size(first);
    if exponent == 0 {
        return 1;
    }
    let mut value = 1;
    while exponent > 1 {
        if exponent & 1 != 0 {
            value = (second * value) % ENCRYPTION_SIZE;
        }
        second = (second * second) % ENCRYPTION_SIZE;
        exponent >>= 1;
    }

    (value * second) % ENCRYPTION_SIZE
}

fn run() -> Result<(), Box<dyn Error>> {
    let (public1, public2) = {
        let path = ["data", "day25", "input.txt"].iter().collect::<PathBuf>();
        let input = read_to_string(path)?;
        let mut values = input.lines().map(|s| s.parse().ok());
        let public1 = values
            .next()
            .flatten()
            .ok_or(ParseError("Missing first number"))?;
        let public2 = values
            .next()
            .flatten()
            .ok_or(ParseError("Missing second number"))?;
        (public1, public2)
    };

    println!("Part 1: encrytion key = {}", find_key(public1, public2));

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
    use super::{find_key, loop_size};

    #[test]
    fn loop_size_test() {
        assert_eq!(loop_size(5764801), 8);
        assert_eq!(loop_size(17807724), 11);
    }

    #[test]
    fn part1_test() {
        let result = find_key(5764801, 17807724);
        assert_eq!(result, 14897079);
    }
}
