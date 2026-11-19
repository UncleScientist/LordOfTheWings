use std::collections::{HashMap, HashSet};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q12_p1.txt").expect("file");
    let stacks = Stacks::new(data.lines());
    println!("Quest 12, part 1 = {}", stacks.min_spells());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q12_p2.txt").expect("file");
    let stacks = Stacks::new(data.lines());
    println!("Quest 12, part 2 = {}", stacks.min_spells());
}

#[derive(Debug)]
struct Block {
    blobs: HashSet<Point>,
    width: usize,
    height: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn up(&self) -> Point {
        Point {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Point {
        Point {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Point {
        Point {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(&self) -> Point {
        Point {
            row: self.row,
            col: self.col + 1,
        }
    }
}

impl Block {
    fn shoot_horizontal(&self) -> Self {
        Self {
            blobs: self
                .blobs
                .iter()
                .filter_map(|point| {
                    if point.row > 0 {
                        Some(Point::new(point.row - 1, point.col))
                    } else {
                        None
                    }
                })
                .collect(),
            width: self.width,
            height: self.height - 1,
        }
    }

    fn shoot_vertical(&self) -> Self {
        Self {
            blobs: self
                .blobs
                .iter()
                .filter_map(|point| {
                    if point.col > 0 {
                        Some(Point::new(point.row, point.col - 1))
                    } else {
                        None
                    }
                })
                .collect(),
            width: self.width - 1,
            height: self.height,
        }
    }

    fn min_spells(&self) -> usize {
        let mut cache = HashMap::<Vec<Point>, usize>::new();

        self.min_spells_cached(&mut cache)
    }

    fn min_spells_cached(&self, cache: &mut HashMap<Vec<Point>, usize>) -> usize {
        let mut points: Vec<Point> = self.blobs.iter().copied().collect();
        points.sort();

        if let Some(answer) = cache.get(&points) {
            return *answer;
        }

        let answer = if self.width == 1 || self.height == 1 {
            1
        } else {
            let h = self.shoot_horizontal();
            if h.blobs.is_empty() {
                return 1;
            }
            let v = self.shoot_vertical();
            if v.blobs.is_empty() {
                return 1;
            }

            1 + h.min_spells_cached(cache).min(v.min_spells_cached(cache))
        };

        cache.insert(points, answer);

        answer
    }

    fn new(inblobs: HashSet<Point>) -> Self {
        let min_row = inblobs.iter().map(|point| point.row).min().unwrap();
        let min_col = inblobs.iter().map(|point| point.col).min().unwrap();
        let max_row = inblobs.iter().map(|point| point.row).max().unwrap();
        let max_col = inblobs.iter().map(|point| point.col).max().unwrap();

        let height = max_row - min_row + 1;
        let width = max_col - min_col + 1;
        let mut blobs: HashSet<Point> = inblobs
            .into_iter()
            .map(|point| Point::new(point.row - min_row, point.col - min_col))
            .collect();
        if !blobs.contains(&Point::new(0, 0)) || !blobs.contains(&Point::new(0, width - 1)) {
            blobs = blobs
                .iter()
                .map(|point| Point::new(height - point.row - 1, point.col))
                .collect();
        }

        if !blobs.contains(&Point::new(height - 1, 0)) {
            blobs = blobs
                .iter()
                .map(|point| Point::new(point.row, width - point.col - 1))
                .collect();
        }

        Self {
            blobs,
            height,
            width,
        }
    }

    fn _print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.blobs.contains(&Point::new(row, col)) {
                    print!("#");
                }
            }
            println!();
        }
    }
}

#[derive(Debug)]
struct Stacks {
    blocks: Vec<Block>,
}

impl Stacks {
    fn new<'a, I>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let grid = lines
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut seen = HashSet::<Point>::new();

        let mut blocks = Vec::new();

        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '#' && seen.insert(Point::new(r, c)) {
                    let mut queue = vec![Point::new(r, c)];
                    let mut blobs = HashSet::<Point>::new();

                    while let Some(point) = queue.pop() {
                        seen.insert(point);
                        if blobs.insert(point) {
                            if point.row > 1 && grid[point.row - 1][point.col] == '#' {
                                queue.push(point.up());
                            }
                            if point.row < rows - 1 && grid[point.row + 1][point.col] == '#' {
                                queue.push(point.down());
                            }
                            if point.col > 1 && grid[point.row][point.col - 1] == '#' {
                                queue.push(point.left());
                            }
                            if point.col < cols - 1 && grid[point.row][point.col + 1] == '#' {
                                queue.push(point.right());
                            }
                        }
                    }

                    blocks.push(Block::new(blobs));
                }
            }
        }

        Self { blocks }
    }

    fn min_spells(&self) -> usize {
        self.blocks
            .iter()
            .map(|block| {
                // block._print();
                let dim = block.min_spells();
                // println!(" > {dim}");
                dim
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let data = include_str!("../test-input/part1");
        let stacks = Stacks::new(data.lines());
        assert_eq!(7, stacks.min_spells());
    }

    #[test]
    fn test_part2a() {
        let data = include_str!("../test-input/part2a");
        let stacks = Stacks::new(data.lines());
        assert_eq!(6, stacks.min_spells());
    }

    #[test]
    fn test_part2b() {
        let data = include_str!("../test-input/part2b");
        let stacks = Stacks::new(data.lines());
        assert_eq!(23, stacks.min_spells());
    }
}
