use bitvec::prelude::*;
use regex::Regex;
use std::{error::Error, fmt, iter, ops::RangeInclusive, str::FromStr};

#[derive(Debug)]
pub struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct FieldInfo {
    name: String,
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl FieldInfo {
    pub fn contains(&self, value: &usize) -> bool {
        self.range1.contains(value) || self.range2.contains(value)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl FromStr for FieldInfo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^([^:]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$").unwrap();
        }

        let captures = RE.captures(s).ok_or(ParseError("Not a field"))?;
        let name = captures[1].to_owned();
        let start1 = captures[2].parse().map_err(|_| ParseError("Bad range"))?;
        let end1 = captures[3].parse().map_err(|_| ParseError("Bad range"))?;
        let start2 = captures[4].parse().map_err(|_| ParseError("Bad range"))?;
        let end2 = captures[5].parse().map_err(|_| ParseError("Bad range"))?;

        Ok(Self {
            name,
            range1: start1..=end1,
            range2: start2..=end2,
        })
    }
}

#[derive(Debug)]
pub struct Problem {
    fields: Vec<FieldInfo>,
    your_ticket: Vec<usize>,
    ticket_data: Vec<usize>,
    allowed: BitVec,
}

fn parse_fields(
    lines: &mut impl Iterator<Item = impl AsRef<str>>,
) -> Result<(Vec<FieldInfo>, BitVec), ParseError> {
    let mut fields = Vec::new();
    let mut allowed = bitvec![0; 1000];

    while let Some(line_ref) = lines.next() {
        let line: &str = line_ref.as_ref();
        if line.is_empty() {
            break;
        }

        let field = line.parse::<FieldInfo>()?;
        allowed[field.range1.clone()].set_all(true);
        allowed[field.range2.clone()].set_all(true);

        fields.push(field);
    }

    Ok((fields, allowed))
}

fn parse_line(
    lines: &mut impl Iterator<Item = impl AsRef<str>>,
    expected: &str,
    message: &'static str,
) -> Result<(), ParseError> {
    if lines.next().ok_or(ParseError(message))?.as_ref() == expected {
        Ok(())
    } else {
        Err(ParseError(message))
    }
}

fn parse_ticket(line: &str, field_count: usize) -> Result<Vec<usize>, ParseError> {
    let result = line
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ParseError("Failed to parse ticket value as number"))?;
    if result.len() == field_count {
        Ok(result)
    } else {
        Err(ParseError("Incorrect field count"))
    }
}

impl Problem {
    pub fn parse(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Result<Self, ParseError> {
        let (fields, allowed) = parse_fields(&mut lines)?;
        let field_count = fields.len();

        parse_line(&mut lines, "your ticket:", "Missing your ticket")?;
        let your_ticket = parse_ticket(
            lines
                .next()
                .ok_or(ParseError("No data for your ticket".into()))?
                .as_ref(),
            field_count,
        )?;
        parse_line(&mut lines, "", "Unexpected data in ticket")?;

        parse_line(&mut lines, "nearby tickets:", "Missing nearby tickets")?;
        let mut ticket_data = Vec::new();
        for line_ref in lines {
            parse_ticket(line_ref.as_ref(), field_count)?
                .drain(..)
                .for_each(|v| ticket_data.push(v));
        }

        Ok(Self {
            fields,
            your_ticket,
            ticket_data,
            allowed,
        })
    }

    pub fn fields(&self) -> &[FieldInfo] {
        &self.fields
    }

    pub fn your_ticket(&self) -> &[usize] {
        &self.your_ticket
    }

    pub fn other_tickets(&self) -> impl Iterator<Item = &[usize]> {
        self.ticket_data.chunks(self.fields.len())
    }

    pub fn all_tickets(&self) -> impl Iterator<Item = &[usize]> {
        self.other_tickets()
            .chain(iter::once(self.your_ticket.as_slice()))
    }

    pub fn error_rate(&self) -> u64 {
        self.ticket_data
            .iter()
            .copied()
            .filter(|&v| !self.allowed[v])
            .map(|v| v as u64)
            .sum()
    }

    pub fn assign_fields(&self) -> Vec<usize> {
        let mut allowed_columns = vec![bitvec![1; self.fields.len()]; self.fields.len()];
        self.all_tickets()
            .filter(|t| t.iter().all(|&v| self.allowed[v]))
            .flat_map(|t| t.iter().enumerate())
            .for_each(|(col, value)| {
                self.fields
                    .iter()
                    .zip(allowed_columns.iter_mut())
                    .filter(|(f, _)| !f.contains(value))
                    .for_each(|(_, a)| a.set(col, false));
            });

        let mut field_assignments = vec![usize::MAX; self.fields.len()];
        for _ in 0..self.fields.len() {
            let (field, allowed) = allowed_columns
                .iter()
                .enumerate()
                .filter(|(_, a)| a.count_ones() == 1)
                .next()
                .expect("Backtracking not implemented");
            let (col, _) = allowed
                .iter()
                .enumerate()
                .filter(|(_, b)| **b)
                .next()
                .unwrap();
            field_assignments[field] = col;
            allowed_columns.iter_mut().for_each(|a| a.set(col, false));
        }

        field_assignments
    }
}

#[cfg(test)]
mod test {
    use super::Problem;

    use std::ops::RangeInclusive;

    const EXAMPLE: &str = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const EXPECTED_FIELDS: [(&str, RangeInclusive<usize>, RangeInclusive<usize>); 3] = [
        ("class", 1..=3, 5..=7),
        ("row", 6..=11, 33..=44),
        ("seat", 13..=40, 45..=50),
    ];

    const EXPECTED_YOUR_TICKET: [usize; 3] = [7, 1, 14];

    const EXPECTED_OTHER_TICKETS: [[usize; 3]; 4] =
        [[7, 3, 47], [40, 4, 50], [55, 2, 20], [38, 6, 12]];

    #[test]
    fn parse_test() {
        let problem = Problem::parse(EXAMPLE.lines()).unwrap();

        for (field, (name, range1, range2)) in problem.fields.iter().zip(EXPECTED_FIELDS.iter()) {
            assert_eq!(field.name, *name);
            assert_eq!(&field.range1, range1);
            assert_eq!(&field.range2, range2);
        }

        assert_eq!(problem.your_ticket, EXPECTED_YOUR_TICKET);
        assert_eq!(
            problem.other_tickets().collect::<Vec<_>>(),
            EXPECTED_OTHER_TICKETS
        );
    }

    #[test]
    fn test_rate() {
        let problem = Problem::parse(EXAMPLE.lines()).unwrap();
        let result = problem.error_rate();
        assert_eq!(result, 71);
    }

    const EXAMPLE2: &str = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    const EXPECTED_ASSIGNMENTS: [usize; 3] = [1, 0, 2];

    #[test]
    fn part2_test() {
        let problem = Problem::parse(EXAMPLE2.lines()).unwrap();
        let field_assignments = problem.assign_fields();
        assert_eq!(&EXPECTED_ASSIGNMENTS[..], field_assignments);
    }
}
