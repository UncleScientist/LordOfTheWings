fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q05_p1.txt").expect("file");
    let map = Grid::new(data.lines());
    let landmass = map.find_landmass();
    println!("Quest 5, part 1 = {}", Grid::landmass_size(&landmass));
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

    fn find_landmass(&self) -> Vec<Tile> {
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
        mygrid
    }

    fn landmass_size(grid: &[Tile]) -> usize {
        grid.iter()
            .filter(|tile| matches!(tile, Tile::Land))
            .count()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Light,
    Dark,
    Land,
    // Water
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
}
