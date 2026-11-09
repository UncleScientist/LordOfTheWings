use std::{collections::HashMap, ops::Deref};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q10_p1.txt").expect("file");
    let maze = Maze::new(data.lines());
    println!("Quest 10, part 1 = {}", maze.collisions_after(10));

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q10_p2.txt").expect("file");
    let maze = Maze::new(data.lines());
    println!("Quest 10, part 2 = {}", maze.fade_after(10000));
}

#[derive(Debug)]
struct Maze {
    grid: HashMap<(usize, usize), Vec<Tile>>,
    ghost_count: usize,
}

impl Maze {
    fn new<'a, I>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut width = 0;
        let mut grid_vec: Vec<Tile> = Vec::new();

        for line in lines {
            width = width.max(line.len());
            #[allow(clippy::redundant_closure)]
            grid_vec.extend(line.chars().map(|ch| <char as Into<Tile>>::into(ch)));
        }

        let ghost_count = grid_vec
            .iter()
            .filter(|tile| matches!(tile, Tile::Ghost(_)))
            .count();

        Self {
            grid: grid_vec
                .into_iter()
                .enumerate()
                .filter(|(_, tile)| *tile != Tile::Empty)
                .map(|(pos, tile)| ((pos / width, pos % width), vec![tile]))
                .collect(),
            ghost_count,
        }
    }

    fn collisions_after(&self, rounds: usize) -> usize {
        let mut cur_grid = self.grid.clone();
        let mut collisions = 0;

        for _ in 0..rounds {
            let mut next_grid: HashMap<(usize, usize), Vec<Tile>> = HashMap::new();
            for (pos, tiles) in cur_grid.iter() {
                for tile in tiles {
                    match tile {
                        Tile::Empty | Tile::Wall => {
                            next_grid.entry(*pos).or_default().push(*tile);
                        }
                        Tile::Ghost(dir) => {
                            let next_pos = dir.step(*pos);
                            if Some(&vec![Tile::Wall]) == cur_grid.get(&next_pos) {
                                collisions += 1;
                                next_grid
                                    .entry(*pos)
                                    .or_default()
                                    .push(Tile::Ghost(dir.right()));
                            } else {
                                next_grid
                                    .entry(next_pos)
                                    .or_default()
                                    .push(Tile::Ghost(*dir));
                            }
                        }
                    }
                }
            }

            for tiles in next_grid.values_mut() {
                if tiles.len() > 1 {
                    collisions += tiles.len();
                    for t in tiles {
                        t.flip_direction();
                    }
                }
            }

            cur_grid = next_grid;
        }

        collisions
    }

    fn fade_after(&self, strength: usize) -> usize {
        let mut cur_grid = self
            .grid
            .iter()
            .map(|(pos, vec)| (*pos, vec.iter().map(|tile| (*tile, 0)).collect::<Vec<_>>()))
            .collect::<HashMap<(usize, usize), Vec<(Tile, usize)>>>();
        let mut current_count = self.ghost_count;
        let mut rounds = 0;

        while current_count > 0 {
            rounds += 1;
            let mut next_grid: HashMap<(usize, usize), Vec<(Tile, usize)>> = HashMap::new();
            for (pos, tiles) in cur_grid.iter() {
                for (tile, count) in tiles {
                    match tile {
                        Tile::Empty | Tile::Wall => {
                            next_grid.entry(*pos).or_default().push((*tile, 0));
                        }
                        Tile::Ghost(dir) => {
                            let next_pos = dir.step(*pos);
                            if Some(&vec![(Tile::Wall, 0)]) == cur_grid.get(&next_pos) {
                                if count + 1 < strength {
                                    next_grid
                                        .entry(*pos)
                                        .or_default()
                                        .push((Tile::Ghost(dir.right()), count + 1));
                                } else {
                                    current_count -= 1;
                                }
                            } else {
                                next_grid
                                    .entry(next_pos)
                                    .or_default()
                                    .push((Tile::Ghost(*dir), *count));
                            }
                        }
                    }
                }
            }

            for tiles in next_grid.values_mut() {
                if tiles.len() > 1 {
                    for tile in tiles.iter_mut() {
                        tile.0.flip_direction();
                        tile.1 += 1;
                    }
                    let start = tiles.len();
                    tiles.retain(|(_, count)| *count < strength);
                    current_count -= start - tiles.len();
                }
            }

            cur_grid = next_grid;
        }

        rounds
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Ghost(Direction),
}

impl Tile {
    fn flip_direction(&mut self) {
        let Tile::Ghost(dir) = self else {
            panic!("can't flip {self:?}");
        };
        dir.flip();
    }
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '^' => Self::Ghost(Direction::new(NameDirection::Up)),
            '>' => Self::Ghost(Direction::new(NameDirection::Right)),
            'v' => Self::Ghost(Direction::new(NameDirection::Down)),
            '<' => Self::Ghost(Direction::new(NameDirection::Left)),
            _ => panic!("invalid char '{ch}'"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Direction(u8);

impl Direction {
    const DIRS: [NameDirection; 4] = [
        NameDirection::Up,
        NameDirection::Right,
        NameDirection::Down,
        NameDirection::Left,
    ];

    fn new(dir: NameDirection) -> Self {
        match dir {
            NameDirection::Up => Self(0),
            NameDirection::Right => Self(1),
            NameDirection::Down => Self(2),
            NameDirection::Left => Self(3),
        }
    }

    fn right(&self) -> Self {
        Self((self.0 + 1) % 4)
    }

    fn step(&self, (row, col): (usize, usize)) -> (usize, usize) {
        match &Self::DIRS[self.0 as usize] {
            NameDirection::Up => (row - 1, col),
            NameDirection::Right => (row, col + 1),
            NameDirection::Down => (row + 1, col),
            NameDirection::Left => (row, col - 1),
        }
    }

    fn flip(&mut self) {
        self.0 = (self.0 + 2) % 4;
    }
}

impl Deref for Direction {
    type Target = NameDirection;

    fn deref(&self) -> &Self::Target {
        &Self::DIRS[self.0 as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum NameDirection {
    Up,
    Right,
    Down,
    Left,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_direction() {
        let dir = Direction::new(NameDirection::Up);
        assert_eq!(NameDirection::Right, *dir.right());
    }

    #[test]
    fn test_part1() {
        let maze = Maze::new(include_str!("../test-input/part1").lines());

        let collision_count = maze.collisions_after(10);
        assert_eq!(29, collision_count);
    }

    #[test]
    fn test_part2() {
        let maze = Maze::new(include_str!("../test-input/part1").lines());

        let last_survivor = maze.fade_after(10);
        assert_eq!(53, last_survivor);

        let last_survivor = maze.fade_after(10000);
        assert_eq!(50000, last_survivor);
    }
}
