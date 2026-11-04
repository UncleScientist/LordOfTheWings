use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = include_str!("../input/everybody_codes_e2026_q07_p1.txt");
    let battlefield = Battlefield::new(data.lines());
    println!("Quest 7, part 1 = {}", battlefield.max_conn_total(5));

    let data = include_str!("../input/everybody_codes_e2026_q07_p2.txt");
    let battlefield = Battlefield::new(data.lines());
    println!("Quest 7, part 2 = {}", battlefield.max_conn_total(10));
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
        mut impulse_amt: usize,
        starting_point: usize,
        explosion: Option<usize>,
    ) -> usize {
        println!("Using cloud {} as starting point", starting_point + 1);

        let mut energy_acquired = self
            .field
            .iter()
            .map(|cloud| cloud.energy)
            .collect::<Vec<_>>();
        let mut impulses = vec![0usize; self.field.len()];
        let mut exploded = vec![false; self.field.len()];
        impulses[starting_point] = 1;

        energy_acquired[starting_point] += impulse_amt;
        impulse_amt -= 1;

        while impulse_amt > 0 {
            let mut next_impulses = vec![0; self.field.len()];
            for (idx, count) in impulses.iter().enumerate().filter(|(_, c)| **c != 0) {
                if exploded[idx] {
                    continue;
                }
                for c in &self.field[idx].connections {
                    if exploded[*c - 1] {
                        continue;
                    }
                    energy_acquired[*c - 1] += impulse_amt * *count;
                    next_impulses[*c - 1] += 1 * *count;
                }
            }

            println!(" {energy_acquired:?}");
            if let Some(explode) = explosion {
                let mut changed = true;
                while changed {
                    changed = false;
                    for idx in 0..energy_acquired.len() {
                        if !exploded[idx] && energy_acquired[idx] >= explode {
                            println!("  cloud {} exploded", self.field[idx].id);
                            exploded[idx] = true;
                            for c in &self.field[idx].connections {
                                if !exploded[*c - 1] {
                                    energy_acquired[*c - 1] += 10;
                                    next_impulses[*c - 1] += 1;
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }

            impulse_amt -= 1;
            impulses = next_impulses;
        }

        let energy = (0..energy_acquired.len())
            .filter(|idx| !exploded[*idx])
            .map(|idx| energy_acquired[idx])
            .sum();
        println!("Total energy: {energy}");
        energy
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
