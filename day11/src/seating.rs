use std::{cmp::min, fmt, str::FromStr};

use crate::error::Day11Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Empty,
    Unoccupied,
    Occupied,
}

#[derive(Debug, Clone)]
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

fn scan_left(src: &[Seat], i: usize, x: usize) -> bool {
    src[i - x..i]
        .iter()
        .rev()
        .filter(|&&s| s != Seat::Empty)
        .next()
        .map_or(false, |&s| s == Seat::Occupied)
}

fn scan_right(src: &[Seat], i: usize, x: usize, width: usize) -> bool {
    src[i + 1..i + width - x]
        .iter()
        .filter(|&&s| s != Seat::Empty)
        .next()
        .map_or(false, |&s| s == Seat::Occupied)
}

fn scan_up(src: &[Seat], i: usize, width: usize) -> bool {
    src[..=i]
        .iter()
        .rev()
        .step_by(width)
        .skip(1)
        .filter(|&&s| s != Seat::Empty)
        .next()
        .map_or(false, |&s| s == Seat::Occupied)
}

fn scan_down(src: &[Seat], i: usize, width: usize) -> bool {
    src[i..]
        .iter()
        .step_by(width)
        .skip(1)
        .filter(|&&s| s != Seat::Empty)
        .next()
        .map_or(false, |&s| s == Seat::Occupied)
}

fn scan_left_up(src: &[Seat], i: usize, x: usize, width: usize) -> bool {
    src[..=i]
        .iter()
        .rev()
        .step_by(width + 1)
        .skip(1)
        .take(x)
        .filter(|&&s| s != Seat::Empty)
        .next()
        .map_or(false, |&s| s == Seat::Occupied)
}

fn scan_right_up(src: &[Seat], i: usize, x: usize, width: usize) -> bool {
    src[..=i]
        .iter()
        .rev()
        .step_by(width - 1)
        .skip(1)
        .take(width - x - 1)
        .filter(|&&s| s != Seat::Empty)
        .next()
        .map_or(false, |&s| s == Seat::Occupied)
}

fn scan_left_down(src: &[Seat], i: usize, x: usize, width: usize) -> bool {
    src[i..]
        .iter()
        .step_by(width - 1)
        .skip(1)
        .take(x)
        .filter(|&&s| s != Seat::Empty)
        .next()
        .map_or(false, |&s| s == Seat::Occupied)
}

fn scan_right_down(src: &[Seat], i: usize, x: usize, width: usize) -> bool {
    src[i..]
        .iter()
        .step_by(width + 1)
        .skip(1)
        .take(width - x - 1)
        .filter(|&&s| s != Seat::Empty)
        .next()
        .map_or(false, |&s| s == Seat::Occupied)
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
            for x in 0..curr_line.len() {
                let current = curr_line[x];
                if current == Seat::Empty {
                    dest.next();
                    continue;
                }

                let range = x.saturating_sub(1)..min(curr_line.len(), x + 2);
                let occupied_neighbors = prev.map_or(0, |p| count_line(&p[range.clone()]))
                    + count_line(&curr_line[range.clone()])
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

            prev = curr;
            curr = next;
            next = src.next();
        }

        self.state = !self.state;
        modified
    }

    pub fn update2(&mut self) -> bool {
        let (src, mut dest) = if self.state {
            (&self.data2, self.data1.iter_mut())
        } else {
            (&self.data1, self.data2.iter_mut())
        };

        let mut modified = false;

        for i in 0..src.len() {
            let current = src[i];
            if current == Seat::Empty {
                dest.next();
                continue;
            }

            let x = i % self.width;
            let visible = [
                scan_left(src, i, x),
                scan_right(src, i, x, self.width),
                scan_up(src, i, self.width),
                scan_down(src, i, self.width),
                scan_left_up(src, i, x, self.width),
                scan_right_up(src, i, x, self.width),
                scan_left_down(src, i, x, self.width),
                scan_right_down(src, i, x, self.width),
            ]
            .iter()
            .filter(|v| **v)
            .count();

            *dest.next().unwrap() = match current {
                Seat::Unoccupied if visible == 0 => {
                    modified = true;
                    Seat::Occupied
                }
                Seat::Occupied if visible >= 5 => {
                    modified = true;
                    Seat::Unoccupied
                }
                _ => current,
            }
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

    const EXAMPLES_PART1: [&str; 6] = [
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

    const EXAMPLES_PART2: [&str; 7] = [
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
        r"#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#",
        r"#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#",
        r"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#",
        r"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
        r"#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#",
    ];

    #[test]
    fn test_string_roundtrip() -> Result<(), Box<dyn Error>> {
        for &layout in EXAMPLES_PART1.iter() {
            let plan = layout.parse::<SeatingPlan>()?;
            let result = plan.to_string();
            assert_eq!(result, layout);
        }

        Ok(())
    }

    #[test]
    fn test_update() -> Result<(), Box<dyn Error>> {
        let mut plan = EXAMPLES_PART1[0].parse::<SeatingPlan>()?;
        for &expected in EXAMPLES_PART1[1..].iter() {
            let was_updated = plan.update();
            assert!(was_updated);
            assert_eq!(plan.to_string(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_no_update() -> Result<(), Box<dyn Error>> {
        let expected = *EXAMPLES_PART1.last().unwrap();
        let mut plan = expected.parse::<SeatingPlan>()?;
        let was_updated = plan.update();
        assert!(!was_updated);
        assert_eq!(plan.to_string(), expected);
        Ok(())
    }

    #[test]
    fn test_occupied() -> Result<(), Box<dyn Error>> {
        let plan = EXAMPLES_PART1.last().unwrap().parse::<SeatingPlan>()?;
        assert_eq!(plan.occupied(), 37);
        Ok(())
    }

    #[test]
    fn test_update2() -> Result<(), Box<dyn Error>> {
        let mut plan = EXAMPLES_PART2[0].parse::<SeatingPlan>()?;
        for &expected in EXAMPLES_PART2[1..].iter() {
            let was_updated = plan.update2();
            assert!(was_updated);
            assert_eq!(plan.to_string(), expected);
        }

        Ok(())
    }

    #[test]
    fn test_no_update2() -> Result<(), Box<dyn Error>> {
        let expected = *EXAMPLES_PART2.last().unwrap();
        let mut plan = expected.parse::<SeatingPlan>()?;
        let was_updated = plan.update2();
        assert!(!was_updated);
        assert_eq!(plan.to_string(), expected);
        Ok(())
    }

    #[test]
    fn test_occupied2() -> Result<(), Box<dyn Error>> {
        let plan = EXAMPLES_PART2.last().unwrap().parse::<SeatingPlan>()?;
        assert_eq!(plan.occupied(), 26);
        Ok(())
    }
}
