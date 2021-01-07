use std::{
    cmp::{Ord, Ordering},
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Add, AddAssign},
    path::PathBuf,
};

#[derive(Debug)]
struct NotFoundError {}

impl fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not found")
    }
}

impl Error for NotFoundError {}

fn find_pair<T>(numbers: &[T], target: T) -> Option<(T, T)>
where
    T: Add<Output = T> + Copy + Ord,
{
    let mut it = numbers.iter();

    let mut low = *it.next()?;
    let mut high = *it.next_back()?;

    loop {
        let total = low + high;
        match total.cmp(&target) {
            Ordering::Equal => return Some((low, high)),
            Ordering::Less => {
                low = *it.next()?;
            }
            Ordering::Greater => {
                high = *it.next_back()?;
            }
        }
    }
}

fn find_incorrect<T>(sequence: &[T], preamble_size: usize) -> Option<T>
where
    T: Add<Output = T> + Copy + Default + Ord,
{
    let mut preamble = vec![Default::default(); preamble_size];
    sequence
        .windows(preamble_size + 1)
        .filter_map(|w| {
            let target = *w.last().unwrap();
            preamble.copy_from_slice(&w[0..preamble_size]);
            preamble.sort_unstable();
            match find_pair(&preamble, target) {
                Some(_) => None,
                None => Some(target),
            }
        })
        .next()
}

fn part1(sequence: &[i64]) -> Result<i64, NotFoundError> {
    let result = find_incorrect(sequence, 25).ok_or(NotFoundError {})?;
    println!("Part1: result = {}", result);
    Ok(result)
}

fn find_contiguous<T>(sequence: &[T], target: T) -> Option<T>
where
    T: Add<Output = T> + AddAssign<T> + Copy + Ord,
{
    let mut subsequence = sequence;
    while !subsequence.is_empty() {
        let mut sum = subsequence[0];
        let mut min_element = sum;
        let mut max_element = sum;
        for &element in &subsequence[1..] {
            if sum >= target {
                break;
            }

            sum += element;
            min_element = std::cmp::min(min_element, element);
            max_element = std::cmp::max(max_element, element);
        }

        if sum == target {
            return Some(min_element + max_element);
        }

        subsequence = &subsequence[1..];
    }

    None
}

fn part2(sequence: &[i64], target: i64) -> Result<(), NotFoundError> {
    let result = find_contiguous(sequence, target).ok_or(NotFoundError {})?;
    println!("Part 2: result = {}", result);
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let source = {
        let path = ["data", "day09", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        let mut source = Vec::new();
        for line in BufReader::new(file).lines() {
            source.push(line?.parse()?);
        }

        source
    };
    let target = part1(&source)?;
    part2(&source, target)?;
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
    use super::{find_contiguous, find_incorrect};

    const EXAMPLE_SEQUENCE: [i32; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn part1_test() {
        let result = find_incorrect(&EXAMPLE_SEQUENCE, 5).unwrap();
        assert_eq!(result, 127);
    }

    #[test]
    fn part2_test() {
        let result = find_contiguous(&EXAMPLE_SEQUENCE, 127).unwrap();
        assert_eq!(result, 62);
    }
}
