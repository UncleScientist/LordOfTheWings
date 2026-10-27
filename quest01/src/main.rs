use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q01_p1.txt").expect("input");
    println!("Quest 1, part 1 = {}", total_cones_needed(data));
}

fn total_cones_needed<S: AsRef<str>>(input: S) -> usize {
    input
        .as_ref()
        .split(',')
        .map(|fort| fort.parse::<Fortress>().unwrap())
        .map(|fortress| fortress.cones_missing())
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Fortress(usize);

impl Fortress {
    fn cones_missing(&self) -> usize {
        let sqrt = self.0.isqrt();

        if sqrt * sqrt == self.0 {
            0
        } else {
            (sqrt + 1) * (sqrt + 1) - self.0
        }
    }
}

impl FromStr for Fortress {
    type Err = Infallible;

    fn from_str(num: &str) -> Result<Self, Self::Err> {
        Ok(Self(num.parse().unwrap()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fortress() {
        assert_eq!(Fortress(27), "27".parse::<Fortress>().unwrap());
    }

    #[test]
    fn test_part1() {
        let notes = "3,11,12,16,24,25";
        assert_eq!(11, total_cones_needed(notes));
    }
}
