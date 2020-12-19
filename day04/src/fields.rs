use ahash::AHashSet;

pub const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn check_valid(prefixes: &AHashSet<String>) -> bool {
    REQUIRED_FIELDS
        .iter()
        .all(|f| prefixes.contains(f.to_owned()))
}
