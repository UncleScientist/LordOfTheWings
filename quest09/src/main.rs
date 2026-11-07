use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q09_p1.txt").expect("file");
    println!(
        "Quest 9, part 1 = {}",
        data.lines()
            .map(|line| line.parse::<Plant>().unwrap())
            .map(|plant| plant.time_limit())
            .sum::<usize>()
    );

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q09_p2.txt").expect("file");
    println!(
        "Quest 9, part 2 = {}",
        data.lines()
            .map(|line| line.parse::<Plant>().unwrap())
            .map(|plant| plant.leaf_limit())
            .sum::<usize>()
    );
}

#[derive(Debug, PartialEq)]
struct Plant {
    _id: usize,
    growth: usize,
    limit: usize,
}

impl Plant {
    fn time_limit(&self) -> usize {
        let mut branches = vec![1usize];
        let mut splits = 0;
        let mut time = 1;

        while splits + branches.iter().sum::<usize>() < self.limit {
            time += 1;
            let mut next = Vec::new();
            for b in &branches {
                let next_len = *b + 1;
                if next_len == self.growth {
                    next.push(next_len / 2);
                    next.push(next_len / 2);
                    next.push(1);
                    splits += 1;
                } else {
                    next.push(next_len);
                }
            }

            branches = next;
        }

        time
    }

    fn leaf_limit(&self) -> usize {
        let mut branches = vec![1usize];
        let mut splits = 0;
        let mut time = 1;

        while branches.len() - splits < self.limit {
            time += 1;
            let mut next = Vec::new();
            for b in &branches {
                let next_len = *b + 1;
                if next_len == self.growth {
                    next.push(next_len / 2);
                    next.push(next_len / 2);
                    next.push(1);
                    splits += 1;
                } else {
                    next.push(next_len);
                }
            }

            branches = next;
        }

        time
    }
}

impl FromStr for Plant {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut words = line.split(", ");

        let id = words.next().unwrap();
        let growth = words.next().unwrap();
        let limit = words.next().unwrap();

        let _id = id.split_once('=').unwrap().1.parse::<usize>().unwrap();
        let growth = growth.split_once('=').unwrap().1.parse::<usize>().unwrap();
        let limit = limit.split_once('=').unwrap().1.parse::<usize>().unwrap();

        Ok(Self { _id, growth, limit })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_plant() {
        let plant: Plant = "id=1, growth=3, limit=10".parse().unwrap();
        assert_eq!(
            Plant {
                _id: 1,
                growth: 3,
                limit: 10
            },
            plant
        );
    }

    #[test]
    fn test_part1() {
        let data = include_str!("../test-input/part1");
        let plant_time = data
            .lines()
            .map(|line| line.parse::<Plant>().unwrap())
            .map(|plant| plant.time_limit())
            .sum::<usize>();
        assert_eq!(27, plant_time);
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../test-input/part2");
        let plant_time = data
            .lines()
            .map(|line| line.parse::<Plant>().unwrap())
            .map(|plant| plant.leaf_limit())
            .sum::<usize>();
        assert_eq!(42, plant_time);
    }
}
