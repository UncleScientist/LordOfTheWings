fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q05_p1.txt").expect("file");
    let map = Grid::new(data.lines());
    let landmass = map.find_landmass();
    println!("Quest 5, part 1 = {}", landmass.landmass_size());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q05_p2.txt").expect("file");
    let map = Grid::new(data.lines());
    let landmass = map.find_landmass();
    let beach = landmass.find_beach();
    println!("Quest 5, part 2 = {}", beach.beach_size());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q05_p3.txt").expect("file");
    let map = Grid::new(data.lines());
    let landmass = map.find_landmass();
    let beach = landmass.find_beach();
    let trees = beach.find_trees();
    println!("Quest 5, part 3 = {}", trees.tree_size());
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Tile>,
}

impl Grid {
    fn new<'a, I>(map: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut width = 0;
        let mut grid = Vec::new();

        for line in map {
            width = line.len();
            for ch in line.chars() {
                match ch {
                    '0' => grid.push(Tile::Light),
                    '1' => grid.push(Tile::Dark),
                    _ => panic!("invalid char {ch}"),
                }
            }
        }

        Grid {
            width,
            height: grid.len() / width,
            grid,
        }
    }

    fn find_landmass(&self) -> Self {
        let mut mygrid = self.grid.clone();

        let start = ((self.height / 2) * self.width + self.width / 2) as isize;
        let width = self.width as isize;
        mygrid[start as usize] = Tile::Land;

        let mut grid_changed = true;
        while grid_changed {
            grid_changed = false;
            let mut nextgrid = mygrid.clone();
            for row in 0..self.height {
                for col in 0..self.width {
                    let pos = ((row * self.width) + col) as isize;
                    let neighbors = [
                        pos - width - 1,
                        pos - width,
                        pos - width + 1,
                        pos - 1,
                        pos + 1,
                        pos + width - 1,
                        pos + width,
                        pos + width + 1,
                    ];
                    let mut count = 0;
                    let mut found_land = false;

                    for loc in neighbors {
                        if loc < 0 || loc >= mygrid.len() as isize {
                            continue;
                        }

                        if matches!(mygrid[loc as usize], Tile::Land) {
                            found_land = true;
                            count += 1;
                        } else if self.grid[loc as usize].is_land() {
                            count += 1;
                        }
                    }

                    if found_land && count >= 5 && nextgrid[pos as usize] != Tile::Land {
                        nextgrid[pos as usize] = Tile::Land;
                        grid_changed = true;
                    }
                }
            }
            mygrid = nextgrid;
        }

        mygrid.iter_mut().for_each(|item| {
            if !matches!(item, Tile::Land) {
                *item = Tile::Water
            }
        });

        Self {
            width: self.width,
            height: self.height,
            grid: mygrid,
        }
    }

    fn landmass_size(&self) -> usize {
        self.grid
            .iter()
            .filter(|tile| matches!(tile, Tile::Land))
            .count()
    }

    fn find_beach(&self) -> Self {
        let mut mygrid = self.grid.clone();

        let width = self.width as isize;

        for row in 0..self.height {
            for col in 0..self.width {
                let pos = ((row * self.width) + col) as isize;
                if mygrid[pos as usize] != Tile::Land {
                    continue;
                }

                let neighbors = [
                    pos - width - 1,
                    pos - width,
                    pos - width + 1,
                    pos - 1,
                    pos + 1,
                    pos + width - 1,
                    pos + width,
                    pos + width + 1,
                ];
                for loc in neighbors {
                    if loc < 0
                        || loc >= mygrid.len() as isize
                        || mygrid[loc as usize] == Tile::Water
                    {
                        mygrid[pos as usize] = Tile::Sand;
                        break;
                    }
                }
            }
        }

        let mut grid_changed = true;
        while grid_changed {
            grid_changed = false;
            let mut nextgrid = mygrid.clone();
            for row in 0..self.height {
                for col in 0..self.width {
                    let pos = ((row * self.width) + col) as isize;
                    let neighbors = [
                        pos - width - 1,
                        pos - width,
                        pos - width + 1,
                        pos - 1,
                        pos + 1,
                        pos + width - 1,
                        pos + width,
                        pos + width + 1,
                    ];
                    let mut count = 0;

                    for loc in neighbors {
                        if loc < 0 || loc >= mygrid.len() as isize {
                            continue;
                        }

                        if matches!(mygrid[loc as usize], Tile::Sand) {
                            count += 1;
                        }
                    }

                    if count >= 5 && nextgrid[pos as usize] != Tile::Sand {
                        nextgrid[pos as usize] = Tile::Sand;
                        grid_changed = true;
                    }
                }
            }
            mygrid = nextgrid;
        }

        Self {
            width: self.width,
            height: self.height,
            grid: mygrid,
        }
    }

    fn beach_size(&self) -> usize {
        self.grid.iter().filter(|tile| **tile == Tile::Sand).count()
    }

    fn find_trees(&self) -> Self {
        let mut mygrid = self.grid.clone();

        let width = self.width as isize;

        for row in 0..self.height {
            for col in 0..self.width {
                let pos = ((row * self.width) + col) as isize;
                if mygrid[pos as usize] != Tile::Land {
                    continue;
                }

                let neighbors = [
                    // outer twelve
                    pos - 2 * width - 1,
                    pos - 2 * width,
                    pos - 2 * width + 1,
                    pos - width - 2,
                    pos - 2,
                    pos + width - 2,
                    pos - width + 2,
                    pos + 2,
                    pos + width + 2,
                    pos + 2 * width - 1,
                    pos + 2 * width,
                    pos + 2 * width + 1,
                    // inner eight
                    pos - width - 1,
                    pos - width,
                    pos - width + 1,
                    pos - 1,
                    pos + 1,
                    pos + width - 1,
                    pos + width,
                    pos + width + 1,
                ];

                let mut count = 0;
                for loc in neighbors {
                    if mygrid[loc as usize] != Tile::Land && mygrid[loc as usize] != Tile::Tree {
                        break;
                    }
                    count += 1;
                }
                if count == neighbors.len() {
                    mygrid[pos as usize] = Tile::Tree;
                }
            }
        }

        for _ in 0..2 {
            let mut nextgrid = mygrid.clone();
            for row in 0..self.height {
                for col in 0..self.width {
                    let pos = ((row * self.width) + col) as isize;
                    let neighbors = [
                        pos - width - 1,
                        pos - width,
                        pos - width + 1,
                        pos - 1,
                        pos + 1,
                        pos + width - 1,
                        pos + width,
                        pos + width + 1,
                    ];
                    let mut count = 0;

                    for loc in neighbors {
                        if loc < 0 || loc >= mygrid.len() as isize {
                            continue;
                        }
                        if mygrid[loc as usize] == Tile::Sand {
                            count = 0;
                            break;
                        }
                        if mygrid[loc as usize] == Tile::Tree {
                            count += 1;
                        }
                    }

                    if count >= 4 {
                        nextgrid[pos as usize] = Tile::Tree;
                    }
                }
            }
            mygrid = nextgrid;
        }

        Self {
            width: self.width,
            height: self.height,
            grid: mygrid,
        }
    }

    fn tree_size(&self) -> usize {
        self.grid.iter().filter(|tile| **tile == Tile::Tree).count()
    }

    fn _print_grid(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!(
                    "{}",
                    match self.grid[row * self.width + col] {
                        Tile::Light => '0',
                        Tile::Dark => '1',
                        Tile::Land => '#',
                        Tile::Water => '.',
                        Tile::Sand => '=',
                        Tile::Tree => '@',
                    }
                );
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Light,
    Dark,
    Land,
    Water,
    Sand,
    Tree,
}

impl Tile {
    fn is_land(&self) -> bool {
        matches!(self, Tile::Dark | Tile::Land)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let data = std::fs::read_to_string("test-input/part1").expect("file");
        let map = Grid::new(data.lines());
        let landmass = map.find_landmass();
        assert_eq!(137, Grid::landmass_size(&landmass));
    }

    #[test]
    fn test_part2() {
        let data = std::fs::read_to_string("test-input/part2").expect("file");
        let map = Grid::new(data.lines());
        let landmass = map.find_landmass();
        let beach = landmass.find_beach();
        assert_eq!(69, beach.beach_size());
    }

    #[test]
    fn test_part3() {
        let data = std::fs::read_to_string("test-input/part3").expect("file");
        let map = Grid::new(data.lines());
        let landmass = map.find_landmass();
        let beach = landmass.find_beach();
        let trees = beach.find_trees();
        assert_eq!(14, trees.tree_size());
    }
}
