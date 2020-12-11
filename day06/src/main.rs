use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn part1(lines: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut current = HashSet::new();
    let result = lines.fold(0, |acc, l| match l.as_ref() {
        "" => {
            let group_len = current.len();
            current.clear();
            acc + group_len
        }
        line => {
            line.chars().for_each(|c| {
                current.insert(c);
            });
            acc
        }
    });
    result + current.len()
}

fn part2(lines: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut current = HashSet::new();
    let result = lines
        .fold((0, true), |(total, is_start), l| match l.as_ref() {
            "" => {
                let group_len = current.len();
                current.clear();
                (total + group_len, true)
            }
            line => {
                if is_start {
                    line.chars().for_each(|c| {
                        current.insert(c);
                    });
                } else {
                    current.retain(|&c| line.contains(c));
                }
                (total, false)
            }
        })
        .0;
    result + current.len()
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = ["data", "day06", "input.txt"].iter().collect::<PathBuf>();
    let file = File::open(path)?;
    let lines = BufReader::new(file)
        .lines()
        .collect::<Result<Vec<_>, _>>()?;

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
