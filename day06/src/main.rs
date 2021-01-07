use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use ahash::AHashSet;

fn part1(lines: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut current = AHashSet::new();
    let mut question_sum = 0;
    for line_ref in lines {
        let line = line_ref.as_ref();
        if line.is_empty() {
            question_sum += current.len();
            current.clear();
        } else {
            for c in line.chars() {
                current.insert(c);
            }
        }
    }

    question_sum + current.len()
}

fn part2(lines: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut current = AHashSet::new();
    let mut question_sum = 0;
    let mut is_first = true;
    for line_ref in lines {
        let line = line_ref.as_ref();
        if line.is_empty() {
            question_sum += current.len();
            current.clear();
            is_first = true;
        } else if is_first {
            for c in line.chars() {
                current.insert(c);
            }

            is_first = false;
        } else {
            current.retain(|&c| line.contains(c));
        }
    }

    question_sum + current.len()
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = {
        let path = ["data", "day06", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        BufReader::new(file)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
    };

    println!("Part 1: sum = {}", part1(lines.iter()));
    println!("Part 2: sum = {}", part2(lines.iter()));

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
    use super::{part1, part2};

    const EXAMPLE: &str = r"abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn sum_test() {
        let result = part1(EXAMPLE.lines());
        assert_eq!(result, 11);
    }

    #[test]
    fn all_test() {
        let result = part2(EXAMPLE.lines());
        assert_eq!(result, 6);
    }
}
