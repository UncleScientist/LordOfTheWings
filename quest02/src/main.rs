use std::{cmp::Ordering, collections::HashSet, convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q02_p1.txt").expect("file");

    println!(
        "Quest 2, part 1 = {}",
        data.lines()
            .map(|line| line.parse::<Notes>().unwrap().find_order())
            .sum::<usize>()
    );
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Blade(u8);

impl From<char> for Blade {
    fn from(value: char) -> Self {
        Self(value as u8 - b'0')
    }
}

#[derive(Debug)]
struct Notes {
    harder: HashSet<(Blade, Blade)>,
    softer: HashSet<(Blade, Blade)>,
    blades: HashSet<Blade>,
}

impl Notes {
    fn find_order(&self) -> usize {
        let mut vec = self.blades.iter().copied().collect::<Vec<_>>();

        vec.sort_by(|a, b| {
            if self.harder.contains(&(*a, *b)) {
                Ordering::Less
            } else if self.softer.contains(&(*a, *b)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        vec.iter()
            .map(|Blade(b)| *b as usize)
            .fold(0, |total, cur| (total * 10) + cur)
    }
}

impl FromStr for Notes {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut harder = HashSet::new();
        let mut softer = HashSet::new();
        let mut blades = HashSet::new();

        for comparison in line.split(' ') {
            let chars = comparison.chars().collect::<Vec<_>>();
            match chars[1] {
                '<' => {
                    harder.insert((chars[2].into(), chars[0].into()));
                    softer.insert((chars[0].into(), chars[2].into()));
                }
                '>' => {
                    harder.insert((chars[0].into(), chars[2].into()));
                    softer.insert((chars[2].into(), chars[0].into()));
                }
                _ => panic!("unexpected comparison {comparison}"),
            }
            blades.insert(chars[0].into());
            blades.insert(chars[2].into());
        }

        Ok(Self {
            harder,
            softer,
            blades,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_line() {
        let note = "4<3 2<5 3>1 1>2 1<5 4<1 3>2 4<2 3>5 4<5"
            .parse::<Notes>()
            .unwrap();
        assert_eq!(35124, note.find_order());
    }
}
