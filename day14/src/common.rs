use std::{error::Error, fmt, str::FromStr};

#[derive(Debug)]
pub struct ParseError(pub &'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseError {}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Mask(u64, u64, u64),
    Assign(u64, u64),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" = ");
        let operation = parts.next().ok_or(ParseError("Missing operation"))?;
        let value_str = parts.next().ok_or(ParseError("Missing value"))?;
        if operation == "mask" {
            let mut zeroes = 0;
            let mut ones = 0;
            let mut floating = u64::MAX;
            for c in value_str.chars() {
                zeroes <<= 1;
                ones <<= 1;
                floating <<= 1;
                match c {
                    '0' => zeroes |= 1,
                    '1' => ones |= 1,
                    'X' => floating |= 1,
                    _ => return Err(ParseError("Bad mask character")),
                }
            }

            Ok(Instruction::Mask(zeroes, ones, floating))
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

#[cfg(test)]
mod test {
    use super::Instruction;

    use std::error::Error;

    const EXAMPLE_TEXT: &str = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const EXAMPLE_PROGRAM: [Instruction; 4] = [
        Instruction::Mask(0b10, 0b1000000, !0b1000010),
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
}
