use ahash::AHashMap;
use nom::Finish;
use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone)]
pub enum ParseRule {
    Character(char),
    Sequence(Vec<u32>),
    Alternative(Vec<u32>, Vec<u32>),
    MoreOfFirst(u32, u32),
}

#[derive(Debug)]
pub struct RuleMap(AHashMap<u32, ParseRule>);

impl Default for RuleMap {
    fn default() -> Self {
        Self(AHashMap::new())
    }
}

fn parse_char<'a>(c: char, s: &'a str) -> Option<&'a str> {
    let mut char_indices = s.char_indices();
    match char_indices.next() {
        Some((_, first)) if first == c => match char_indices.next() {
            Some((pos, _)) => Some(&s[pos..]),
            None => Some(""),
        },
        _ => None,
    }
}

fn parse_seq<'a>(rule_map: &RuleMap, seq: &[u32], s: &'a str) -> Option<&'a str> {
    let mut remaining = s;
    for sub_rule in seq.iter() {
        match test_rule(rule_map, *sub_rule, remaining) {
            Some(r) => remaining = r,
            _ => return None,
        }
    }

    Some(remaining)
}

fn parse_more_first<'a>(rule_map: &RuleMap, start: u32, end: u32, s: &'a str) -> Option<&'a str> {
    let mut remaining = s;
    let mut count: usize = 0;
    while let Some(r) = test_rule(rule_map, start, remaining) {
        remaining = r;
        count += 1;
    }

    if count < 2 {
        return None;
    }

    for i in 0..count - 1 {
        match test_rule(rule_map, end, remaining) {
            Some(r) => remaining = r,
            None if i == 0 => return None,
            None => break,
        }
    }

    Some(remaining)
}

fn test_rule<'a>(rule_map: &RuleMap, rule_id: u32, s: &'a str) -> Option<&'a str> {
    match rule_map.0.get(&rule_id).unwrap() {
        ParseRule::Character(c) => parse_char(*c, s),
        ParseRule::Sequence(seq) => parse_seq(rule_map, seq, s),
        ParseRule::Alternative(seq1, seq2) => {
            parse_seq(rule_map, seq1, s).or_else(|| parse_seq(rule_map, seq2, s))
        }
        ParseRule::MoreOfFirst(start, end) => parse_more_first(rule_map, *start, *end, s),
    }
}

fn test_rules(rule_map: &RuleMap, s: &str) -> bool {
    matches!(test_rule(rule_map, 0, s), Some(r) if r.is_empty())
}

mod rule_parsing {
    use super::ParseRule;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{anychar, char, digit1},
        combinator::{all_consuming, map, map_res},
        multi::separated_list1,
        sequence::{delimited, separated_pair},
        IResult,
    };

    fn character(s: &str) -> IResult<&str, ParseRule> {
        map(delimited(char('"'), anychar, char('"')), |c: char| {
            ParseRule::Character(c)
        })(s)
    }

    fn number(s: &str) -> IResult<&str, u32> {
        map_res(digit1, str::parse)(s)
    }

    fn sequence(s: &str) -> IResult<&str, Vec<u32>> {
        separated_list1(char(' '), number)(s)
    }

    fn alternative(s: &str) -> IResult<&str, ParseRule> {
        map(separated_pair(sequence, tag(" | "), sequence), |(a, b)| {
            ParseRule::Alternative(a, b)
        })(s)
    }

    pub fn rule(s: &str) -> IResult<&str, (u32, ParseRule)> {
        all_consuming(separated_pair(
            number,
            tag(": "),
            alt((character, alternative, map(sequence, ParseRule::Sequence))),
        ))(s)
    }
}

impl RuleMap {
    fn try_add_rule(&mut self, line: impl AsRef<str>) -> Result<(), ParseError> {
        let (id, parse_rule) = rule_parsing::rule(line.as_ref())
            .finish()
            .map_err(|e| ParseError(e.to_string()))?
            .1;
        self.0.insert(id, parse_rule);
        Ok(())
    }

    fn update_rules(&mut self) {
        self.0.insert(0, ParseRule::MoreOfFirst(42, 31));
        // not implemented individually, remove them to trigger a panic if they get referenced.
        self.0.remove(&8);
        self.0.remove(&11);
    }
}

fn read_file(path: impl AsRef<Path>) -> Result<(RuleMap, Vec<String>), Box<dyn Error>> {
    enum ReadState {
        Rules,
        Messages,
    };
    let file = File::open(path)?;
    let mut rule_map = RuleMap::default();
    let mut messages = Vec::new();
    let mut state = ReadState::Rules;
    for line_result in BufReader::new(file).lines() {
        let line = line_result?;
        if line.is_empty() {
            state = ReadState::Messages;
            continue;
        }
        match state {
            ReadState::Rules => rule_map.try_add_rule(line)?,
            ReadState::Messages => messages.push(line),
        }
    }

    Ok((rule_map, messages))
}

fn run() -> Result<(), Box<dyn Error>> {
    let (mut rule_map, messages) = {
        let path = ["data", "day19", "input.txt"].iter().collect::<PathBuf>();
        read_file(path)?
    };

    let part1 = messages.iter().filter(|m| test_rules(&rule_map, m)).count();
    println!("Part 1: valid count = {}", part1);

    rule_map.update_rules();

    let part2 = messages.iter().filter(|m| test_rules(&rule_map, m)).count();
    println!("Part 2: valid count = {}", part2);

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
    use super::{test_rules, RuleMap};

    const PART1_RULES: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b""#;

    const PART1_TESTS: [(&str, bool); 5] = [
        ("ababbb", true),
        ("bababa", false),
        ("abbbab", true),
        ("aaabbb", false),
        ("aaaabbb", false),
    ];

    #[test]
    fn part1_test() {
        let mut rule_map = RuleMap::default();
        PART1_RULES
            .lines()
            .for_each(|line| rule_map.try_add_rule(line).unwrap());

        for &(message, expected) in PART1_TESTS.iter() {
            let result = test_rules(&rule_map, message);
            assert_eq!(result, expected, "message {} failed", message);
        }
    }

    const PART2_RULES: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"#;

    const PART2_TESTS: [(&str, bool); 15] = [
        ("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa", false),
        ("bbabbbbaabaabba", true),
        ("babbbbaabbbbbabbbbbbaabaaabaaa", true),
        ("aaabbbbbbaaaabaababaabababbabaaabbababababaaa", true),
        ("bbbbbbbaaaabbbbaaabbabaaa", true),
        ("bbbababbbbaaaaaaaabbababaaababaabab", true),
        ("ababaaaaaabaaab", true),
        ("ababaaaaabbbaba", true),
        ("baabbaaaabbaaaababbaababb", true),
        ("abbbbabbbbaaaababbbbbbaaaababb", true),
        ("aaaaabbaabaaaaababaa", true),
        ("aaaabbaaaabbaaa", false),
        ("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa", true),
        ("babaaabbbaaabaababbaabababaaab", false),
        ("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", true),
    ];

    #[test]
    fn part2_test() {
        let mut rule_map = RuleMap::default();
        PART2_RULES
            .lines()
            .for_each(|line| rule_map.try_add_rule(line).unwrap());
        rule_map.update_rules();
        for &(message, expected) in PART2_TESTS.iter() {
            let result = test_rules(&rule_map, message);
            assert_eq!(result, expected, "message {} failed", message);
        }
    }
}
