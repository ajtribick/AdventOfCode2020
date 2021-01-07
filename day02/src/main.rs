use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use nom::Finish;

#[derive(Debug)]
struct ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error")
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq)]
struct LineInfo {
    pub min: usize,
    pub max: usize,
    pub character: char,
    pub password: String,
}

mod line_parsing {
    use super::LineInfo;

    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, anychar, char, digit1},
        combinator::{map, map_res},
        sequence::separated_pair,
        IResult,
    };

    fn min_max(input: &str) -> IResult<&str, (usize, usize)> {
        separated_pair(
            map_res(digit1, str::parse),
            char('-'),
            map_res(digit1, str::parse),
        )(input)
    }

    fn min_max_char(input: &str) -> IResult<&str, (usize, usize, char)> {
        map(
            separated_pair(min_max, char(' '), anychar),
            |((min, max), character)| (min, max, character),
        )(input)
    }

    pub(super) fn line_parser(input: &str) -> IResult<&str, LineInfo> {
        map(
            separated_pair(min_max_char, tag(": "), alpha1),
            |((min, max, character), password)| LineInfo {
                min,
                max,
                character,
                password: password.to_owned(),
            },
        )(input)
    }
}

fn split_line(line: impl AsRef<str>) -> Result<LineInfo, ParseError> {
    line_parsing::line_parser(line.as_ref())
        .finish()
        .map_or(Err(ParseError {}), |(_, li)| Ok(li))
}

fn count_valid<'a>(parsed_lines: impl Iterator<Item = &'a LineInfo>) -> usize {
    parsed_lines
        .filter(|&line_info| {
            let occurrence = line_info
                .password
                .chars()
                .filter(|&c| c == line_info.character)
                .take(line_info.max + 1)
                .count();
            (line_info.min..=line_info.max).contains(&occurrence)
        })
        .count()
}

fn count_valid2<'a>(parsed_lines: impl Iterator<Item = &'a LineInfo>) -> usize {
    parsed_lines
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
        .count()
}

fn run() -> Result<(), Box<dyn Error>> {
    let parsed_lines = {
        let path = ["data", "day02", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        let mut parsed_lines = Vec::new();
        for line in BufReader::new(file).lines() {
            parsed_lines.push(split_line(line?)?);
        }

        parsed_lines
    };

    let part1 = count_valid(parsed_lines.iter());
    println!("Part 1: found {} valid passwords", part1);
    let part2 = count_valid2(parsed_lines.iter());
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
    fn parse_test() {
        let parse_result = TEST_DATA
            .iter()
            .map(|&s| split_line(s).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(parse_result, create_test_info());
    }

    #[test]
    fn test1() {
        let count = count_valid(create_test_info().iter());
        assert_eq!(count, 2);
    }

    #[test]
    fn test2() {
        let count = count_valid2(create_test_info().iter());
        assert_eq!(count, 1);
    }
}
