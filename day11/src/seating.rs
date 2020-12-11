use std::{cmp::min, fmt, str::FromStr};

use crate::error::Day11Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Seat {
    Empty,
    Unoccupied,
    Occupied,
}

#[derive(Debug)]
pub struct SeatingPlan {
    width: usize,
    height: usize,
    state: bool,
    data1: Vec<Seat>,
    data2: Vec<Seat>,
}

fn count_line(line: &[Seat]) -> usize {
    line.iter().filter(|s| **s == Seat::Occupied).count()
}

fn process_lines<'a>(
    prev: Option<&[Seat]>,
    curr: &[Seat],
    next: Option<&[Seat]>,
    dest: &mut impl Iterator<Item = &'a mut Seat>,
) -> bool {
    let mut modified = false;

    for x in 0..curr.len() {
        let current = curr[x];
        if current == Seat::Empty {
            dest.next();
            continue;
        }

        let range = x.saturating_sub(1)..min(curr.len(), x + 2);
        let occupied_neighbors = prev.map_or(0, |p| count_line(&p[range.clone()]))
            + count_line(&curr[range.clone()])
            + next.map_or(0, |n| count_line(&n[range]));

        *dest.next().unwrap() = match current {
            Seat::Unoccupied if occupied_neighbors == 0 => {
                modified = true;
                Seat::Occupied
            }
            Seat::Occupied if occupied_neighbors >= 5 => {
                modified = true;
                Seat::Unoccupied
            }
            _ => current,
        };
    }

    modified
}

impl SeatingPlan {
    fn current(&self) -> &[Seat] {
        if self.state {
            &self.data2
        } else {
            &self.data1
        }
    }

    pub fn occupied(&self) -> usize {
        self.current()
            .iter()
            .filter(|s| **s == Seat::Occupied)
            .count()
    }

    pub fn update(&mut self) -> bool {
        let (mut src, mut dest) = if self.state {
            (self.data2.chunks(self.width), self.data1.iter_mut())
        } else {
            (self.data1.chunks(self.width), self.data2.iter_mut())
        };

        let mut modified = false;

        let mut prev: Option<&[Seat]> = None;
        let mut curr = src.next();
        let mut next = src.next();

        while let Some(curr_line) = curr {
            let line_modified = process_lines(prev, curr_line, next, &mut dest);
            modified |= line_modified;

            prev = curr;
            curr = next;
            next = src.next();
        }

        self.state = !self.state;
        modified
    }
}

impl fmt::Display for SeatingPlan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = self.current().chunks(self.width).map(|line| {
            line.iter()
                .map(|c| match c {
                    Seat::Empty => '.',
                    Seat::Unoccupied => 'L',
                    Seat::Occupied => '#',
                })
                .collect::<String>()
        });

        if let Some(line) = lines.next() {
            write!(f, "{}", line)?;
            for line in lines {
                write!(f, "\n{}", line)?;
            }
        }

        Ok(())
    }
}

impl FromStr for SeatingPlan {
    type Err = Day11Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let mut data1 = Vec::with_capacity(s.len());
        let mut width = 0;
        let mut height = 0;
        for line in lines {
            if width != 0 && line.len() != width {
                return Err(Day11Error::ParseError("Inconsistent widths"));
            } else {
                width = line.len();
            }

            for c in line.chars() {
                let seat = match c {
                    '.' => Ok(Seat::Empty),
                    'L' => Ok(Seat::Unoccupied),
                    '#' => Ok(Seat::Occupied),
                    _ => Err(Day11Error::ParseError("Unknown character")),
                };
                data1.push(seat?);
            }

            height += 1;
        }

        let data2 = data1.clone();

        Ok(SeatingPlan {
            width,
            height,
            state: false,
            data1,
            data2,
        })
    }
}

#[cfg(test)]
mod test {
    use super::SeatingPlan;

    use std::error::Error;

    const EXAMPLES: [&str; 6] = [
        r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        r"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
        r"#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
        r"#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
        r"#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
        r"#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
    ];

    #[test]
    fn test_string_roundtrip() -> Result<(), Box<dyn Error>> {
        for &layout in EXAMPLES.iter() {
            let plan = layout.parse::<SeatingPlan>()?;
            let result = plan.to_string();
            assert_eq!(result, layout);
        }

        Ok(())
    }

    #[test]
    fn test_update() -> Result<(), Box<dyn Error>> {
        let mut plan = EXAMPLES[0].parse::<SeatingPlan>()?;
        for &expected in EXAMPLES[1..].iter() {
            let was_updated = plan.update();
            assert!(was_updated);
            assert_eq!(plan.to_string(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_no_update() -> Result<(), Box<dyn Error>> {
        let expected = *EXAMPLES.last().unwrap();
        let mut plan = expected.parse::<SeatingPlan>()?;
        let was_updated = plan.update();
        assert!(!was_updated);
        assert_eq!(plan.to_string(), expected);
        Ok(())
    }

    #[test]
    fn test_occupied() -> Result<(), Box<dyn Error>> {
        let mut plan = EXAMPLES[0].parse::<SeatingPlan>()?;
        while plan.update() {}
        assert_eq!(plan.occupied(), 37);
        Ok(())
    }
}
