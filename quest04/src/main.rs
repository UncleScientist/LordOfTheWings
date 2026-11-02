use std::{collections::HashMap, convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q04_p1.txt").expect("file");
    let part1 = Results::new(data.lines().map(|line| line.parse::<LogLine>().unwrap()));
    println!("Quest 4, part 1 = {}", part1.checksum());
}

#[derive(Debug)]
struct Results {
    participants: HashMap<usize, Vec<LogLine>>,
}

impl Results {
    fn new<I>(log_lines: I) -> Self
    where
        I: IntoIterator<Item = LogLine>,
    {
        let mut participants = HashMap::<usize, Vec<LogLine>>::new();
        for line in log_lines {
            participants.entry(line.id).or_default().push(line);
        }

        Self { participants }
    }

    fn checksum(&self) -> usize {
        let mut results = self
            .participants
            .values()
            .map(|results| results.iter().map(|r| r.time).sum::<usize>())
            .collect::<Vec<_>>();
        results.sort();
        results
            .iter()
            .enumerate()
            .map(|(idx, val)| (idx + 1) * val)
            .sum::<usize>()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct LogLine {
    id: usize,
    quest: usize,
    part: usize,
    time: usize,
}

impl FromStr for LogLine {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words = line
            .split_whitespace()
            .map(|val| val.split_once('=').unwrap().1.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self {
            id: words[0],
            quest: words[1],
            part: words[2],
            time: words[3],
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_log_line() {
        assert_eq!(
            LogLine {
                id: 1,
                quest: 1,
                part: 1,
                time: 187
            },
            "id=1 quest=1 part=1 time=187".parse::<LogLine>().unwrap()
        );
    }

    #[test]
    fn test_part_1() {
        let data = std::fs::read_to_string("test-input/part1").expect("file");
        let results = Results::new(data.lines().map(|line| line.parse::<LogLine>().unwrap()));
        assert_eq!(27598, results.checksum());
    }
}
