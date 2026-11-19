fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q12_p1.txt").expect("file");
    let stacks = Stacks::new(data.lines());
    println!("Quest 12, part 1 = {}", stacks.min_spells());
}

#[derive(Debug)]
struct Block {
    width: usize,
    height: usize,
}

impl Block {
    fn min_dimension(&self) -> usize {
        self.width.min(self.height)
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
        let mut grid = lines
            .into_iter()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut blocks = Vec::new();

        let rows = grid.len();
        let cols = grid[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '#' {
                    let (start_row, start_col) = (r, c);
                    let (mut cur_row, mut cur_col) = (r, c);

                    let mut max_col = 0;

                    while cur_col < cols && cur_row < rows && grid[cur_row][cur_col] == '#' {
                        while grid[cur_row][cur_col] == '#' {
                            grid[cur_row][cur_col] = '.';
                            cur_col += 1;
                            if cur_col >= cols {
                                break;
                            }
                        }
                        max_col = cur_col;
                        cur_col = start_col;
                        cur_row += 1;
                    }
                    blocks.push(Block {
                        width: max_col - start_col,
                        height: cur_row - start_row,
                    });
                }
            }
        }

        Self { blocks }
    }

    fn min_spells(&self) -> usize {
        self.blocks.iter().map(|block| block.min_dimension()).sum()
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
}
