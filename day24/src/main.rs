use std::{
    error::Error,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use ahash::AHashSet;

#[derive(Debug)]
struct ParseCoordsError(&'static str);

impl fmt::Display for ParseCoordsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseCoordsError {}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Coords(i32, i32);

impl Coords {
    pub fn parse_line(line: impl AsRef<str>) -> Result<Self, ParseCoordsError> {
        enum ParseState {
            None,
            North,
            South,
        }

        let mut state = ParseState::None;
        let mut x = 0;
        let mut y = 0;

        for c in line.as_ref().bytes() {
            match (&state, c) {
                (ParseState::None, b'w') => x -= 1,
                (ParseState::None, b'e') => x += 1,
                (ParseState::None, b'n') => {
                    y -= 1;
                    state = ParseState::North;
                }
                (ParseState::None, b's') => {
                    y += 1;
                    state = ParseState::South;
                }
                (ParseState::North, b'w') => {
                    x -= 1;
                    state = ParseState::None;
                }
                (ParseState::North, b'e') => state = ParseState::None,
                (ParseState::South, b'w') => state = ParseState::None,
                (ParseState::South, b'e') => {
                    x += 1;
                    state = ParseState::None;
                }
                _ => return Err(ParseCoordsError("Invalid character")),
            }
        }

        match state {
            ParseState::None => Ok(Coords(x, y)),
            _ => Err(ParseCoordsError("Unexpected end of line")),
        }
    }

    pub fn get_neighbors(&self) -> [Self; 6] {
        [
            Self(self.0 - 1, self.1),
            Self(self.0 + 1, self.1),
            Self(self.0 - 1, self.1 - 1),
            Self(self.0, self.1 - 1),
            Self(self.0, self.1 + 1),
            Self(self.0 + 1, self.1 + 1),
        ]
    }
}

struct Floor {
    black_tiles: AHashSet<Coords>,
}

impl Floor {
    fn parse<S, I>(lines: I) -> Result<Self, ParseCoordsError>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let mut black_tiles = AHashSet::new();
        for line in lines {
            let coordinates = Coords::parse_line(line.as_ref())?;
            if !black_tiles.remove(&coordinates) {
                black_tiles.insert(coordinates);
            }
        }

        Ok(Self { black_tiles })
    }

    pub fn count_black_tiles(&self) -> usize {
        self.black_tiles.len()
    }

    pub fn update(&mut self) {
        let mut new_tiles = AHashSet::with_capacity(self.count_black_tiles() * 2);
        let mut white_tile_check = AHashSet::with_capacity(self.count_black_tiles() * 6);

        for coordinates in self.black_tiles.iter() {
            let mut neighbor_count = 0;
            for neighbor in coordinates.get_neighbors().iter() {
                if self.black_tiles.contains(neighbor) {
                    neighbor_count += 1;
                } else {
                    white_tile_check.insert(*neighbor);
                }
            }

            if matches!(neighbor_count, 1 | 2) {
                new_tiles.insert(*coordinates);
            }
        }

        for coordinates in white_tile_check.drain() {
            if coordinates
                .get_neighbors()
                .iter()
                .filter(|n| self.black_tiles.contains(n))
                .count()
                == 2
            {
                new_tiles.insert(coordinates);
            }
        }

        self.black_tiles = new_tiles;
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut floor = {
        let path = ["data", "day24", "input.txt"].iter().collect::<PathBuf>();
        let file = File::open(path)?;
        Floor::parse(BufReader::new(file).lines().filter_map(Result::ok))?
    };

    println!("Part 1: result = {}", floor.count_black_tiles());
    for _ in 0..100 {
        floor.update();
    }
    println!("Part 2: result = {}", floor.count_black_tiles());

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
    use super::Floor;

    const TEST_INPUT: &str = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn part1_test() {
        let floor = Floor::parse(TEST_INPUT.lines()).unwrap();
        assert_eq!(floor.count_black_tiles(), 10);
    }

    const EXAMPLE_TILES: [(usize, usize); 19] = [
        (1, 15),
        (2, 12),
        (3, 25),
        (4, 14),
        (5, 23),
        (6, 28),
        (7, 41),
        (8, 37),
        (9, 49),
        (10, 37),
        (20, 132),
        (30, 259),
        (40, 406),
        (50, 566),
        (60, 788),
        (70, 1106),
        (80, 1373),
        (90, 1844),
        (100, 2208),
    ];

    #[test]
    fn part2_test() {
        let mut floor = Floor::parse(TEST_INPUT.lines()).unwrap();
        for i in 0..100 {
            floor.update();
            if let Some((_, expected)) = EXAMPLE_TILES.iter().find(|(n, _)| *n == i + 1) {
                assert_eq!(floor.count_black_tiles(), *expected);
            }
        }
    }
}
