use std::{str::FromStr, collections::HashMap};

use crate::error::ParseError;

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Mask(u64, u64),
    Assign(u64, u64),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" = ");
        let operation = parts.next().ok_or(ParseError("Missing operation"))?;
        let value_str = parts.next().ok_or(ParseError("Missing value"))?;
        if operation == "mask" {
            let mut set_mask = 0;
            let mut unset_mask = 0;
            for c in value_str.chars() {
                set_mask <<= 1;
                unset_mask <<= 1;
                match c {
                    'X' => (),
                    '0' => unset_mask |= 1,
                    '1' => set_mask |= 1,
                    _ => return Err(ParseError("Bad mask character")),
                }
            }

            Ok(Instruction::Mask(set_mask, !unset_mask))
        } else if operation.starts_with("mem[") && operation.ends_with(']') {
            let address = operation[4..operation.len() - 1]
                .parse()
                .map_err(|_| ParseError("Could not parse address"))?;
            let value = value_str
                .parse()
                .map_err(|_| ParseError("Could not parse value"))?;
            Ok(Instruction::Assign(address, value))
        } else {
            Err(ParseError("Unknown operation"))
        }
    }
}

fn execute_program<'a>(program: impl Iterator<Item = &'a Instruction>) -> u64 {
    let mut memory = HashMap::new();
    let (mut set_mask, mut unset_mask) = (0, u64::MAX);
    for instruction in program {
        match instruction {
            Instruction::Mask(set, unset) => {
                set_mask = *set;
                unset_mask = *unset;
            }
            Instruction::Assign(address, value) => {
                match (value | set_mask) & unset_mask {
                    0 => memory.remove(address),
                    v => memory.insert(*address, v),
                };
            }
        }
    }

    memory.values().sum()
}

pub fn part1(code: &str) -> Result<(), ParseError> {
    let program = code.lines().map(|line| line.parse()).collect::<Result<Vec<_>, _>>()?;
    println!("Part 1: result = {}", execute_program(program.iter()));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{execute_program, Instruction};

    use std::error::Error;

    const EXAMPLE_TEXT: &str = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const EXAMPLE_PROGRAM: [Instruction; 4] = [
        Instruction::Mask(0b1000000, !0b10),
        Instruction::Assign(8, 11),
        Instruction::Assign(7, 101),
        Instruction::Assign(8, 0),
    ];

    #[test]
    fn parse_test() -> Result<(), Box<dyn Error>> {
        let program = EXAMPLE_TEXT
            .lines()
            .map(|l| l.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()?;
        assert!(program.iter().eq(EXAMPLE_PROGRAM.iter()));
        Ok(())
    }

    #[test]
    fn execute_test() {
        let result = execute_program(EXAMPLE_PROGRAM.iter());
        assert_eq!(result, 165);
    }
}
