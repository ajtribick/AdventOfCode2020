use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use bitvec::prelude::*;

#[derive(Debug)]
enum Day8Error {
    IoError(io::Error),
    ParseError,
    NoSolution,
}

impl fmt::Display for Day8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO Error ({})", e),
            Self::ParseError => write!(f, "Parse error"),
            Self::NoSolution => write!(f, "No solution found"),
        }
    }
}

impl Error for Day8Error {}

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = Day8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 5 {
            return Err(Day8Error::ParseError);
        }
        let opcode = &s[0..3];
        let value = s[4..].parse().map_err(|_| Day8Error::ParseError)?;
        match opcode {
            "acc" => Ok(Instruction::Acc(value)),
            "jmp" => Ok(Instruction::Jmp(value)),
            "nop" => Ok(Instruction::Nop(value)),
            _ => Err(Day8Error::ParseError),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ProgramResult {
    Terminate(i32),
    Loop(i32),
}

fn execute(program: &[Instruction]) -> ProgramResult {
    let mut accumulator = 0;
    let mut counter = 0;
    let mut visited = BitVec::<LocalBits, usize>::repeat(false, program.len());
    loop {
        if counter >= program.len() {
            return ProgramResult::Terminate(accumulator);
        }
        if visited[counter] {
            return ProgramResult::Loop(accumulator);
        }
        visited.set(counter, true);
        match program[counter] {
            Instruction::Acc(delta) => {
                accumulator += delta;
                counter += 1;
            }
            Instruction::Jmp(delta) if delta >= 0 => counter += delta as usize,
            Instruction::Jmp(delta) => counter -= delta.abs() as usize,
            Instruction::Nop(_) => counter += 1,
        }
    }
}

fn part1(program: &[Instruction]) -> Result<(), Day8Error> {
    match execute(&program) {
        ProgramResult::Loop(result) => {
            println!("Part 1: accumulator = {}", result);
            Ok(())
        }
        _ => Err(Day8Error::NoSolution),
    }
}

fn patch(instruction: &mut Instruction) -> bool {
    match *instruction {
        Instruction::Jmp(delta) => {
            *instruction = Instruction::Nop(delta);
            true
        }
        Instruction::Nop(delta) => {
            *instruction = Instruction::Jmp(delta);
            true
        }
        _ => false,
    }
}

fn execute_patched(patched: &mut [Instruction]) -> Result<i32, Day8Error> {
    for p in 0..patched.len() {
        if patch(&mut patched[p]) {
            match execute(&patched) {
                ProgramResult::Terminate(result) => return Ok(result),
                _ => {
                    patch(&mut patched[p]);
                }
            }
        }
    }

    Err(Day8Error::NoSolution)
}

fn part2(program: &mut [Instruction]) -> Result<(), Day8Error> {
    let result = execute_patched(program)?;
    println!("Part 2: accumulator = {}", result);
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = ["data", "day08", "input.txt"].iter().collect::<PathBuf>();
    let file = File::open(path)?;
    let mut program = BufReader::new(file)
        .lines()
        .map(|l| l.map_err(Day8Error::IoError).and_then(|s| s.parse()))
        .collect::<Result<Vec<_>, _>>()?;
    part1(&program)?;
    part2(&mut program)?;
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
    use super::{execute, execute_patched, Instruction, ProgramResult};

    const EXAMPLE: [Instruction; 9] = [
        Instruction::Nop(0),
        Instruction::Acc(1),
        Instruction::Jmp(4),
        Instruction::Acc(3),
        Instruction::Jmp(-3),
        Instruction::Acc(-99),
        Instruction::Acc(1),
        Instruction::Jmp(-4),
        Instruction::Acc(6),
    ];

    #[test]
    fn part1_test() {
        let result = execute(&EXAMPLE);
        assert_eq!(result, ProgramResult::Loop(5));
    }

    #[test]
    fn part2_test() {
        let mut program = EXAMPLE.clone();
        let result = execute_patched(&mut program).unwrap();
        assert_eq!(result, 8);
    }
}
