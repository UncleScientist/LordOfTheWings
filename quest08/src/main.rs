use std::{collections::HashSet, fmt::Display, ops::AddAssign};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q08_p1.txt").expect("file");
    let field = Field::new(data.lines());
    println!("Quest 8, part 1 = {}", field.part1_energy());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q08_p2.txt").expect("file");
    let field = Field::new(data.lines());
    println!("Quest 8, part 2 = {}", field.part2_energy());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q08_p3.txt").expect("file");
    let field = Field::new(data.lines());
    println!("Quest 8, part 3 = {}", field.part3_energy());
}

#[derive(Debug, Clone)]
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
    Seed(u8),
    Empty,
}

impl Plot {
    fn is_ent(&self) -> bool {
        matches!(self, Self::Ent(_) | Self::Seed(_))
    }
}

impl From<char> for Plot {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '1'..='9' => Self::Ent(value as u8 - b'0'),
            'A'..='D' => Self::Seed(value as u8 - b'A'),
            _ => panic!("invalid value {value}"),
        }
    }
}

impl Display for Plot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Plot::Ent(x) => write!(f, "{x}"),
            Plot::Empty => write!(f, "."),
            Plot::Seed(v) => match v {
                0 => write!(f, "A"),
                1 => write!(f, "B"),
                2 => write!(f, "C"),
                3 => write!(f, "D"),
                _ => panic!("invalid seed {v}"),
            },
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

    fn get_rc(first: Index, second: Index) -> (usize, usize) {
        match (first, second) {
            (Index::Row(r), Index::Col(c)) => (r, c),
            (Index::Col(c), Index::Row(r)) => (r, c),
            _ => panic!("two rows or two cols, or out of bounds: {first:?}, {second:?}"),
        }
    }

    fn get(&self, first: Index, second: Index) -> Plot {
        let (row, col) = Self::get_rc(first, second);
        self.grid[row * self.width + col]
    }

    fn set(&mut self, first: Index, second: Index, plot: Plot) {
        let (row, col) = Self::get_rc(first, second);
        self.grid[row * self.width + col] = plot
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

    fn part1_energy(&self) -> usize {
        self.energy_from_dir(
            (0..self.height).map(Index::Row),
            (0..self.width).map(Index::Col),
            1,
            1,
        )
        .0
    }

    // Southeast
    fn round1(&self) -> (usize, usize, Self) {
        let forward_row = (0..self.height).map(Index::Row);
        let forward_col = (0..self.width).map(Index::Col);
        self.energy_from_dir(forward_row, forward_col, 1, 1)
    }

    // southwest
    fn round2(&self) -> (usize, usize, Self) {
        let forward_row = (0..self.height).map(Index::Row);
        let reverse_col = (0..self.width).rev().map(Index::Col);
        self.energy_from_dir(forward_row, reverse_col, 1, -1)
    }

    // west
    fn round3(&self) -> (usize, usize, Self) {
        let reverse_col = (0..self.width).rev().map(Index::Col);
        let forward_row = (0..self.height).map(Index::Row);
        self.energy_from_dir(reverse_col, forward_row, -1, 0)
    }

    // northweat
    fn round4(&self) -> (usize, usize, Self) {
        let reverse_row = (0..self.height).rev().map(Index::Row);
        let reverse_col = (0..self.width).rev().map(Index::Col);
        self.energy_from_dir(reverse_row, reverse_col, -1, -1)
    }

    // northeast
    fn round5(&self) -> (usize, usize, Self) {
        let reverse_row = (0..self.height).rev().map(Index::Row);
        let forward_col = (0..self.width).map(Index::Col);
        self.energy_from_dir(reverse_row, forward_col, -1, 1)
    }

    // east
    fn round6(&self) -> (usize, usize, Self) {
        let forward_col = (0..self.width).map(Index::Col);
        let forward_row = (0..self.height).map(Index::Row);
        self.energy_from_dir(forward_col, forward_row, 1, 0)
    }

    fn part2_energy(&self) -> usize {
        self.round1().0
            + self.round2().0
            + self.round3().0
            + self.round4().0
            + self.round5().0
            + self.round6().0
    }

    fn part3_energy(&self) -> usize {
        let mut total_ents = 0;
        let mut total_energy = 0;

        type FuncCall = dyn Fn(&Field) -> (usize, usize, Field);

        let funcs: Vec<&FuncCall> = vec![
            &Self::round1,
            &Self::round2,
            &Self::round3,
            &Self::round4,
            &Self::round5,
            &Self::round6,
        ];

        let mut current_field = self.clone();
        let mut idx = 0;
        while total_ents < 100_000 {
            let (energy, ents, next_field) = funcs[idx](&current_field);
            total_ents += ents;
            total_energy += energy;
            current_field = next_field;
            idx = (idx + 1) % funcs.len();
        }

        total_energy
    }

    fn energy_from_dir<O, I>(
        &self,
        outer_iter: O,
        inner_iter: I,
        delta_outer: isize,
        delta_inner: isize,
    ) -> (usize, usize, Self)
    where
        O: Iterator<Item = Index>,
        I: IntoIterator<Item = Index> + Clone,
    {
        let mut energy_needed = 0;
        let mut fully_grown_ents = 0;

        let mut next_field = self.clone();

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
                                    if h == 9 {
                                        fully_grown_ents += 1;
                                        next_field.set(diag_outer, diag_inner, Plot::Seed(3));
                                    } else {
                                        next_field.set(diag_outer, diag_inner, Plot::Ent(h + 1));
                                    }
                                }
                                if shadow_height > 0 {
                                    shadow_height = (shadow_height - 1).max(h - 1);
                                } else {
                                    shadow_height = h - 1;
                                }
                            }
                            Plot::Empty => {}
                            Plot::Seed(s) => {
                                if s == 1 {
                                    next_field.set(diag_outer, diag_inner, Plot::Ent(1));
                                } else {
                                    next_field.set(diag_outer, diag_inner, Plot::Seed(s - 1));
                                }
                                shadow_height = shadow_height.saturating_sub(1);
                            }
                        }
                        diag_outer += delta_outer;
                        diag_inner += delta_inner;
                    }
                }
            }
        }

        (energy_needed, fully_grown_ents, next_field)
    }

    fn _print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!("{}", self.grid[row * self.width + col]);
            }
            println!();
        }
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

        assert_eq!(32, field.round1().0);
        assert_eq!(32, field.round2().0);
        assert_eq!(29, field.round3().0);
        assert_eq!(32, field.round4().0);
        assert_eq!(30, field.round5().0);
        assert_eq!(32, field.round6().0);
    }

    #[test]
    fn test_part3() {
        let field = Field::new(include_str!("../test-input/part3").lines());
        assert_eq!(899889, field.part3_energy());
    }
}
