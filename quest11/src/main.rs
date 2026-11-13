use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q11_p1.txt").expect("file");
    let portal = Portal::new(data.lines());
    println!("Quest 11, part 1 = {}", portal.portal_resonance());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q11_p2.txt").expect("file");
    let portal = Portal::new(data.lines());
    // println!("{portal:?}");
    println!("Quest 11, part 2 = {}", portal.advanced_portal_resonance());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q11_p3.txt").expect("file");
    let portal = Portal::new(data.lines());
    // println!("{portal:?}");
    println!("Quest 11, part 3 = {}", portal.spaced_portal_resonance());
}

#[derive(Debug, Default, Clone)]
struct Portal {
    portals: usize,
    connections: HashSet<(usize, usize)>,
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
        let mut connections = HashSet::new();

        for line in iter {
            if !line.is_empty() {
                let (start, end) = line.split_once(' ').unwrap();
                let start = start.parse::<usize>().unwrap() - 1;
                let end = end.parse::<usize>().unwrap() - 1;
                connections.insert((start, end));
                connections.insert((end, start));
            }
        }

        Self {
            portals,
            connections,
        }
    }

    fn simple_portal_strain(&self) -> usize {
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

    fn portal_strain(
        &self,
        advanced: bool,
        use_connections: bool,
        extra: &Option<(usize, usize)>,
    ) -> usize {
        let mut total = 0;
        // println!(" -- {extra:?} --");
        for start in 0..self.portals {
            let mut subtotal = 0;
            for end in 0..self.portals {
                if start == end {
                    continue;
                }
                let dist = dijkstra(
                    &start,
                    |pos: &usize| {
                        let mut response = if advanced {
                            let r1 = (pos + 1) % self.portals;
                            let r2 = (pos + 2) % self.portals;
                            let r3 = (pos + 3) % self.portals;
                            let l1 = (pos + self.portals - 1) % self.portals;
                            let l2 = (pos + self.portals - 2) % self.portals;
                            let l3 = (pos + self.portals - 3) % self.portals;
                            vec![(r1, 1), (r2, 1), (r3, 1), (l1, 1), (l2, 1), (l3, 1)]
                        } else {
                            let left = (pos + self.portals - 1) % self.portals;
                            let right = (pos + 1) % self.portals;
                            vec![(left, 1), (right, 1)]
                        };
                        if use_connections {
                            for c in &self.connections {
                                if c.0 == *pos {
                                    response.push((c.1, 1));
                                }
                            }
                        }
                        if let Some((first, second)) = extra {
                            if *first == *pos {
                                // println!("extra: {pos} -> {second}");
                                response.push((*second, 1));
                            } else if *second == *pos {
                                // println!("extra: {pos} -> {first}");
                                response.push((*first, 1));
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
        let s1 = self.simple_portal_strain();
        let s2 = self.portal_strain(false, true, &None);
        // println!("{s1} - {s2} = {}", s1 - s2);
        s1 - s2
    }

    fn advanced_portal_resonance(&self) -> usize {
        let s1 = self.portal_strain(true, false, &None);

        let mut max_resonance = 0;
        let mut extras = self.connections.clone();
        for pos in 0..self.portals {
            for pair in (pos + 4)..=(pos + self.portals - 4) {
                let pair = pair % self.portals;
                if !extras.contains(&(pos, pair)) {
                    extras.insert((pos, pair));
                    extras.insert((pair, pos));
                    // print!("adding extra pair ({:2}, {:2})", pos + 1, pair + 1);
                    let s2 = self.portal_strain(true, true, &Some((pos, pair)));
                    let resonance = s1 - s2;
                    // println!(" -> {resonance}");
                    max_resonance = max_resonance.max(resonance);
                }
            }
        }

        max_resonance
    }

    fn spaced_portal_resonance(&self) -> usize {
        let s1 = self.portal_strain(true, false, &None);
        // println!( "{} normal = {s1}, connections = {:?}", self.portals, self.connections);

        let step = self.portals / 8;

        let mut overall_min_strain = usize::MAX;

        for starting_portal in 0..2 {
            let mut extras = self.connections.clone();
            let mut portals = (starting_portal..(starting_portal + self.portals))
                .step_by(step)
                .map(|portal| portal % self.portals)
                .collect::<Vec<_>>();
            println!("{portals:?}");

            let mut min_strain = usize::MAX;
            let mut pair = (0, 0);
            for (a, b) in portals.pairs() {
                if !extras.contains(&(a, b)) {
                    let strain = self.portal_strain(true, true, &Some((a, b)));
                    println!("   ({a},{b}) -> {strain}");
                    if strain < min_strain {
                        min_strain = strain;
                        pair = (a, b);
                    }
                }
            }

            let mut next_portal = self.clone();
            portals.retain(|&x| x != pair.0 && x != pair.1);
            println!("inserted {pair:?} @ {min_strain} | {portals:?}");

            next_portal.connections.insert((pair.0, pair.1));
            next_portal.connections.insert((pair.1, pair.0));

            extras.insert((pair.0, pair.1));
            extras.insert((pair.1, pair.0));

            let mut min_strain = usize::MAX;
            let mut pair = (0, 0);
            for (a, b) in portals.pairs() {
                if !extras.contains(&(a, b)) {
                    let strain = next_portal.portal_strain(true, true, &Some((a, b)));
                    println!("   ({a},{b}) -> {strain}");
                    if strain < min_strain {
                        min_strain = strain;
                        pair = (a, b);
                    }
                }
            }

            let mut penultimate_portal = next_portal.clone();

            portals.retain(|&x| x != pair.0 && x != pair.1);
            println!("inserted {pair:?} @ {min_strain} | {portals:?}");

            penultimate_portal.connections.insert((pair.0, pair.1));
            penultimate_portal.connections.insert((pair.1, pair.0));

            extras.insert((pair.0, pair.1));
            extras.insert((pair.1, pair.0));

            let mut min_strain = usize::MAX;
            let mut pair = (0, 0);
            for (a, b) in portals.pairs() {
                if !extras.contains(&(a, b)) {
                    let strain = penultimate_portal.portal_strain(true, true, &Some((a, b)));
                    println!("   ({a},{b}) -> {strain}");
                    if strain < min_strain {
                        min_strain = strain;
                        pair = (a, b);
                    }
                }
            }

            let mut final_portal = penultimate_portal.clone();
            portals.retain(|&x| x != pair.0 && x != pair.1);
            println!("inserted {pair:?} @ {min_strain} | {portals:?}");

            final_portal.connections.insert((pair.0, pair.1));
            final_portal.connections.insert((pair.1, pair.0));

            extras.insert((pair.0, pair.1));
            extras.insert((pair.1, pair.0));

            let mut min_strain = usize::MAX;
            let mut pair = (0, 0);
            for (a, b) in portals.pairs() {
                if !extras.contains(&(a, b)) {
                    // println!(" > {a} <-> {b}");
                    let strain = final_portal.portal_strain(true, true, &Some((a, b)));
                    // println!("   (strain = {strain})");
                    if strain < min_strain {
                        min_strain = strain;
                        pair = (a, b);
                    }
                }
            }
            println!("inserted {pair:?} @ {min_strain}");
            overall_min_strain = overall_min_strain.min(min_strain);
        }
        s1 - overall_min_strain
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

#[derive(Debug)]
struct PairIterator<'a, T> {
    i: usize,
    j: usize,
    list: &'a Vec<T>,
}

trait GeneratePairs<T> {
    fn pairs<'a>(&'a self) -> PairIterator<'a, T>;
}

impl<T> GeneratePairs<T> for Vec<T> {
    fn pairs<'a>(&'a self) -> PairIterator<'a, T> {
        PairIterator {
            i: 0,
            j: 1,
            list: self,
        }
    }
}

impl<'a, T: Copy> Iterator for PairIterator<'a, T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i + 1 >= self.list.len() {
            None
        } else {
            let retval = Some((self.list[self.i], self.list[self.j]));

            self.j += 1;
            if self.j >= self.list.len() {
                self.i += 1;
                self.j = self.i + 1;
            }

            retval
        }
    }
}

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
            connections: HashSet::new(),
        };
        assert_eq!(330, portal.simple_portal_strain());
    }

    #[test]
    fn test_31_portal_count() {
        let portal = Portal {
            portals: 31,
            connections: HashSet::new(),
        };
        assert_eq!(7440, portal.simple_portal_strain());
        assert_eq!(7440, portal.dijkstra_strain());
    }

    #[test]
    fn test_part2() {
        let test_input = include_str!("../test-input/part2");
        let portal = Portal::new(test_input.lines());
        assert_eq!(120, portal.portal_strain(true, false, &None));
        assert_eq!(6, portal.advanced_portal_resonance());
    }

    #[test]
    fn test_part_3a() {
        let test_input = include_str!("../test-input/part3.1");
        let portal = Portal::new(test_input.lines());
        assert_eq!(854, portal.spaced_portal_resonance());
    }

    #[test]
    fn test_part_3b() {
        let test_input = include_str!("../test-input/part3.2");
        let portal = Portal::new(test_input.lines());
        assert_eq!(9634, portal.spaced_portal_resonance());
    }

    #[test]
    fn test_pair_iterator() {
        let vec = vec![2, 3, 5, 7, 11, 13];
        let mut pair_iter = vec.pairs();

        assert_eq!(Some((2, 3)), pair_iter.next());
        assert_eq!(Some((2, 5)), pair_iter.next());
        assert_eq!(Some((2, 7)), pair_iter.next());
        assert_eq!(Some((2, 11)), pair_iter.next());
        assert_eq!(Some((2, 13)), pair_iter.next());

        assert_eq!(Some((3, 5)), pair_iter.next());
        assert_eq!(Some((3, 7)), pair_iter.next());
        assert_eq!(Some((3, 11)), pair_iter.next());
        assert_eq!(Some((3, 13)), pair_iter.next());

        assert_eq!(Some((5, 7)), pair_iter.next());
        assert_eq!(Some((5, 11)), pair_iter.next());
        assert_eq!(Some((5, 13)), pair_iter.next());

        assert_eq!(Some((7, 11)), pair_iter.next());
        assert_eq!(Some((7, 13)), pair_iter.next());

        assert_eq!(Some((11, 13)), pair_iter.next());

        assert_eq!(None, pair_iter.next());
    }
}
