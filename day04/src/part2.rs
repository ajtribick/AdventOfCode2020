use crate::fields::check_valid;
use std::collections::HashSet;

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn value_ok<'a>(prefix: &'a str, value: &'a str) -> bool {
    match prefix {
        "byr" => value.len() == 4 && (1920..=2002).contains(&value.parse::<i32>().unwrap_or(0)),
        "iyr" => value.len() == 4 && (2010..=2020).contains(&value.parse::<i32>().unwrap_or(0)),
        "eyr" => value.len() == 4 && (2020..=2030).contains(&value.parse::<i32>().unwrap_or(0)),
        "hgt" => {
            if value.ends_with("cm") {
                (150..=193).contains(&value[..value.len() - 2].parse::<i32>().unwrap_or(0))
            } else if value.ends_with("in") {
                (59..=76).contains(&value[..value.len() - 2].parse::<i32>().unwrap_or(0))
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

fn count_valid(lines: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let (num_valid, last_item) =
        lines.fold((0, None), |(valid, current), line| match line.as_ref() {
            "" => match current {
                Some(set) if check_valid(&set) => (valid + 1, None),
                _ => (valid, None),
            },
            item_str => {
                let mut updated = current.unwrap_or(HashSet::new());
                item_str
                    .split(' ')
                    .map(|s| {
                        let (prefix, value) = s.split_at(s.find(':').unwrap());
                        if value_ok(prefix, &value[1..]) {
                            prefix
                        } else {
                            ""
                        }
                    })
                    .for_each(|s| {
                        updated.insert(String::from(s));
                    });
                (valid, Some(updated))
            }
        });

    match last_item {
        Some(set) if check_valid(&set) => num_valid + 1,
        _ => num_valid,
    }
}

pub fn part2(lines: impl Iterator<Item = impl AsRef<str>>) {
    println!("Part 2: found {} valid passports", count_valid(lines));
}

#[cfg(test)]
mod test {
    use super::count_valid;

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

    #[test]
    fn check_invalid() {
        let valid = count_valid(EXAMPLES_INVALID.iter());
        assert_eq!(valid, 0);
    }

    #[test]
    fn check_valid() {
        let valid = count_valid(EXAMPLES_VALID.iter());
        assert_eq!(valid, 4);
    }
}
