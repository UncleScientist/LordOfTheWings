use std::{collections::HashSet, fmt::Display};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q08_p1.txt").expect("file");
    let field = Field::new(data.lines());
    println!("Quest 8, part 1 = {}", field.energy_required_from());
}

#[derive(Debug)]
struct Field {
    grid: Vec<Plot>,
    width: usize,
    height: usize,
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

    fn get(&self, row: usize, col: usize) -> Plot {
        self.grid[row * self.width + col]
    }

    fn energy_required_from(&self) -> usize {
        let mut energy_needed = 0;

        let mut seen = HashSet::new();

        for row in 0..self.height {
            for col in 0..self.width {
                if !self.get(row, col).is_ent() {
                    continue;
                }
                let mut shadow_height = 0;
                if seen.insert((row, col)) {
                    let (mut diag_row, mut diag_col) = (row, col);
                    while diag_row < self.height && diag_col < self.width {
                        seen.insert((diag_row, diag_col));
                        match self.get(diag_row, diag_col) {
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
                        diag_row += 1;
                        diag_col += 1;
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_grid() {
        let field = Field::new(include_str!("../test-input/part1").lines());
        assert_eq!(Plot::Empty, field.get(0, 0));
        assert_eq!(Plot::Ent(1), field.get(0, 4));
        assert_eq!(Plot::Ent(8), field.get(8, 12));
    }

    #[test]
    fn test_part1() {
        let field = Field::new(include_str!("../test-input/part1").lines());
        assert_eq!(32, field.energy_required_from());
    }
}
