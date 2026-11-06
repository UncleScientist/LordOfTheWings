use std::{collections::HashSet, fmt::Display, ops::AddAssign};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q08_p1.txt").expect("file");
    let field = Field::new(data.lines());
    println!("Quest 8, part 1 = {}", field.part1_energy());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q08_p2.txt").expect("file");
    let field = Field::new(data.lines());
    println!("Quest 8, part 1 = {}", field.part2_energy());
}

#[derive(Debug)]
struct Field {
    grid: Vec<Plot>,
    width: usize,
    height: usize,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Index {
    Row(usize),
    Col(usize),
    OutOfBounds,
}

impl AddAssign<isize> for Index {
    fn add_assign(&mut self, rhs: isize) {
        match self {
            Index::Row(r) => {
                if let Some(val) = r.checked_add_signed(rhs) {
                    *r = val
                } else {
                    *self = Index::OutOfBounds
                }
            }
            Index::Col(c) => {
                if let Some(val) = c.checked_add_signed(rhs) {
                    *c = val
                } else {
                    *self = Index::OutOfBounds
                }
            }
            Index::OutOfBounds => {}
        };
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Plot {
    Ent(u8),
    Empty,
}

impl Plot {
    fn is_ent(&self) -> bool {
        matches!(self, Self::Ent(_))
    }
}

impl From<char> for Plot {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '1'..='9' => Self::Ent(value as u8 - b'0'),
            _ => panic!("invalid value {value}"),
        }
    }
}

impl Display for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Plot::Ent(x) => write!(f, "{x}"),
            Plot::Empty => write!(f, "."),
        }
    }
}

impl Field {
    fn new<'a, I>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut grid = Vec::<Plot>::new();
        let mut width = 0;
        let mut height = 0;

        for (row, line) in lines.into_iter().enumerate() {
            grid.extend(line.chars().map(<char as Into<Plot>>::into));
            if height == 0 {
                width = grid.len();
            }
            height = row + 1;
        }

        Self {
            grid,
            width,
            height,
        }
    }

    fn get(&self, first: Index, second: Index) -> Plot {
        let (row, col) = match (first, second) {
            (Index::Row(r), Index::Col(c)) => (r, c),
            (Index::Col(c), Index::Row(r)) => (r, c),
            _ => panic!("two rows or two cols"),
        };
        self.grid[row * self.width + col]
    }

    fn part1_energy(&self) -> usize {
        self.energy_from_dir(
            (0..self.height).map(Index::Row),
            (0..self.width).map(Index::Col),
            1,
            1,
        )
    }

    fn part2_energy(&self) -> usize {
        let forward_row = (0..self.height).map(Index::Row);
        let forward_col = (0..self.width).map(Index::Col);
        let reverse_row = (0..self.height).rev().map(Index::Row);
        let reverse_col = (0..self.width).rev().map(Index::Col);

        self.energy_from_dir(forward_row.clone(), forward_col.clone(), 1, 1)
            + self.energy_from_dir(forward_row.clone(), reverse_col.clone(), 1, -1)
            + self.energy_from_dir(reverse_col.clone(), forward_row.clone(), -1, 0)
            + self.energy_from_dir(reverse_row.clone(), reverse_col.clone(), -1, -1)
            + self.energy_from_dir(reverse_row.clone(), forward_col.clone(), -1, 1)
            + self.energy_from_dir(forward_col.clone(), forward_row.clone(), 1, 0)
    }

    fn energy_from_dir<O, I>(
        &self,
        outer_iter: O,
        inner_iter: I,
        delta_outer: isize,
        delta_inner: isize,
    ) -> usize
    where
        O: Iterator<Item = Index>,
        I: IntoIterator<Item = Index> + Clone,
    {
        let mut energy_needed = 0;

        let mut seen = HashSet::new();

        for outer in outer_iter {
            let ii = inner_iter.clone();
            for inner in ii.into_iter() {
                if !self.get(outer, inner).is_ent() {
                    continue;
                }
                let mut shadow_height = 0;
                if seen.insert((outer, inner)) {
                    let (mut diag_outer, mut diag_inner) = (outer, inner);
                    while self.in_bounds(diag_outer, diag_inner) {
                        seen.insert((diag_outer, diag_inner));
                        match self.get(diag_outer, diag_inner) {
                            Plot::Ent(h) => {
                                if h > shadow_height {
                                    energy_needed += 1;
                                }
                                if shadow_height > 0 {
                                    shadow_height = (shadow_height - 1).max(h - 1);
                                } else {
                                    shadow_height = h - 1;
                                }
                            }
                            Plot::Empty => {}
                        }
                        diag_outer += delta_outer;
                        diag_inner += delta_inner;
                    }
                }
            }
        }

        energy_needed
    }

    fn _print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!("{}", self.grid[row * self.width + col]);
            }
            println!();
        }
    }

    fn in_bounds(&self, first: Index, second: Index) -> bool {
        let (row, col) = match (first, second) {
            (Index::Row(r), Index::Col(c)) => (r, c),
            (Index::Col(c), Index::Row(r)) => (r, c),
            (Index::OutOfBounds, _) => return false,
            (_, Index::OutOfBounds) => return false,
            _ => panic!("two rows or two cols"),
        };
        row < self.height && col < self.width
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_grid() {
        let field = Field::new(include_str!("../test-input/part1").lines());
        assert_eq!(Plot::Empty, field.get(Index::Row(0), Index::Col(0)));
        assert_eq!(Plot::Ent(1), field.get(Index::Row(0), Index::Col(4)));
        assert_eq!(Plot::Ent(8), field.get(Index::Col(12), Index::Row(8)));
    }

    #[test]
    fn test_part1() {
        let field = Field::new(include_str!("../test-input/part1").lines());
        assert_eq!(32, field.part1_energy());
    }

    #[test]
    fn test_part2() {
        let field = Field::new(include_str!("../test-input/part2").lines());
        assert_eq!(187, field.part2_energy());
    }

    #[test]
    fn test_individual_rounds_part2() {
        let field = Field::new(include_str!("../test-input/part2").lines());

        let forward_row = (0..field.height).map(|row| Index::Row(row));
        let forward_col = (0..field.width).map(|col| Index::Col(col));
        let reverse_row = (0..field.height).rev().map(|row| Index::Row(row));
        let reverse_col = (0..field.width).rev().map(|col| Index::Col(col));

        assert_eq!(
            32,
            field.energy_from_dir(forward_row.clone(), forward_col.clone(), 1, 1),
        );
        assert_eq!(
            32,
            field.energy_from_dir(forward_row.clone(), reverse_col.clone(), 1, -1),
        );
        assert_eq!(
            29,
            field.energy_from_dir(reverse_col.clone(), forward_row.clone(), -1, 0),
        );
        assert_eq!(
            32,
            field.energy_from_dir(reverse_row.clone(), reverse_col.clone(), -1, -1),
        );
        assert_eq!(
            30,
            field.energy_from_dir(reverse_row.clone(), forward_col.clone(), -1, 1),
        );
        assert_eq!(
            32,
            field.energy_from_dir(forward_col.clone(), forward_row.clone(), 1, 0),
        );
    }
}
