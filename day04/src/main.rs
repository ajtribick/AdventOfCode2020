use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use ahash::AHashSet;

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn check_valid(prefixes: &AHashSet<String>) -> bool {
    REQUIRED_FIELDS
        .iter()
        .all(|f| prefixes.contains(f.to_owned()))
}

fn value_ok(prefix: &str, value: &str) -> bool {
    match prefix {
        "byr" => value.len() == 4 && (1920..=2002).contains(&value.parse::<i32>().unwrap_or(0)),
        "iyr" => value.len() == 4 && (2010..=2020).contains(&value.parse::<i32>().unwrap_or(0)),
        "eyr" => value.len() == 4 && (2020..=2030).contains(&value.parse::<i32>().unwrap_or(0)),
        "hgt" => {
            if let Some(height) = value.strip_suffix("cm") {
                (150..=193).contains(&height.parse::<i32>().unwrap_or(0))
            } else if let Some(height) = value.strip_suffix("in") {
                (59..=76).contains(&height.parse::<i32>().unwrap_or(0))
            } else {
                false
            }
        }
        "hcl" => {
            if value.len() == 7 {
                let mut chars = value.chars();
                chars.next().unwrap() == '#' && chars.all(|c| "0123456789abcdef".contains(c))
            } else {
                false
            }
        }
        "ecl" => EYE_COLORS.contains(&value),
        "pid" => value.len() == 9 && value.chars().all(|c| "0123456789".contains(c)),
        _ => false,
    }
}

fn count_valid<S, L, F>(lines: L, check: F) -> usize
where
    S: AsRef<str>,
    L: Iterator<Item = S>,
    F: Fn(&str, &str) -> bool,
{
    let mut valid = 0;
    let mut prefixes = AHashSet::new();
    for line_ref in lines {
        let line = line_ref.as_ref();
        if line.is_empty() {
            if check_valid(&prefixes) {
                valid += 1;
            }

            prefixes.clear();
        } else {
            for field in line.split(' ') {
                if let Some(pos) = field.find(':') {
                    let (prefix, value) = field.split_at(pos);
                    if check(prefix, &value[1..]) {
                        prefixes.insert(prefix.to_owned());
                    }
                }
            }
        }
    }

    if check_valid(&prefixes) {
        valid + 1
    } else {
        valid
    }
}

fn part1(lines: impl Iterator<Item = impl AsRef<str>>) {
    println!(
        "Part 1: found {} valid passports",
        count_valid(lines, |_, _| true)
    );
}

pub fn part2(lines: impl Iterator<Item = impl AsRef<str>>) {
    println!(
        "Part 2: found {} valid passports",
        count_valid(lines, value_ok)
    );
}

fn run() -> Result<(), Box<dyn Error>> {
    let lines = {
        let path = ["data", "day04", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        BufReader::new(file)
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
    });
}

#[cfg(test)]
mod test {
    use super::{count_valid, value_ok};

    #[test]
    fn part1() {
        const EXAMPLE: [&str; 13] = [
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ];
        let valid = count_valid(EXAMPLE.iter(), |_, _| true);
        assert_eq!(valid, 2);
    }

    #[test]
    fn part2_invalid() {
        const EXAMPLES_INVALID: [&str; 13] = [
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
        ];
        let valid = count_valid(EXAMPLES_INVALID.iter(), value_ok);
        assert_eq!(valid, 0);
    }

    #[test]
    fn part2_valid() {
        const EXAMPLES_VALID: [&str; 12] = [
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ];
        let valid = count_valid(EXAMPLES_VALID.iter(), value_ok);
        assert_eq!(valid, 4);
    }
}
