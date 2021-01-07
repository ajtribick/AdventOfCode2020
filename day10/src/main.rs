use std::{
    cmp::Reverse,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn count_differences(source: &[i32]) -> usize {
    let mut adapters = source.to_vec();
    adapters.sort_unstable();
    let mut previous = 0;
    let mut count1 = 0;
    let mut count3 = 1;
    for adapter in adapters {
        match adapter - previous {
            1 => count1 += 1,
            3 => count3 += 1,
            _ => (),
        }

        previous = adapter;
    }

    count1 * count3
}

fn count_ways(source: &[i32]) -> u64 {
    let mut adapters = source.to_vec();
    adapters.sort_unstable_by_key(|&a| Reverse(a));
    adapters.push(0);
    let mut scores = Vec::with_capacity(adapters.len());
    scores.push(1);
    for &adapter in &adapters[1..] {
        let score = adapters
            .iter()
            .copied()
            .zip(scores.iter().copied())
            .rev()
            .take_while(|(a, _)| *a - adapter <= 3)
            .map(|(_, s)| s)
            .sum();
        scores.push(score);
    }

    *scores.last().unwrap()
}

fn run() -> Result<(), Box<dyn Error>> {
    let adapters = {
        let path = ["data", "day10", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        let mut adapters = Vec::new();
        for line in BufReader::new(file).lines() {
            adapters.push(line?.parse()?);
        }

        adapters
    };

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
