use std::{
    cmp::Reverse,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn count_differences(source: &[i32]) -> usize {
    let mut adapters = Vec::from(source);
    adapters.sort();
    let (_, count1, count3) =
        adapters
            .iter()
            .copied()
            .fold((0, 0, 1), |(previous, count1, count3), i| {
                match i - previous {
                    1 => (i, count1 + 1, count3),
                    3 => (i, count1, count3 + 1),
                    _ => (i, count1, count3),
                }
            });
    count1 * count3
}

fn count_ways(source: &[i32]) -> u64 {
    let mut adapters = Vec::from(source);
    adapters.sort_by_key(|a| Reverse(*a));
    adapters.push(0);
    let mut scores = Vec::with_capacity(adapters.len());
    scores.push(1);
    adapters.iter().skip(1).copied().for_each(|adapter| {
        let score = adapters
            .iter()
            .copied()
            .zip(scores.iter().copied())
            .rev()
            .take_while(|(a, _)| *a - adapter <= 3)
            .map(|(_, s)| s)
            .sum();
        scores.push(score);
    });

    *scores.last().unwrap()
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = ["data", "day10", "input.txt"].iter().collect::<PathBuf>();
    let file = File::open(path)?;
    let mut adapters = BufReader::new(file)
        .lines()
        .map(|l| {
            l.map_err(Box::<dyn Error>::from)
                .and_then(|s| s.parse().map_err(Box::<dyn Error>::from))
        })
        .collect::<Result<Vec<_>, _>>()?;

    adapters.sort();

    println!("Part 1: result = {}", count_differences(&adapters));
    println!("Part 2: result = {}", count_ways(&adapters));

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
    use super::{count_differences, count_ways};

    const EXAMPLE1: [i32; 11] = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

    const EXAMPLE2: [i32; 31] = [
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];

    #[test]
    fn test_part1_small() {
        let result = count_differences(&EXAMPLE1);
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part1_large() {
        let result = count_differences(&EXAMPLE2);
        assert_eq!(result, 220);
    }

    #[test]
    fn test_part2_small() {
        let result = count_ways(&EXAMPLE1);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_large() {
        let result = count_ways(&EXAMPLE2);
        assert_eq!(result, 19208);
    }
}
