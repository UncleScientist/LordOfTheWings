use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q03_p1.txt").expect("file");
    println!(
        "Quest 3, part 1 = {}",
        data.lines()
            .map(|line| line.parse::<Iso>().unwrap().value())
            .sum::<usize>()
    );

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q03_p2.txt").expect("file");
    println!(
        "Quest 3, part 2 = {}",
        data.lines()
            .map(|line| line.parse::<Iso>().unwrap().value())
            .sum::<usize>()
    );

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q03_p3.txt").expect("file");
    let isos = data
        .lines()
        .map(|line| line.parse::<Iso>().unwrap())
        .collect::<Vec<_>>();
    println!("Quest 3, part 3 = {}", calc_part_3(&isos));
}

fn calc_part_3(isos: &[Iso]) -> usize {
    isos.iter()
        .filter(|iso| iso.is_valid())
        .map(|iso| iso.value())
        .sum::<usize>()
}

#[derive(PartialEq, Eq, Debug)]
struct Iso {
    bases: Vec<usize>,
    digits: Vec<usize>,
}

impl Iso {
    fn value(&self) -> usize {
        self.digits
            .iter()
            .zip(self.bases.iter().cycle())
            .enumerate()
            .map(|(exp, (factor, base))| base.pow(exp as u32) * factor)
            .sum::<usize>()
    }

    fn decimal_representation(&self) -> usize {
        self.digits
            .iter()
            .rev()
            .fold(0, |total, digit| total * 10 + digit)
    }

    fn is_valid(&self) -> bool {
        let actual_value = self.value();
        let dr = self.decimal_representation();

        for val in 1..dr {
            let s = format!("{val}")
                .chars()
                .rev()
                .map(|ch| (ch as u8 - b'0') as usize)
                .zip(self.bases.iter().cycle())
                .enumerate()
                .map(|(exp, (factor, base))| base.pow(exp as u32) * factor)
                .sum::<usize>();
            if s == actual_value {
                return false;
            }
        }
        true
    }
}

impl FromStr for Iso {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (iso_part, num_part) = line.split_once(' ').unwrap();
        let (_, iso) = iso_part.split_once('-').unwrap();
        Ok(Iso {
            bases: iso
                .chars()
                .rev()
                .map(|ch| (ch as u8 - b'0') as usize)
                .collect(),
            digits: num_part
                .chars()
                .rev()
                .map(|ch| (ch as u8 - b'0') as usize)
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_iso_1() {
        assert_eq!(
            Iso {
                bases: vec![0, 1, 2, 3],
                digits: vec![4, 5, 6, 7],
            },
            "ISO-3210 7654".parse::<Iso>().unwrap()
        );
    }

    #[test]
    fn test_calc_num() {
        let iso = "ISO-8742 2468".parse::<Iso>().unwrap();
        assert_eq!(1252, iso.value());
    }

    #[test]
    fn test_decimal_representation() {
        let iso = "ISO2-76877 123749".parse::<Iso>().unwrap();
        assert_eq!(123749, iso.decimal_representation());
    }

    #[test]
    fn test_part_3() {
        let data = std::fs::read_to_string("test-input/part3").expect("file");
        let isos = data
            .lines()
            .map(|line| line.parse::<Iso>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(50987, calc_part_3(&isos));
    }
}
