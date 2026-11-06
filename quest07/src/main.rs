use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q07_p1.txt").expect("file");
    let battlefield = Battlefield::new(data.lines());
    println!("Quest 7, part 1 = {}", battlefield.max_conn_total(5));

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q07_p2.txt").expect("file");
    let battlefield = Battlefield::new(data.lines());
    println!("Quest 7, part 2 = {}", battlefield.max_conn_total(10));

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q07_p3.txt").expect("file");
    let battlefield = Battlefield::new(data.lines());
    println!(
        "Quest 7, part 3 = {}",
        battlefield.best_explosive_total(10, 1_000_000)
    );
}

#[derive(Debug)]
struct Battlefield {
    field: Vec<Cloud>,
    max_conn_idx: usize,
}

impl Battlefield {
    fn new<'a, I>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut field: Vec<_> = lines
            .into_iter()
            .map(|line| line.parse::<Cloud>().unwrap())
            .collect();
        field.sort_by_key(|cloud| cloud.id);
        let (max_conn_idx, _) = field
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.connections.len().cmp(&b.1.connections.len()))
            .unwrap();
        Self {
            field,
            max_conn_idx,
        }
    }

    fn max_conn_total(&self, impulse_amt: usize) -> usize {
        self.total_impulse(impulse_amt, self.max_conn_idx, None)
    }

    fn total_impulse(
        &self,
        impulse_amt: usize,
        starting_point: usize,
        explosion: Option<usize>,
    ) -> usize {
        let mut energy_acquired = self
            .field
            .iter()
            .map(|cloud| cloud.energy)
            .collect::<Vec<_>>();

        // id | 0   1    2    3    4    5  ... 10
        // 0  |                         4
        // 1  |                         0  ...  1

        let max_impulse_size = 1 + 10.max(impulse_amt);
        let mut impulses = vec![vec![0usize; max_impulse_size]; self.field.len()];
        let mut exploded = vec![false; self.field.len()];
        impulses[starting_point][impulse_amt] = 1;

        let mut sent_pulse = true;
        while sent_pulse {
            sent_pulse = false;

            let mut next_impulses = vec![vec![0; max_impulse_size]; self.field.len()];
            for cloud_idx in 0..self.field.len() {
                if exploded[cloud_idx] {
                    continue;
                }

                for (impulse_size, impulse_count) in impulses[cloud_idx].iter().enumerate() {
                    if impulse_size > 0 && *impulse_count > 0 {
                        energy_acquired[cloud_idx] += impulse_size * impulse_count;
                        for conn in &self.field[cloud_idx].connections {
                            if !exploded[*conn - 1] {
                                next_impulses[*conn - 1][impulse_size - 1] += impulse_count;
                                sent_pulse = true;
                            }
                        }
                    }
                }
            }

            if let Some(explode) = explosion {
                for idx in 0..energy_acquired.len() {
                    if !exploded[idx] && energy_acquired[idx] >= explode {
                        exploded[idx] = true;
                        for c in &self.field[idx].connections {
                            if !exploded[*c - 1] {
                                next_impulses[*c - 1][10] += 1;
                                sent_pulse = true;
                            }
                        }
                    }
                }
            }

            impulses = next_impulses;
        }

        (0..energy_acquired.len())
            .filter(|idx| !exploded[*idx])
            .map(|idx| energy_acquired[idx])
            .sum()
    }

    fn best_explosive_total(&self, impulse_amt: usize, explosion_limit: usize) -> usize {
        (0..self.field.len())
            .map(|idx| self.total_impulse(impulse_amt, idx, Some(explosion_limit)))
            .max()
            .unwrap()
    }
}

#[derive(Debug, PartialEq)]
struct Cloud {
    id: usize,
    energy: usize,
    connections: Vec<usize>,
}

impl FromStr for Cloud {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut words = line.split(", ");

        let id = words.next().unwrap();
        let energy = words.next().unwrap();
        let connections = words.next().unwrap();

        let (_, id) = id.split_once('=').unwrap();
        let (_, energy) = energy.split_once('=').unwrap();
        let (_, connections) = connections.split_once('=').unwrap();

        Ok(Self {
            id: id.parse().unwrap(),
            energy: energy.parse().unwrap(),
            connections: connections
                .split(',')
                .map(|id| id.parse().unwrap())
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_cloud() {
        let cloud: Cloud = "id=1, energy=5, connections=2,5,7".parse().unwrap();
        assert_eq!(
            Cloud {
                id: 1,
                energy: 5,
                connections: vec![2, 5, 7]
            },
            cloud
        );
    }

    #[test]
    fn test_part1() {
        let part1_data = include_str!("../test-input/part1");
        let battlefield = Battlefield::new(part1_data.lines());
        assert_eq!(102, battlefield.max_conn_total(5));
    }

    #[test]
    fn test_part2() {
        let part2_data = include_str!("../test-input/part2");
        let battlefield = Battlefield::new(part2_data.lines());
        assert_eq!(1316, battlefield.max_conn_total(10));
    }

    #[test]
    fn test_part3() {
        let part3_data = include_str!("../test-input/part3");
        let battlefield = Battlefield::new(part3_data.lines());
        assert_eq!(187, battlefield.best_explosive_total(10, 100));
    }
}
