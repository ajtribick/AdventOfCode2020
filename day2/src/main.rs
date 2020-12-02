use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
enum Day2Error {
    IoError(io::Error),
    ParseError,
}

impl fmt::Display for Day2Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Day2Error::IoError(e) => write!(f, "IO error {}", e),
            Day2Error::ParseError => write!(f, "Parse error"),
        }
    }
}

impl Error for Day2Error {}

#[derive(Debug, PartialEq)]
struct LineInfo {
    pub min: usize,
    pub max: usize,
    pub character: char,
    pub password: String,
}

fn iter_parse<'a, T: FromStr>(iter: &mut impl Iterator<Item = &'a str>) -> Result<T, Day2Error> {
    iter.next()
        .map(|s| s.parse::<T>().ok())
        .flatten()
        .ok_or(Day2Error::ParseError)
}

fn split_line(line: impl AsRef<str>) -> Result<LineInfo, Day2Error> {
    let mut parts = line
        .as_ref()
        .split(|c| match c {
            '-' | ' ' | ':' => true,
            _ => false,
        })
        .filter(|&s| !s.is_empty());
    let min = iter_parse(&mut parts)?;
    let max = iter_parse(&mut parts)?;
    let character = iter_parse(&mut parts)?;
    let password = parts.next().ok_or(Day2Error::ParseError)?.into();
    match parts.next() {
        None => Ok(LineInfo {
            min,
            max,
            character,
            password,
        }),
        _ => Err(Day2Error::ParseError),
    }
}

fn count_valid<'a>(parsed_lines: impl Iterator<Item = &'a LineInfo>) -> Result<usize, Day2Error> {
    Ok(parsed_lines
        .filter(|&line_info| {
            let occurrence = line_info
                .password
                .chars()
                .filter(|&c| c == line_info.character)
                .take(line_info.max + 1)
                .count();
            (line_info.min..=line_info.max).contains(&occurrence)
        })
        .count())
}

fn count_valid2<'a>(parsed_lines: impl Iterator<Item = &'a LineInfo>) -> Result<usize, Day2Error> {
    Ok(parsed_lines
        .filter(|&line_info| {
            let mut password_chars = line_info.password.chars();
            let first_ok = password_chars
                .nth(line_info.min - 1)
                .map_or(false, |c| c == line_info.character);
            let second_ok = password_chars
                .nth(line_info.max - line_info.min - 1)
                .map_or(false, |c| c == line_info.character);
            first_ok ^ second_ok
        })
        .count())
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::new();
    ["data", "day2", "input.txt"]
        .iter()
        .for_each(|&p| path.push(p));
    let file = File::open(path)?;
    let parsed_lines = BufReader::new(file)
        .lines()
        .map(|l| l.map_err(Day2Error::IoError).and_then(split_line))
        .collect::<Result<Vec<_>, _>>()?;

    let part1 = count_valid(parsed_lines.iter())?;
    println!("Part 1: found {} valid passwords", part1);
    let part2 = count_valid2(parsed_lines.iter())?;
    println!("Part 2: found {} valid passwords", part2);
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
    use super::{count_valid, count_valid2, split_line, LineInfo};
    use std::error::Error;

    const TEST_DATA: [&str; 3] = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

    fn create_test_info() -> Vec<LineInfo> {
        vec![
            LineInfo {
                min: 1,
                max: 3,
                character: 'a',
                password: String::from("abcde"),
            },
            LineInfo {
                min: 1,
                max: 3,
                character: 'b',
                password: String::from("cdefg"),
            },
            LineInfo {
                min: 2,
                max: 9,
                character: 'c',
                password: String::from("ccccccccc"),
            },
        ]
    }

    #[test]
    fn parse_test() -> Result<(), Box<dyn Error>> {
        let parse_result = TEST_DATA
            .iter()
            .map(|&s| split_line(s))
            .collect::<Result<Vec<_>, _>>()?;
        assert!(parse_result.iter().eq(create_test_info().iter()));
        Ok(())
    }

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        let count = count_valid(create_test_info().iter())?;
        assert_eq!(count, 2);
        Ok(())
    }

    #[test]
    fn test2() -> Result<(), Box<dyn Error>> {
        let count = count_valid2(create_test_info().iter())?;
        assert_eq!(count, 1);
        Ok(())
    }
}
