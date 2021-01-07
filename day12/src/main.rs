use std::{
    convert::TryInto,
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    str::FromStr,
};

use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug)]
struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Application error ({})", self.0)
    }
}

impl Error for ParseError {}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(i32)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn turn_right(&self, steps: i32) -> Direction {
        let new_direction = (i32::from(*self) + steps) & 0b11;
        new_direction
            .try_into()
            .expect("new direction out-of-range")
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    North(i32),
    East(i32),
    Right(i32),
    Forward(i32),
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_indices = s.char_indices();
        let opcode = char_indices.next().ok_or(ParseError("missing opcode"))?.1;
        let value_start = char_indices.next().ok_or(ParseError("missing value"))?.0;
        let value = s[value_start..]
            .parse()
            .map_err(|_| ParseError("could not parse value"))?;
        match opcode {
            'N' => Ok(Instruction::North(value)),
            'S' => Ok(Instruction::North(-value)),
            'E' => Ok(Instruction::East(value)),
            'W' => Ok(Instruction::East(-value)),
            'L' if value % 90 == 0 => Ok(Instruction::Right(-value / 90)),
            'R' if value % 90 == 0 => Ok(Instruction::Right(value / 90)),
            'L' | 'R' => Err(ParseError("bad rotation")),
            'F' => Ok(Instruction::Forward(value)),
            _ => Err(ParseError("unknown opcode")),
        }
    }
}

#[derive(Debug)]
struct Point {
    north: i32,
    east: i32,
}

impl Point {
    pub fn manhattan_distance(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }
}

fn process_path<'a>(path: impl Iterator<Item = &'a Instruction>) -> i32 {
    let mut position = Point { north: 0, east: 0 };
    let mut direction = Direction::East;

    for instruction in path {
        match instruction {
            Instruction::North(delta) => position.north += *delta,
            Instruction::East(delta) => position.east += *delta,
            Instruction::Right(steps) => direction = direction.turn_right(*steps),
            Instruction::Forward(steps) => match direction {
                Direction::East => position.east += *steps,
                Direction::South => position.north -= *steps,
                Direction::West => position.east -= *steps,
                Direction::North => position.north += *steps,
            },
        }
    }

    position.manhattan_distance()
}

fn process_waypoint<'a>(path: impl Iterator<Item = &'a Instruction>) -> i32 {
    let mut ship = Point { north: 0, east: 0 };
    let mut waypoint = Point { north: 1, east: 10 };

    for instruction in path {
        match instruction {
            Instruction::North(delta) => waypoint.north += delta,
            Instruction::East(delta) => waypoint.east += delta,
            Instruction::Right(steps) => {
                waypoint = match steps & 0b11 {
                    1 => Point {
                        north: -waypoint.east,
                        east: waypoint.north,
                    },
                    2 => Point {
                        north: -waypoint.north,
                        east: -waypoint.east,
                    },
                    3 => Point {
                        north: waypoint.east,
                        east: -waypoint.north,
                    },
                    _ => waypoint,
                }
            }
            Instruction::Forward(steps) => {
                ship.north += waypoint.north * steps;
                ship.east += waypoint.east * steps;
            }
        }
    }

    ship.manhattan_distance()
}

fn run() -> Result<(), Box<dyn Error>> {
    let instructions = {
        let path = ["data", "day12", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        let mut instructions = Vec::new();
        for line in BufReader::new(file).lines() {
            instructions.push(line?.parse()?);
        }

        instructions
    };

    println!("Part 1: result = {}", process_path(instructions.iter()));
    println!("Part 2: result = {}", process_waypoint(instructions.iter()));

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
    use super::{process_path, process_waypoint, Direction, Instruction};

    const EXAMPLE1_TEXT: &str = r"F10
N3
F7
R90
F11";

    const EXAMPLE1: [Instruction; 5] = [
        Instruction::Forward(10),
        Instruction::North(3),
        Instruction::Forward(7),
        Instruction::Right(1),
        Instruction::Forward(11),
    ];

    #[test]
    fn turn_left_test() {
        const TEST_CASES: [(Direction, i32, Direction); 9] = [
            (Direction::East, -4, Direction::East),
            (Direction::East, -3, Direction::South),
            (Direction::East, -2, Direction::West),
            (Direction::East, -1, Direction::North),
            (Direction::East, 0, Direction::East),
            (Direction::East, 1, Direction::South),
            (Direction::East, 2, Direction::West),
            (Direction::East, 3, Direction::North),
            (Direction::East, 4, Direction::East),
        ];

        for (start, steps, finish) in &TEST_CASES {
            let result = start.turn_right(*steps);
            assert_eq!(result, *finish);

            let back_result = finish.turn_right(-*steps);
            assert_eq!(back_result, *start);
        }
    }

    #[test]
    fn parse_test() {
        let result = EXAMPLE1_TEXT
            .lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(result, EXAMPLE1);
    }

    #[test]
    fn part1_test() {
        let result = process_path(EXAMPLE1.iter());
        assert_eq!(result, 25);
    }

    #[test]
    fn part2_test() {
        let result = process_waypoint(EXAMPLE1.iter());
        assert_eq!(result, 286);
    }
}
