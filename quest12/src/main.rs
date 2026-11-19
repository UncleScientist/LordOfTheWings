use std::collections::{HashMap, HashSet};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q12_p1.txt").expect("file");
    let stacks = Stacks::new(data.lines());
    println!("Quest 12, part 1 = {}", stacks.min_spells());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q12_p2.txt").expect("file");
    let stacks = Stacks::new(data.lines());
    println!("Quest 12, part 2 = {}", stacks.min_spells());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q12_p3.txt").expect("file");
    let stacks = Stacks::new(data.lines());
    println!("Quest 12, part 3 = {}", stacks.min_spells());
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
    fn shoot_horizontally(&self) -> Self {
        let mut c = 0;

        while c < self.width && !self.blobs.contains(&Point::new(0, c)) {
            c += 1;
        }
        assert!(c < self.width);

        let mut blobs = self.blobs.clone();
        while c < self.width && self.blobs.contains(&Point::new(0, c)) {
            blobs.remove(&Point::new(0, c));
            c += 1;
        }

        let mut more = false;
        while c < self.width {
            if self.blobs.contains(&Point::new(0, c)) {
                more = true;
                break;
            }
            c += 1;
        }

        let (blobs, new_width, new_height) = if more {
            (blobs, self.width, self.height)
        } else {
            let mut new_width = self.width - 1;
            for r in 0..self.height {
                if blobs.contains(&Point::new(r, new_width)) {
                    new_width = self.width;
                    break;
                }
            }
            (
                self.blobs
                    .iter()
                    .filter_map(|point| {
                        if point.row > 0 {
                            Some(Point::new(point.row - 1, point.col))
                        } else {
                            None
                        }
                    })
                    .collect(),
                new_width,
                self.height - 1,
            )
        };
        let blobs = Self::normalize(blobs, new_width, new_height, 0, 0);
        let result = if more {
            Self { blobs, ..*self }
        } else {
            Self {
                blobs,
                width: new_width,
                height: new_height,
            }
        };
        result
    }

    fn shoot_vertically(&self) -> Self {
        let mut r = 0;

        while r < self.height && !self.blobs.contains(&Point::new(r, 0)) {
            r += 1;
        }
        assert!(r < self.height);

        let mut blobs = self.blobs.clone();
        while r < self.height && self.blobs.contains(&Point::new(r, 0)) {
            blobs.remove(&Point::new(r, 0));
            r += 1;
        }

        let mut more = false;
        while r < self.height {
            if self.blobs.contains(&Point::new(r, 0)) {
                more = true;
                break;
            }
            r += 1;
        }

        let (blobs, new_width, new_height) = if more {
            (blobs, self.width, self.height)
        } else {
            let mut new_height = self.height - 1;
            for c in 0..self.width {
                if blobs.contains(&Point::new(new_height, c)) {
                    new_height = self.height;
                    break;
                }
            }
            (
                self.blobs
                    .iter()
                    .filter_map(|point| {
                        if point.col > 0 {
                            Some(Point::new(point.row, point.col - 1))
                        } else {
                            None
                        }
                    })
                    .collect(),
                self.width - 1,
                new_height,
            )
        };

        let blobs = Self::normalize(blobs, new_width, new_height, 0, 0);

        let result = if more {
            Self { blobs, ..*self }
        } else {
            Self {
                blobs,
                width: new_width,
                height: new_height,
            }
        };
        result
    }

    fn min_spells(&self) -> usize {
        let mut cache = HashMap::<Vec<Point>, usize>::new();

        let result = self.min_spells_cached(&mut cache);

        result
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
            let h = self.shoot_horizontally();
            if h.blobs.is_empty() {
                return 1;
            }
            let v = self.shoot_vertically();
            if v.blobs.is_empty() {
                return 1;
            }

            1 + h.min_spells_cached(cache).min(v.min_spells_cached(cache))
        };

        cache.insert(points, answer);

        answer
    }

    fn normalize(
        inblobs: HashSet<Point>,
        width: usize,
        height: usize,
        min_row: usize,
        min_col: usize,
    ) -> HashSet<Point> {
        if width == 0 || height == 0 {
            return inblobs;
        }

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

        blobs
    }

    fn new(inblobs: HashSet<Point>) -> Self {
        let min_row = inblobs.iter().map(|point| point.row).min().unwrap();
        let min_col = inblobs.iter().map(|point| point.col).min().unwrap();
        let max_row = inblobs.iter().map(|point| point.row).max().unwrap();
        let max_col = inblobs.iter().map(|point| point.col).max().unwrap();

        let height = max_row - min_row + 1;
        let width = max_col - min_col + 1;
        let blobs = Self::normalize(inblobs, width, height, min_row, min_col);

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
                } else {
                    print!(".");
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
        self.blocks.iter().map(|block| block.min_spells()).sum()
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

    #[test]
    fn test_part3a() {
        let data = include_str!("../test-input/part3a");
        let stacks = Stacks::new(data.lines());
        assert_eq!(8, stacks.min_spells());
    }

    #[test]
    fn test_part3b() {
        let data = include_str!("../test-input/part3b");
        let stacks = Stacks::new(data.lines());
        assert_eq!(29, stacks.min_spells());
    }

    #[test]
    fn test_part3c() {
        let data = include_str!("../test-input/part3c");
        let stacks = Stacks::new(data.lines());
        assert_eq!(16, stacks.min_spells());
    }

    #[test]
    fn test_horizontal_shot() {
        let block = Block {
            blobs: vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2)]
                .into_iter()
                .collect(),
            height: 1,
            width: 3,
        };
        let shot = block.shoot_horizontally();
        assert_eq!(0, shot.height);
    }

    #[test]
    fn test_horizontal_shot_with_gap() {
        let block = Block {
            blobs: vec![
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 5),
                Point::new(0, 6),
                Point::new(0, 7),
            ]
            .into_iter()
            .collect(),
            height: 1,
            width: 8,
        };
        let shot = block.shoot_horizontally();
        let remaining: HashSet<Point> = vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2)]
            .into_iter()
            .collect();
        assert_eq!(1, shot.height);
        assert_eq!(remaining, shot.blobs);

        let final_shot = shot.shoot_horizontally();
        assert_eq!(0, final_shot.height);
        assert!(final_shot.blobs.is_empty());
    }

    #[test]
    fn test_hole() {
        let lines = r#"##########
##########
####..####
##########
##########"#;
        let stacks = Stacks::new(lines.lines());
        assert_eq!(6, stacks.min_spells());
    }
}
