use std::{error::Error, fmt};

use crate::{
    tile::{parse_tiles, EdgeConstraints, ParseTileError, Tile},
    utils::sqrt_exact,
};

#[derive(Debug)]
pub enum ParseGridError {
    GridError(&'static str),
    TileError(ParseTileError),
}

impl fmt::Display for ParseGridError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GridError(s) => write!(f, "Grid error: {}", s),
            Self::TileError(e) => write!(f, "{}", e),
        }
    }
}

impl Error for ParseGridError {}

fn find_corner(parsed_tiles: &mut [Tile]) -> Option<usize> {
    let (corner, edges1, edges2) = parsed_tiles.iter().enumerate().find_map(|(idx, tile)| {
        let mut connected = parsed_tiles
            .iter()
            .filter(|t| t.id() != tile.id())
            .map(|t| tile.connect(t))
            .filter(|c| !c.is_empty());
        let first = connected.next()?;
        let second = connected.next()?;
        connected
            .next()
            .map_or_else(|| Some((idx, first, second)), |_| None)
    })?;

    let oriented = edges1
        .iter()
        .copied()
        .flat_map(|e1| edges2.iter().copied().map(move |e2| (e1, e2)))
        .any(|(e1, e2)| parsed_tiles[corner].orient(EdgeConstraints::right(e1).and_bottom(e2)));

    if oriented {
        Some(corner)
    } else {
        None
    }
}

fn build_grid(
    parsed_tiles: &mut Vec<Tile>,
    corner: usize,
    size: usize,
) -> Result<Vec<Tile>, ParseGridError> {
    let mut tiles = Vec::with_capacity(parsed_tiles.len());

    tiles.push(parsed_tiles.remove(corner));

    while !parsed_tiles.is_empty() {
        let idx = tiles.len();
        let mut constraints = EdgeConstraints::default();
        if idx % size != 0 {
            constraints.and_left(tiles[idx - 1].right_edge());
        }

        if idx >= size {
            constraints.and_top(tiles[idx - size].bottom_edge());
        }

        let mut success = false;
        for src_idx in 0..parsed_tiles.len() {
            if parsed_tiles[src_idx].orient(&constraints) {
                tiles.push(parsed_tiles.remove(src_idx));
                success = true;
                break;
            }
        }

        if !success {
            return Err(ParseGridError::GridError("Ambiguous grid edge constraints"));
        }
    }

    Ok(tiles)
}

pub struct Grid {
    size: usize,
    tile_size: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn parse<S, I>(lines: I) -> Result<Self, ParseGridError>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let mut parsed_tiles = parse_tiles(lines).map_err(ParseGridError::TileError)?;
        let size =
            sqrt_exact(parsed_tiles.len()).ok_or(ParseGridError::GridError("Non-square grid"))?;

        let tile_size = parsed_tiles[0].size();

        let corner = find_corner(&mut parsed_tiles).ok_or(ParseGridError::GridError(
            "Could not find suitable top-left corner",
        ))?;

        let tiles = build_grid(&mut parsed_tiles, corner, size)?;

        Ok(Self {
            size,
            tile_size,
            tiles,
        })
    }

    pub fn corner_ids(&self) -> [u64; 4] {
        [
            self.tiles[0].id(),
            self.tiles[self.size - 1].id(),
            self.tiles[self.tiles.len() - self.size].id(),
            self.tiles[self.tiles.len() - 1].id(),
        ]
    }

    pub fn merge_tiles(&self) -> Tile {
        let inner_size = self.tile_size - 2;
        let mut tile_data = Vec::with_capacity(self.size * self.size * inner_size * inner_size);
        for grid_row in self.tiles.chunks(self.size) {
            for row in 1..=inner_size {
                for tile in grid_row.iter() {
                    let inner_start = self.tile_size * row + 1;
                    let inner_end = self.tile_size * (row + 1) - 1;
                    for &element in tile.data()[inner_start..inner_end].iter() {
                        tile_data.push(element);
                    }
                }
            }
        }

        Tile::from_data(&tile_data, 0).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Grid;

    const EXAMPLE_DATA: &str = include_str!("test_input.txt");

    #[test]
    fn test_corners() {
        let grid = Grid::parse(EXAMPLE_DATA.lines()).unwrap();
        let expected = [1951, 3079, 2971, 1171];
        assert_eq!(grid.corner_ids().len(), 4);
        assert!(grid.corner_ids().iter().all(|id| expected.contains(id)));
    }

    const EXAMPLE_MERGED: &str = r".#.#..#.##...#.##..#####
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
    fn merge_tiles_test() {
        let grid = Grid::parse(EXAMPLE_DATA.lines()).unwrap();
        let expected = EXAMPLE_MERGED
            .lines()
            .flat_map(|line| line.chars().map(|c| c == '#'))
            .collect::<Vec<_>>();
        let result = grid.merge_tiles();
        assert_eq!(result.data(), expected);
    }
}
