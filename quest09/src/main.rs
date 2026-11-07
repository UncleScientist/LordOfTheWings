use std::{collections::HashMap, convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q09_p1.txt").expect("file");
    println!(
        "Quest 9, part 1 = {}",
        data.lines()
            .map(|line| line.parse::<Plant>().unwrap())
            .map(|plant| plant.time_limit())
            .sum::<u128>()
    );

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q09_p2.txt").expect("file");
    println!(
        "Quest 9, part 2 = {}",
        data.lines()
            .map(|line| line.parse::<Plant>().unwrap())
            .map(|plant| plant.leaf_limit())
            .sum::<u128>()
    );

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q09_p3.txt").expect("file");
    println!(
        "Quest 9, part 2 = {}",
        data.lines()
            .map(|line| line.parse::<Plant>().unwrap())
            .map(|plant| plant.cached_leaf_limit())
            .sum::<u128>()
    );
}

#[derive(Debug, PartialEq)]
struct Plant {
    _id: usize,
    growth: u128,
    limit: u128,
}

impl Plant {
    fn time_limit(&self) -> u128 {
        let mut branches = vec![1u128];
        let mut splits = 0;
        let mut time = 1;

        while splits + branches.iter().sum::<u128>() < self.limit {
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

    fn leaf_limit(&self) -> u128 {
        let mut branches = vec![1u128];
        let mut splits = 0;
        let mut time = 1;

        while branches.len() as u128 - splits < self.limit {
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

    fn cached_leaf_limit(&self) -> u128 {
        let mut cache = Cache::new(self.growth);

        let mut time = self.growth;
        loop {
            let limit = cache.search(1, time) + 1;
            if limit < self.limit {
                time *= 2;
            } else {
                break;
            }
        }

        let mut low = time / 2;
        let mut high = time;
        while low != high {
            let mid = (high + low) / 2;
            let value = cache.search(1, mid) + 1;
            if value >= self.limit {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low
    }
}

struct Cache {
    cache: HashMap<(u128, u128), u128>,
    growth: u128,
}

impl Cache {
    fn new(growth: u128) -> Self {
        Self {
            cache: HashMap::new(),
            growth,
        }
    }

    fn search(&mut self, branch_len: u128, time: u128) -> u128 {
        if let Some(answer) = self.cache.get(&(branch_len, time)) {
            return *answer;
        }
        let answer = if time == 0 {
            0
        } else {
            let remaining = self.growth - branch_len;
            if remaining >= time {
                0
            } else {
                let delta = time - remaining;
                1 + 2 * self.search(self.growth / 2, delta) + self.search(1, delta)
            }
        };

        self.cache.insert((branch_len, time), answer);
        answer
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
        let growth = growth.split_once('=').unwrap().1.parse::<u128>().unwrap();
        let limit = limit.split_once('=').unwrap().1.parse::<u128>().unwrap();

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
            .sum::<u128>();
        assert_eq!(27, plant_time);
    }

    #[test]
    fn test_part2() {
        let data = include_str!("../test-input/part2");
        let plant_time = data
            .lines()
            .map(|line| line.parse::<Plant>().unwrap())
            .map(|plant| plant.leaf_limit())
            .sum::<u128>();
        assert_eq!(42, plant_time);
    }

    #[test]
    fn test_part3() {
        let data = include_str!("../test-input/part2"); // re-use part2 input
        let plant_time = data
            .lines()
            .map(|line| line.parse::<Plant>().unwrap())
            .map(|plant| plant.cached_leaf_limit())
            .sum::<u128>();
        assert_eq!(42, plant_time);
    }
}
