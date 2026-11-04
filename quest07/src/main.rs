use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = include_str!("../input/everybody_codes_e2026_q07_p1.txt");
    let battlefield = Battlefield::new(data.lines());
    println!("Quest 7, part 1 = {}", battlefield.total_impulse(5));

    let data = include_str!("../input/everybody_codes_e2026_q07_p2.txt");
    let battlefield = Battlefield::new(data.lines());
    println!("Quest 7, part 2 = {}", battlefield.total_impulse(10));
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

    fn total_impulse(&self, mut impulse: usize) -> usize {
        let mut energy_acquired = vec![0; self.field.len()];

        let mut send_impulse = Vec::from([self.max_conn_idx]);
        energy_acquired[self.max_conn_idx] = impulse;
        impulse -= 1;

        while impulse > 0 {
            let mut next_impulse = Vec::new();
            for idx in &send_impulse {
                for c in &self.field[*idx].connections {
                    energy_acquired[*c - 1] += impulse;
                    next_impulse.push(*c - 1);
                }
            }
            impulse -= 1;
            send_impulse = next_impulse;
        }

        (0..energy_acquired.len())
            .map(|idx| energy_acquired[idx] + self.field[idx].energy)
            .sum()
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
        assert_eq!(102, battlefield.total_impulse(5));
    }

    #[test]
    fn test_part2() {
        let part2_data = include_str!("../test-input/part2");
        let battlefield = Battlefield::new(part2_data.lines());
        assert_eq!(1316, battlefield.total_impulse(10));
    }
}
