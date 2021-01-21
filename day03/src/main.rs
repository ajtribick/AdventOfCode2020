use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

fn count_trees(
    lines: impl Iterator<Item = impl AsRef<str>>,
    right_step: usize,
    down_step: usize,
) -> u32 {
    let mut pos = 0;
    let mut trees = 0;
    for line_ref in lines.step_by(down_step) {
        let line = line_ref.as_ref();
        if line.as_bytes()[pos] == b'#' {
            trees += 1;
        }

        pos = (pos + right_step) % line.len();
    }

    trees
}

fn part1(lines: impl Iterator<Item = impl AsRef<str>>) {
    let trees = count_trees(lines, 3, 1);
    println!("Part 1: encountered {} trees", trees);
}

fn part2(lines: impl Iterator<Item = impl AsRef<str>> + Clone) {
    let result = SLOPES
        .iter()
        .map(|&(right_step, down_step)| count_trees(lines.clone(), right_step, down_step))
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
    part2(lines.iter());
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
        let iter = EXAMPLE_LAYOUT.iter();
        let trees = SLOPES
            .iter()
            .map(|&(right_step, down_step)| count_trees(iter.clone(), right_step, down_step))
            .collect::<Vec<_>>();

        let expected = [2, 7, 3, 4, 2];

        assert_eq!(trees, expected);

        assert_eq!(trees.iter().product::<u32>(), 336);
    }
}
