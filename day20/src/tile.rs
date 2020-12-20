use std::{error::Error, fmt};

use ahash::AHashMap;

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

#[derive(Debug)]
pub struct Tile {
    size: usize,
    data: TileData,
}

impl Tile {
    pub fn parse<S, I>(lines: &mut I) -> Result<Self, ParseTileError>
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

        Ok(Self { size, data })
    }

    fn row_fwd(&self, row: usize) -> u32 {
        let row_start = row * self.size;
        self.data[row_start..row_start + self.size]
            .iter()
            .fold(0, |acc, &b| (acc << 1) + b as u32)
    }

    fn row_rev(&self, row: usize) -> u32 {
        let row_start = row * self.size;
        self.data[row_start..row_start + self.size]
            .iter()
            .rev()
            .fold(0, |acc, &b| (acc << 1) + b as u32)
    }

    fn col_fwd(&self, col: usize) -> u32 {
        self.data[col..]
            .iter()
            .step_by(self.size)
            .fold(0, |acc, &b| (acc << 1) + b as u32)
    }

    fn col_rev(&self, col: usize) -> u32 {
        self.data[col..]
            .iter()
            .step_by(self.size)
            .rev()
            .fold(0, |acc, &b| (acc << 1) + b as u32)
    }

    pub fn can_connect(&self, other: &Tile) -> bool {
        let edges = [
            self.row_fwd(0),
            self.row_fwd(self.size - 1),
            self.row_rev(0),
            self.row_rev(self.size - 1),
            self.col_fwd(0),
            self.col_fwd(self.size - 1),
            self.col_rev(0),
            self.col_rev(self.size - 1),
        ];

        let other_edges = [
            other.row_fwd(0),
            other.row_fwd(other.size - 1),
            other.row_rev(0),
            other.row_rev(other.size - 1),
            other.col_fwd(0),
            other.col_fwd(other.size - 1),
            other.col_rev(0),
            other.col_rev(other.size - 1),
        ];

        edges.iter().any(|e| other_edges.contains(e))
    }
}

pub fn parse_tiles<S, I>(mut lines: I) -> Result<AHashMap<u64, Tile>, ParseTileError>
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
{
    let mut tiles = AHashMap::new();

    while let Some(row_data) = lines.next() {
        let row = row_data.as_ref();
        if row.is_empty() {
            continue;
        }
        let id = parse_id(row)?;
        tiles.insert(id, Tile::parse(&mut lines)?);
    }

    Ok(tiles)
}

#[cfg(test)]
mod tests {
    use super::parse_tiles;

    const EXAMPLE_DATA: &str = include_str!("test_input.txt");
    const EXAMPLE_IDS: [u64; 9] = [2311, 1951, 1171, 1427, 1489, 2473, 2971, 2729, 3079];

    #[test]
    fn parse_test() {
        let tiles = parse_tiles(EXAMPLE_DATA.lines()).unwrap();
        assert_eq!(tiles.len(), 9);
        assert!(EXAMPLE_IDS.iter().all(|id| tiles.contains_key(id)));
    }

    #[test]
    fn can_connect_test() {
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
            let tile = tiles.get(id).unwrap();
            for (other_id, other_tile) in tiles.iter().filter(|(k, _)| **k != *id) {
                assert_eq!(tile.can_connect(other_tile), expected.contains(other_id));
            }
        }
    }
}
