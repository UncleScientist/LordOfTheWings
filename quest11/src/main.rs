use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q11_p1.txt").expect("file");
    let portal = Portal::new(data.lines());
    println!("Quest 11, part 1 = {}", portal.portal_resonance());
}

#[derive(Debug, Default)]
struct Portal {
    portals: usize,
    connections: Vec<(usize, usize)>,
}

impl Portal {
    fn new<'a, I>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut iter = lines.into_iter();
        let line = iter.next().unwrap();
        let (_, portals) = line.split_once(' ').unwrap();

        let portals = portals.parse::<usize>().unwrap();
        let mut connections = Vec::new();

        for line in iter {
            if !line.is_empty() {
                let (start, end) = line.split_once(' ').unwrap();
                let start = start.parse::<usize>().unwrap() - 1;
                let end = end.parse::<usize>().unwrap() - 1;
                connections.push((start, end));
                connections.push((end, start));
            }
        }

        Self {
            portals,
            connections,
        }
    }

    fn portal_strain(&self) -> usize {
        self.portals
            * if self.portals.is_multiple_of(2) {
                // we have one directly opposite the first
                let max_dist = self.portals / 2;
                let n = max_dist - 1;
                max_dist + n * (n + 1)
            } else {
                // no direct opposite
                let n = self.portals / 2;
                n * (n + 1)
            }
    }

    #[cfg(test)]
    fn dijkstra_strain(&self) -> usize {
        let mut total = 0;
        for start in 0..self.portals {
            let mut subtotal = 0;
            for end in 0..self.portals {
                if start == end {
                    continue;
                }
                let dist = dijkstra(
                    &start,
                    |pos: &usize| {
                        let left = (pos + self.portals - 1) % self.portals;
                        let right = (pos + 1) % self.portals;
                        vec![(left, 1), (right, 1)]
                    },
                    |pos: &usize| *pos == end,
                )
                .unwrap()
                .1;
                // println!("{} -> {} = {dist}", start + 1, end + 1);
                subtotal += dist;
            }
            // println!("Starting at {}: {subtotal}", start + 1);
            total += subtotal;
        }

        total
    }

    fn portal_strain_with_shortcuts(&self) -> usize {
        let mut total = 0;
        for start in 0..self.portals {
            let mut subtotal = 0;
            for end in 0..self.portals {
                if start == end {
                    continue;
                }
                let dist = dijkstra(
                    &start,
                    |pos: &usize| {
                        let left = (pos + self.portals - 1) % self.portals;
                        let right = (pos + 1) % self.portals;

                        let mut response = vec![(left, 1), (right, 1)];
                        for c in &self.connections {
                            if c.0 == *pos {
                                response.push((c.1, 1));
                            }
                        }
                        response
                    },
                    |pos: &usize| *pos == end,
                )
                .unwrap()
                .1;
                // println!("{} -> {} = {dist}", start + 1, end + 1);
                subtotal += dist;
            }
            // println!("Starting at {}: {subtotal}", start + 1);
            total += subtotal;
        }

        total
    }

    fn portal_resonance(&self) -> usize {
        let s1 = self.portal_strain();
        let s2 = self.portal_strain_with_shortcuts();
        println!("{s1} - {s2} = {}", s1 - s2);
        s1 - s2
    }
}

pub fn dijkstra<T, S>(
    start: &T,
    neighbors: impl Fn(&T) -> Vec<(T, S)>,
    is_end: impl Fn(&T) -> bool,
) -> Option<(T, S)>
where
    T: Clone + Hash + Eq,
    S: Copy + From<u8> + Ord + Add<Output = S>,
{
    let zero: S = 0u8.into();
    let mut queue = BTreeMap::from([(zero, HashSet::from([start.clone()]))]);
    let mut visited = HashSet::new();
    let mut dist = HashMap::new();

    dist.insert(start.clone(), Some(zero));

    while let Some((time, pos_list)) = queue.pop_first() {
        for pos in pos_list.iter() {
            if is_end(pos) {
                return Some((pos.clone(), time));
            }
            if visited.insert(pos.clone()) {
                for (new_pos, cost) in neighbors(pos) {
                    let new_time = time + cost;
                    let dist_entry = dist.entry(new_pos.clone()).or_insert(None);
                    if let Some(dist_time) = dist_entry {
                        if new_time >= *dist_time {
                            continue;
                        }

                        if let Some(qentry) = queue.get_mut(dist_time) {
                            qentry.remove(pos);
                        }
                    }
                    *dist_entry = Some(new_time);
                    queue.entry(new_time).or_default().insert(new_pos);
                }
            }
        }
    }

    None
}

/*
pub fn bfs<T, S>(
    start: &T,
    neighbors: impl Fn(&T) -> Vec<T>,
    is_end: impl Fn(&T) -> bool,
) -> Option<S>
where
    T: Clone + Hash + Eq,
    S: Copy + From<u8> + Ord + Add<Output = S>,
{
    let zero: S = 0u8.into();
    let one: S = 1u8.into();
    let mut queue = VecDeque::from([(zero, start.clone())]);
    let mut visited = HashSet::new();

    while let Some((dist, entry)) = queue.pop_front() {
        if is_end(&entry) {
            return Some(dist);
        }
        if visited.insert(entry.clone()) {
            for neighbor in neighbors(&entry) {
                queue.push_back((dist + one, neighbor));
            }
        }
    }

    None
}
*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = include_str!("../test-input/part1");
        let portal = Portal::new(test_input.lines());

        assert_eq!(60, portal.portal_resonance());
    }

    #[test]
    fn test_odd_portal_count() {
        let portal = Portal {
            portals: 11,
            connections: Vec::new(),
        };
        assert_eq!(330, portal.portal_strain());
    }

    #[test]
    fn test_31_portal_count() {
        let portal = Portal {
            portals: 31,
            connections: Vec::new(),
        };
        assert_eq!(7440, portal.portal_strain());
        assert_eq!(7440, portal.dijkstra_strain());
    }
}
