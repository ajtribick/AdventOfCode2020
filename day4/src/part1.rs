use crate::fields::check_valid;
use std::collections::HashSet;

fn count_valid<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> usize {
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
                    .map(|s| s.split(':').next().unwrap_or(""))
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

pub fn part1(lines: impl Iterator<Item = impl AsRef<str>>) {
    println!("Part 1: found {} valid passports", count_valid(lines));
}

#[cfg(test)]
mod test {
    use super::count_valid;

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

    #[test]
    fn part1() {
        let valid = count_valid(EXAMPLE.iter());
        assert_eq!(valid, 2);
    }
}
