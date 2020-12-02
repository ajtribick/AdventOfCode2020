use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{error::Error, path::PathBuf};

fn count_valid<'a, T: AsRef<str>>(lines: impl Iterator<Item = T>) -> usize {
    lines
        .into_iter()
        .filter(|s| {
            let parts = s
                .as_ref()
                .split(|c| match c {
                    '-' | ' ' | ':' => true,
                    _ => false,
                })
                .collect::<Vec<_>>();
            let min = parts[0].parse::<usize>().unwrap();
            let max = parts[1].parse::<usize>().unwrap();
            let character = parts[2].chars().next().unwrap();
            let pwd = parts[4];
            let occurrence = pwd
                .chars()
                .filter(|&c| c == character)
                .take(max + 1)
                .count();
            (min..=max).contains(&occurrence)
        })
        .count()
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::new();
    ["data", "day2", "input.txt"]
        .iter()
        .for_each(|p| path.push(p));
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines().filter_map(Result::ok);
    let valid_count = count_valid(lines);
    println!("Found {} valid passwords", valid_count);
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
    use super::count_valid;

    #[test]
    fn test1() {
        let list = ["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let count = count_valid(list.iter());
        assert_eq!(count, 2);
    }
}
