use std::{
    cmp::Ordering,
    error, fmt,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug)]
enum Day1Error {
    EmptySeq,
    NotFound,
    MultiplyOverflow,
}

impl fmt::Display for Day1Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Day1Error::EmptySeq => write!(f, "No values in sequence."),
            Day1Error::NotFound => write!(f, "No answer found"),
            Day1Error::MultiplyOverflow => write!(f, "Multiplication overflow"),
        }
    }
}

impl error::Error for Day1Error {}

const TARGET: i32 = 2020;

fn get_numbers(path: impl AsRef<Path>) -> io::Result<Vec<i32>> {
    let infile = File::open(path)?;
    let mut numbers = BufReader::new(infile)
        .lines()
        .filter_map(|l| l.map_or(None, |s| s.parse().ok()))
        .collect::<Vec<_>>();
    numbers.sort_unstable();
    Ok(numbers)
}

fn find_pair(numbers: &[i32], target: i32) -> Result<(i32, i32), Day1Error> {
    assert!(numbers.len() >= 2);
    assert!(numbers.windows(2).all(|w| w[0] <= w[1])); // numbers.is_sorted() in unstable

    let mut it = numbers.iter();

    let mut low = *it.next().ok_or(Day1Error::EmptySeq)?;
    let mut high = *it.next_back().ok_or(Day1Error::EmptySeq)?;

    loop {
        let total = low + high;
        match total.cmp(&target) {
            Ordering::Equal => return Ok((low, high)),
            Ordering::Less => {
                low = *it.next().ok_or(Day1Error::NotFound)?;
            }
            Ordering::Greater => {
                high = *it.next_back().ok_or(Day1Error::NotFound)?;
            }
        }
    }
}

fn find_triple(numbers: &[i32], target: i32) -> Result<(i32, i32, i32), Day1Error> {
    assert!(numbers.len() >= 3);
    assert!(numbers.windows(2).all(|w| w[0] <= w[1])); // numbers.is_sorted() in unstable

    numbers[0..numbers.len() - 2]
        .iter()
        .enumerate()
        .filter_map(|(i, &l)| {
            find_pair(&numbers[i + 1..], target - l)
                .map(|(m, h)| (l, m, h))
                .ok()
        })
        .next()
        .ok_or(Day1Error::NotFound)
}

fn part1(numbers: &[i32]) -> Result<(), Day1Error> {
    let (low, high) = find_pair(numbers, TARGET)?;
    let product = low.checked_mul(high).ok_or(Day1Error::MultiplyOverflow)?;
    println!(
        "Part 1: low = {}, high = {}, product = {}",
        low, high, product
    );
    Ok(())
}

fn part2(numbers: &[i32]) -> Result<(), Day1Error> {
    let (low, middle, high) = find_triple(numbers, TARGET)?;

    let product = low
        .checked_mul(middle)
        .map(|lm| lm.checked_mul(high))
        .flatten()
        .ok_or(Day1Error::MultiplyOverflow)?;

    println!(
        "Part 2: low = {}, middle = {}, high = {}, product = {}",
        low, middle, high, product
    );

    Ok(())
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let path = ["data", "day01", "input.txt"].iter().collect::<PathBuf>();
    let numbers = get_numbers(path)?;

    part1(&numbers)?;
    part2(&numbers)?;
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
    use super::{find_pair, find_triple, TARGET};

    const NUMBERS: [i32; 6] = [1721, 979, 366, 299, 675, 1456];

    #[test]
    fn part1_test() {
        let mut numbers = NUMBERS;
        numbers.sort_unstable();
        let (low, high) = find_pair(&numbers, TARGET).unwrap();
        assert_eq!(TARGET, low + high);
        let product = low.checked_mul(high);
        assert_eq!(Some(514579), product);
    }

    #[test]
    fn part2() {
        let mut numbers = NUMBERS;
        numbers.sort_unstable();
        let (low, middle, high) = find_triple(&numbers, TARGET).unwrap();
        let sum = low + middle + high;
        assert_eq!(TARGET, sum);
        let product = low * middle * high;
        assert_eq!(241861950, product);
    }
}
