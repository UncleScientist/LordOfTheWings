use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q01_p1.txt").expect("input");
    println!(
        "Quest 1, part 1 = {}",
        total_cones_needed(data, Fortress::square_cones_missing)
    );

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q01_p2.txt").expect("input");
    println!(
        "Quest 1, part 2 = {}",
        total_cones_needed(data, Fortress::perimeter_cones_missing)
    );

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q01_p3.txt").expect("input");
    println!(
        "Quest 1, part 3 = {}",
        total_cones_needed(data, Fortress::odd_square_cones_missing)
    );
}

fn total_cones_needed<S: AsRef<str>>(input: S, func: impl Fn(&Fortress) -> usize) -> usize {
    input
        .as_ref()
        .split(',')
        .map(|fort| fort.parse::<Fortress>().unwrap())
        .map(|fortress| func(&fortress))
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Fortress(usize);

impl Fortress {
    fn cones_missing(&self, func: impl Fn(usize) -> (usize, usize)) -> usize {
        let (low, high) = func(self.0);
        if self.0 == low { 0 } else { high - self.0 }
    }

    fn square_cones_missing(&self) -> usize {
        self.cones_missing(|target| {
            let sqrt = target.isqrt();
            (sqrt * sqrt, (sqrt + 1) * (sqrt + 1))
        })
    }

    fn perimeter_cones_missing(&self) -> usize {
        self.cones_missing(|target| {
            let grid_size = 2.max(1 + target / 4);
            let lower_limit = 8.max((grid_size - 1) * 4);
            let upper_limit = grid_size * 4;
            (lower_limit, upper_limit)
        })
    }

    fn odd_square_cones_missing(&self) -> usize {
        self.cones_missing(|target| {
            let sqrt = target.isqrt();
            let (low, high) = if sqrt.is_multiple_of(2) {
                (sqrt - 1, sqrt + 1)
            } else {
                (sqrt, sqrt + 2)
            };
            (low * low - 1, high * high - 1)
        })
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
        assert_eq!(
            11,
            total_cones_needed(notes, Fortress::square_cones_missing)
        );
    }

    #[test]
    fn test_part2() {
        let notes = "3,11,12,16,24,25";
        assert_eq!(
            9,
            total_cones_needed(notes, Fortress::perimeter_cones_missing)
        );
    }

    #[test]
    fn test_perimeter_4_works() {
        let notes = "4";
        assert_eq!(
            4,
            total_cones_needed(notes, Fortress::perimeter_cones_missing)
        );
    }

    #[test]
    fn test_part3() {
        let notes = "3,11,12,16,24,25";
        assert_eq!(
            61,
            total_cones_needed(notes, Fortress::odd_square_cones_missing)
        );
    }
}
