use std::{error::Error, fmt};

use crate::utils::sqrt_exact;

lazy_static! {
    static ref MONSTER_PATTERN: Vec<Vec<bool>> = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   "
    ]
    .iter()
    .map(|line| line.chars().map(|c| c == '#').collect())
    .collect();
    static ref MONSTER_HEIGHT: usize = MONSTER_PATTERN.len();
    static ref MONSTER_WIDTH: usize = MONSTER_PATTERN[0].len();
}

#[derive(Debug)]
pub struct ParseTileError(&'static str);

impl fmt::Display for ParseTileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl Error for ParseTileError {}

pub type TileData = Vec<bool>;

fn write_row(data: &mut TileData, row: &str) -> Result<(), ParseTileError> {
    for c in row.chars() {
        match c {
            '.' => data.push(false),
            '#' => data.push(true),
            _ => return Err(ParseTileError("Unknown character")),
        }
    }

    Ok(())
}

fn parse_id(line: &str) -> Result<u64, ParseTileError> {
    line.strip_prefix("Tile ")
        .and_then(|s| s.strip_suffix(':'))
        .and_then(|s| s.parse().ok())
        .ok_or(ParseTileError("Could not parse id"))
}

fn check_line(row: &[bool], monster_row: &[bool]) -> bool {
    row.iter()
        .copied()
        .zip(monster_row.iter())
        .all(|(t, m)| t || !m)
}

fn update_line(row: &mut [bool], monster_row: &[bool]) {
    row.iter_mut()
        .zip(monster_row.iter())
        .for_each(|(t, m)| *t &= !m);
}

#[derive(Debug)]
enum HorizontalEdge {
    Left,
    Right,
}

#[derive(Debug)]
enum VerticalEdge {
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct EdgeConstraints {
    left: Option<u32>,
    right: Option<u32>,
    top: Option<u32>,
    bottom: Option<u32>,
}

impl EdgeConstraints {
    pub fn right(value: u32) -> Self {
        Self {
            right: Some(value),
            ..Default::default()
        }
    }

    pub fn and_left(&mut self, value: u32) -> &Self {
        self.left = Some(value);
        self
    }

    pub fn and_top(&mut self, value: u32) -> &Self {
        self.top = Some(value);
        self
    }

    pub fn and_bottom(&mut self, value: u32) -> &Self {
        self.bottom = Some(value);
        self
    }
}

impl Default for EdgeConstraints {
    fn default() -> Self {
        Self {
            left: None,
            right: None,
            top: None,
            bottom: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tile {
    id: u64,
    size: usize,
    data: TileData,
}

impl Tile {
    pub fn parse<S, I>(lines: &mut I, id: u64) -> Result<Self, ParseTileError>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let row_ref = lines.next().ok_or(ParseTileError("Missing tile data"))?;
        let first_row = row_ref.as_ref();
        let size = first_row.len();
        if size == 0 {
            return Err(ParseTileError("Empty tile"));
        } else if size > 32 {
            return Err(ParseTileError("Tiles larger than 32x32 not supported"));
        }

        let mut data = TileData::with_capacity(size * size);
        write_row(&mut data, first_row)?;

        let mut rows = 1;
        for row_ref in lines.take(size - 1) {
            write_row(&mut data, row_ref.as_ref())?;
            rows += 1;
        }

        if rows != size {
            return Err(ParseTileError("Incomplete tile"));
        }

        let tile = Self { id, size, data };

        let mut edge_values = [
            tile.row_fwd(VerticalEdge::Top),
            tile.row_fwd(VerticalEdge::Bottom),
            tile.row_rev(VerticalEdge::Top),
            tile.row_rev(VerticalEdge::Bottom),
            tile.col_fwd(HorizontalEdge::Left),
            tile.col_fwd(HorizontalEdge::Right),
            tile.col_rev(HorizontalEdge::Left),
            tile.col_rev(HorizontalEdge::Right),
        ];
        edge_values.sort_unstable();
        if edge_values.windows(2).all(|w| w[0] != w[1]) {
            Ok(tile)
        } else {
            Err(ParseTileError("Ambiguous edge values"))
        }
    }

    pub fn from_data(data: &[bool], id: u64) -> Result<Self, ParseTileError> {
        let size = sqrt_exact(data.len()).ok_or(ParseTileError("Tile is not square"))?;
        Ok(Self {
            id,
            size,
            data: data.to_vec(),
        })
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn data(&self) -> &[bool] {
        &self.data
    }

    fn row_fwd(&self, edge: VerticalEdge) -> u32 {
        let row_start = match edge {
            VerticalEdge::Top => 0,
            VerticalEdge::Bottom => self.data.len() - self.size,
        };

        self.data[row_start..row_start + self.size]
            .iter()
            .fold(0, |acc, &b| (acc << 1) + b as u32)
    }

    fn row_rev(&self, edge: VerticalEdge) -> u32 {
        let row_start = match edge {
            VerticalEdge::Top => 0,
            VerticalEdge::Bottom => self.data.len() - self.size,
        };

        self.data[row_start..row_start + self.size]
            .iter()
            .rev()
            .fold(0, |acc, &b| (acc << 1) + b as u32)
    }

    fn col_fwd(&self, edge: HorizontalEdge) -> u32 {
        let col = match edge {
            HorizontalEdge::Left => 0,
            HorizontalEdge::Right => self.size - 1,
        };

        self.data[col..]
            .iter()
            .step_by(self.size)
            .fold(0, |acc, &b| (acc << 1) + b as u32)
    }

    fn col_rev(&self, edge: HorizontalEdge) -> u32 {
        let col = match edge {
            HorizontalEdge::Left => 0,
            HorizontalEdge::Right => self.size - 1,
        };

        self.data[col..]
            .iter()
            .step_by(self.size)
            .rev()
            .fold(0, |acc, &b| (acc << 1) + b as u32)
    }

    pub fn right_edge(&self) -> u32 {
        self.col_fwd(HorizontalEdge::Right)
    }

    pub fn bottom_edge(&self) -> u32 {
        self.row_fwd(VerticalEdge::Bottom)
    }

    pub fn connect(&self, other: &Tile) -> Vec<u32> {
        let edges = [
            self.row_fwd(VerticalEdge::Top),
            self.row_fwd(VerticalEdge::Bottom),
            self.col_fwd(HorizontalEdge::Left),
            self.col_fwd(HorizontalEdge::Right),
            self.row_rev(VerticalEdge::Top),
            self.row_rev(VerticalEdge::Bottom),
            self.col_rev(HorizontalEdge::Left),
            self.col_rev(HorizontalEdge::Right),
        ];

        let other_edges = [
            other.row_fwd(VerticalEdge::Top),
            other.row_fwd(VerticalEdge::Bottom),
            other.col_fwd(HorizontalEdge::Left),
            other.col_fwd(HorizontalEdge::Right),
            other.row_rev(VerticalEdge::Top),
            other.row_rev(VerticalEdge::Bottom),
            other.col_rev(HorizontalEdge::Left),
            other.col_rev(HorizontalEdge::Right),
        ];

        edges
            .iter()
            .filter(|e| other_edges.contains(e))
            .copied()
            .collect()
    }

    pub fn orient(&mut self, constraints: &EdgeConstraints) -> bool {
        use HorizontalEdge::{Left, Right};
        use VerticalEdge::{Bottom, Top};

        for i in 0..8 {
            let oriented = constraints.left.map_or(true, |l| l == self.col_fwd(Left))
                && constraints.right.map_or(true, |r| r == self.col_fwd(Right))
                && constraints.top.map_or(true, |t| t == self.row_fwd(Top))
                && constraints
                    .bottom
                    .map_or(true, |b| b == self.row_fwd(Bottom));
            if oriented {
                return true;
            }

            self.rotate_right();
            if i == 3 {
                self.flip_horizontal();
            }
        }

        false
    }

    pub fn flip_horizontal(&mut self) {
        self.data.chunks_mut(self.size).for_each(|r| r.reverse());
    }

    pub fn rotate_right(&mut self) {
        let src = self.data.clone();
        for (y, row) in src.chunks(self.size).enumerate() {
            self.data[self.size - y - 1..]
                .iter_mut()
                .step_by(self.size)
                .zip(row)
                .for_each(|(d, s)| *d = *s);
        }
    }

    pub fn remove_monsters(&mut self) {
        for i in 0..8 {
            let mut found_monsters = false;
            let mut rows = self.data.chunks_mut(self.size).collect::<Vec<_>>();
            for y in 0..rows.len() - *MONSTER_HEIGHT {
                let row_slice = &mut rows[y..y + *MONSTER_HEIGHT];
                for x in 0..self.size - *MONSTER_WIDTH {
                    let has_monster = row_slice
                        .iter()
                        .map(|r| &r[x..x + *MONSTER_WIDTH])
                        .zip(MONSTER_PATTERN.iter())
                        .all(|(row, monster_row)| check_line(row, monster_row));

                    if !has_monster {
                        continue;
                    }

                    found_monsters = true;
                    row_slice
                        .iter_mut()
                        .map(|r| &mut r[x..x + *MONSTER_WIDTH])
                        .zip(MONSTER_PATTERN.iter())
                        .for_each(|(row, monster_row)| update_line(row, monster_row));
                }
            }

            if found_monsters {
                break;
            }

            self.rotate_right();
            if i == 3 {
                self.flip_horizontal();
            }
        }
    }

    pub fn roughness(&self) -> usize {
        self.data.iter().filter(|d| **d).count()
    }
}

pub fn parse_tiles<S, I>(mut lines: I) -> Result<Vec<Tile>, ParseTileError>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    let mut tiles = Vec::new();
    let mut tile_size = 0;

    while let Some(row_data) = lines.next() {
        let row = row_data.as_ref();
        if row.is_empty() {
            continue;
        }

        let id = parse_id(row)?;
        let tile = Tile::parse(&mut lines, id)?;
        if tile_size == 0 {
            tile_size = tile.size;
        } else if tile.size != tile_size {
            return Err(ParseTileError("Inconsistent tile sizes"));
        }

        tiles.push(tile);
    }

    Ok(tiles)
}

#[cfg(test)]
mod tests {
    use super::{parse_tiles, Tile};

    const EXAMPLE_DATA: &str = include_str!("test_input.txt");
    const EXAMPLE_IDS: [u64; 9] = [2311, 1951, 1171, 1427, 1489, 2473, 2971, 2729, 3079];

    #[test]
    fn parse_test() {
        let tiles = parse_tiles(EXAMPLE_DATA.lines()).unwrap();
        assert_eq!(tiles.len(), 9);
        assert!(tiles
            .iter()
            .map(|t| t.id())
            .all(|id| EXAMPLE_IDS.contains(&id)));
    }

    #[test]
    fn connect_test() {
        let tiles = parse_tiles(EXAMPLE_DATA.lines()).unwrap();
        let expected_connections = [
            (1951, vec![2729, 2311]),
            (2311, vec![1951, 1427, 3079]),
            (3079, vec![2311, 2473]),
            (2729, vec![1951, 1427, 2971]),
            (1427, vec![2311, 2729, 2473, 1489]),
            (2473, vec![3079, 1427, 1171]),
            (2971, vec![2729, 1489]),
            (1489, vec![2971, 1427, 1171]),
            (1171, vec![1489, 2473]),
        ];

        for (id, expected) in expected_connections.iter() {
            let tile = tiles.iter().find(|t| t.id() == *id).unwrap();
            for other_tile in tiles.iter().filter(|t| t.id() != tile.id()) {
                assert_ne!(
                    tile.connect(other_tile).is_empty(),
                    expected.contains(&other_tile.id())
                );
            }
        }
    }

    #[test]
    fn flip_horizontal_test() {
        let mut tile = Tile {
            id: 0,
            size: 3,
            data: vec![true, false, true, true, true, false, false, false, true],
        };
        tile.flip_horizontal();
        let expected = vec![true, false, true, false, true, true, true, false, false];
        assert_eq!(tile.data, expected);
    }

    #[test]
    fn rotate_right_test() {
        let mut tile = Tile {
            id: 0,
            size: 3,
            data: vec![true, false, true, true, true, false, false, false, true],
        };
        tile.rotate_right();
        let expected = vec![false, true, true, false, true, false, true, false, true];
        assert_eq!(tile.data, expected);
    }

    const EXAMPLE_MONSTERS: &str = r".#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###";

    #[test]
    fn monsters_test() {
        let mut tile = Tile::from_data(
            EXAMPLE_MONSTERS
                .lines()
                .flat_map(|s| s.chars().map(|c| c == '#'))
                .collect::<Vec<_>>()
                .as_slice(),
            0,
        )
        .unwrap();

        tile.remove_monsters();

        let roughness = tile.roughness();
        assert_eq!(roughness, 273);
    }
}
