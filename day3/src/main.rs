use std::io::{self, BufRead, BufReader};
use std::{fs::File, path::PathBuf};

const RIGHT_STEP: usize = 3;

fn count_trees(lines: impl Iterator<Item = impl AsRef<str>>) -> u32 {
    lines
        .fold((0, 0), |(mut trees, mut pos), line| {
            let line_ref = line.as_ref();
            if line_ref.chars().nth(pos).unwrap() == '#' {
                trees += 1;
            }
            pos += RIGHT_STEP;
            if pos >= line_ref.len() {
                pos -= line_ref.len();
            }
            (trees, pos)
        })
        .0
}

fn part1() -> Result<(), io::Error> {
    let mut path = PathBuf::new();
    ["data", "day3", "input.txt"]
        .iter()
        .for_each(|p| path.push(p));
    let input_file = File::open(path)?;
    let lines = BufReader::new(input_file).lines().filter_map(Result::ok);

    let trees = count_trees(lines);

    println!("Encountered {} trees", trees);

    Ok(())
}

fn main() {
    std::process::exit(match part1() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            1
        }
    })
}

#[cfg(test)]
mod test {
    use super::count_trees;

    const EXAMPLE_LAYOUT: [&str; 11] = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    #[test]
    fn part1() {
        let trees = count_trees(EXAMPLE_LAYOUT.iter());
        assert_eq!(trees, 7);
    }
}
