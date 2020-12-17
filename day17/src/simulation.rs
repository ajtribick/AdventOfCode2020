use std::{cmp::min, error::Error, fmt};

#[derive(Debug)]
pub struct ParseSimulationError(&'static str);

impl fmt::Display for ParseSimulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Simulation parse error: {}", self.0)
    }
}

impl Error for ParseSimulationError {}

#[derive(Debug, Clone, Copy)]
enum Cube {
    Inactive,
    Active,
}

impl Cube {
    pub fn is_active(&self) -> bool {
        match self {
            Self::Active => true,
            _ => false,
        }
    }
}

fn coords_to_idx(coords: &[usize], axes: &[usize]) -> usize {
    let mut idx = coords[0];
    let mut step = 1;
    for (c, a) in coords[1..].iter().zip(axes.iter()) {
        step *= a;
        idx += c * step;
    }
    idx
}

fn update_in_axes(pos: &mut [usize], axes: &[usize]) -> bool {
    assert_eq!(pos.len(), axes.len());
    for d in 0..pos.len() {
        pos[d] += 1;
        if pos[d] == axes[d] {
            pos[d] = 0;
        } else {
            return false;
        }
    }

    return true;
}

fn update_in_range(pos: &mut [usize], start: &[usize], end: &[usize]) -> bool {
    assert_eq!(pos.len(), start.len());
    assert_eq!(pos.len(), end.len());
    for d in 0..pos.len() {
        pos[d] += 1;
        if pos[d] == end[d] {
            pos[d] = start[d];
        } else {
            return false;
        }
    }

    return true;
}

#[derive(Debug)]
pub struct Simulation {
    data: Vec<Cube>,
    axes: Vec<usize>,
}

impl Simulation {
    pub fn parse(s: &str, dimensions: usize) -> Result<Self, ParseSimulationError> {
        if dimensions < 2 {
            return Err(ParseSimulationError("Needs at least two dimensions"));
        }

        let mut axes = vec![1; dimensions];

        let lines = s.lines().collect::<Vec<_>>();
        if lines.len() == 0 {
            return Err(ParseSimulationError("Empty grid"));
        }
        axes[1] = lines.len();

        if lines[0].len() == 0 {
            return Err(ParseSimulationError("No row data"));
        }
        axes[0] = lines[0].len();
        if lines.iter().any(|line| line.len() != axes[0]) {
            return Err(ParseSimulationError("Inconsistent widths"));
        }

        let data = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '.' => Ok(Cube::Inactive),
                '#' => Ok(Cube::Active),
                _ => Err(ParseSimulationError("Unexpected character")),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { data, axes })
    }

    pub fn active_count(&self) -> usize {
        self.data.iter().filter(|&c| c.is_active()).count()
    }

    fn get_src_pos(&self, dest_pos: &[usize], src_pos: &mut [usize]) {
        src_pos
            .iter_mut()
            .zip(dest_pos.iter().zip(self.axes.iter()))
            .for_each(|(s, (d, a))| {
                *s = if (1..=*a).contains(d) {
                    d - 1
                } else {
                    usize::MAX
                }
            });
    }

    fn get_range(&self, dest_pos: &[usize], start: &mut [usize], end: &mut [usize]) {
        dest_pos
            .iter()
            .zip(self.axes.iter())
            .zip(start.iter_mut().zip(end.iter_mut()))
            .for_each(|((x, a), (s, e))| {
                *s = x.saturating_sub(2);
                *e = min(x + 1, *a);
            });
    }

    fn check_neighbors(
        &self,
        src_pos: &[usize],
        start: &[usize],
        end: &[usize],
        scratch_pos: &mut [usize],
    ) -> (Cube, usize) {
        assert_eq!(src_pos.len(), start.len());
        assert_eq!(src_pos.len(), end.len());
        assert_eq!(src_pos.len(), scratch_pos.len());

        scratch_pos.copy_from_slice(&start);

        let mut current_cube = Cube::Inactive;
        let mut active_count = 0;
        loop {
            let j = coords_to_idx(scratch_pos, &self.axes);
            if scratch_pos == src_pos {
                current_cube = self.data[j];
            } else if self.data[j].is_active() {
                active_count += 1;
            }

            if update_in_range(scratch_pos, start, end) {
                return (current_cube, active_count);
            }
        }
    }

    pub fn update(&mut self) {
        let new_axes = self.axes.iter().map(|a| a + 2).collect::<Vec<_>>();
        let mut new_data = vec![Cube::Inactive; new_axes.iter().product()];

        let mut src_pos = vec![0; new_axes.len()];
        let mut dest_pos = vec![0; new_axes.len()];
        let mut scratch_pos = vec![0; new_axes.len()];
        let mut start = vec![0; new_axes.len()];
        let mut end = vec![0; new_axes.len()];

        for i in 0..new_data.len() {
            self.get_src_pos(&dest_pos, &mut src_pos);
            self.get_range(&dest_pos, &mut start, &mut end);

            let (current_cube, active_count) =
                self.check_neighbors(&src_pos, &start, &end, &mut scratch_pos);

            new_data[i] = match current_cube {
                Cube::Inactive if active_count == 3 => Cube::Active,
                Cube::Active if !(2..=3).contains(&active_count) => Cube::Inactive,
                _ => current_cube,
            };

            update_in_axes(&mut dest_pos, &new_axes);
        }

        self.axes = new_axes;
        self.data = new_data;
    }
}

#[cfg(test)]
mod test {
    use super::Simulation;

    const EXAMPLE: &str = r".#.
..#
###";

    #[test]
    fn one_step_3d() {
        let mut simulation = Simulation::parse(EXAMPLE, 3).unwrap();
        simulation.update();
        assert_eq!(simulation.active_count(), 11);
    }

    #[test]
    fn six_steps_3d() {
        let mut simulation = Simulation::parse(EXAMPLE, 3).unwrap();
        for _ in 0..6 {
            simulation.update();
        }
        assert_eq!(simulation.active_count(), 112);
    }

    #[test]
    fn six_steps_4d() {
        let mut simulation = Simulation::parse(EXAMPLE, 4).unwrap();
        for _ in 0..6 {
            simulation.update();
        }
        assert_eq!(simulation.active_count(), 848);
    }
}
