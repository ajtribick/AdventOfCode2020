use std::{cmp::Ordering, error::Error, fmt};

use crate::tile::{parse_tiles, ParseTileError};

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

fn grid_size(count: usize) -> Option<usize> {
    for size in 0usize.. {
        if let Some(sqr_size) = size.checked_mul(size) {
            match sqr_size.cmp(&count) {
                Ordering::Less => (),
                Ordering::Equal => return Some(size),
                Ordering::Greater => return None,
            }
        } else {
            return None;
        }
    }

    unreachable!()
}

pub struct Grid {
    corners: Vec<u64>,
}

impl Grid {
    pub fn parse<S, I>(lines: I) -> Result<Self, ParseGridError>
    where
        S: AsRef<str>,
        I: Iterator<Item = S>,
    {
        let tiles = parse_tiles(lines).map_err(ParseGridError::TileError)?;
        let size = grid_size(tiles.len()).ok_or(ParseGridError::GridError("Non-square grid"))?;

        let mut corners = Vec::with_capacity(4);
        let mut edges = 0;
        for (id, tile) in tiles.iter() {
            let connect_count = tiles
                .iter()
                .filter(|(other_id, other_tile)| *id != **other_id && tile.can_connect(other_tile))
                .count();
            match connect_count {
                2 => corners.push(*id),
                3 => edges += 1,
                4 => (),
                _ => return Err(ParseGridError::GridError("Ambiguous connections")),
            }
        }

        if corners.len() != 4 {
            return Err(ParseGridError::GridError("Incorrect corner count"));
        }
        if edges != size.saturating_sub(2) * 4 {
            return Err(ParseGridError::GridError("Incorrect edge count"));
        }

        Ok(Self { corners })
    }

    pub fn corner_ids(&self) -> &[u64] {
        &self.corners
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
}
