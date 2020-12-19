use std::io::{self, BufRead, BufReader};
use std::{fs::File, path::PathBuf};

const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn count_trees(
    lines: impl Iterator<Item = impl AsRef<str>>,
    right_step: usize,
    down_step: usize,
) -> u32 {
    lines
        .step_by(down_step)
        .fold((0, 0), |(mut trees, mut pos), line| {
            let line_ref = line.as_ref();
            if line_ref.chars().nth(pos).unwrap() == '#' {
                trees += 1;
            }
            pos += right_step;
            if pos >= line_ref.len() {
                pos -= line_ref.len();
            }
            (trees, pos)
        })
        .0
}

fn part1(lines: impl Iterator<Item = impl AsRef<str>>) {
    let trees = count_trees(lines, 3, 1);
    println!("Part 1: encountered {} trees", trees);
}

fn part2(lines: &[impl AsRef<str>]) {
    let result = SLOPES
        .iter()
        .map(|&(right_step, down_step)| count_trees(lines.iter(), right_step, down_step))
        .product::<u32>();
    println!("Part 2: product is {}", result);
}

fn run() -> Result<(), io::Error> {
    let lines = {
        let path = ["data", "day03", "input.txt"].iter().collect::<PathBuf>();
        let input_file = File::open(path)?;
        BufReader::new(input_file)
            .lines()
            .collect::<Result<Vec<_>, _>>()?
    };

    part1(lines.iter());
    part2(&lines);
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error occurred: {}", e);
            1
        }
    })
}

#[cfg(test)]
mod test {
    use super::{count_trees, SLOPES};

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
        let trees = count_trees(EXAMPLE_LAYOUT.iter(), 3, 1);
        assert_eq!(trees, 7);
    }

    #[test]
    fn part2() {
        let trees = SLOPES
            .iter()
            .map(|&(right_step, down_step)| {
                count_trees(EXAMPLE_LAYOUT.iter(), right_step, down_step)
            })
            .collect::<Vec<_>>();

        let expected = [2, 7, 3, 4, 2];

        assert_eq!(trees, expected);

        assert_eq!(trees.iter().product::<u32>(), 336);
    }
}
