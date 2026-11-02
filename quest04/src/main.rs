use std::{cmp::Ordering, collections::HashMap, convert::Infallible, str::FromStr};

fn main() {
    let data = std::fs::read_to_string("input/everybody_codes_e2026_q04_p1.txt").expect("file");
    let part1 = Results::new(data.lines().map(|line| line.parse::<LogLine>().unwrap()));
    println!("Quest 4, part 1 = {}", part1.total_time_checksum());

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q04_p2.txt").expect("file");
    let part2 = Results::new(data.lines().map(|line| line.parse::<LogLine>().unwrap()));
    println!(
        "Quest 4, part 2 = {}",
        part2.total_parts_and_time_checksum()
    );

    /*
    let data = std::fs::read_to_string("test-input/big-test-input.txt").expect("file");
    let part3 = Results::new(data.lines().map(|line| line.parse::<LogLine>().unwrap()));
    println!("Quest 4, part 3 = {}", part3.total_ranking_points());
    */

    let data = std::fs::read_to_string("input/everybody_codes_e2026_q04_p3.txt").expect("file");
    let part3 = Results::new(data.lines().map(|line| line.parse::<LogLine>().unwrap()));
    println!("Quest 4, part 3 = {}", part3.total_ranking_points());
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

    fn total_time_checksum(&self) -> usize {
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

    fn total_parts_and_time_checksum(&self) -> usize {
        let mut results = self
            .participants
            .values()
            .map(|results| (results.len(), results.iter().map(|r| r.time).sum::<usize>()))
            .collect::<Vec<_>>();
        results.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                b.0.cmp(&a.0)
            }
        });
        results
            .iter()
            .enumerate()
            .map(|(idx, (_, val))| (idx + 1) * val)
            .sum::<usize>()
    }

    fn total_ranking_points(&self) -> usize {
        let mut summaries = Vec::<Vec<Summary>>::new();
        let mut max_quest = 0;
        let mut max_participants = 0;
        for (participant_id, p) in &self.participants {
            let idx = *participant_id - 1;
            max_participants = max_participants.max(idx);
            if summaries.len() <= idx {
                summaries.resize(idx + 2, Vec::default());
            }
            for entry in p {
                let quest_idx = entry.quest - 1;
                max_quest = max_quest.max(quest_idx);
                if summaries[idx].len() <= quest_idx {
                    summaries[idx].resize(quest_idx + 2, Summary::default());
                }
                summaries[idx][quest_idx].id = entry.id;
                summaries[idx][quest_idx].quest = entry.quest;
                if entry.part == 3 {
                    summaries[idx][quest_idx].completed_part3 = true;
                    summaries[idx][quest_idx].part3_time += entry.time;
                }
                if entry.part == 2 {
                    summaries[idx][quest_idx].completed_part2 = true;
                    summaries[idx][quest_idx].part2_time += entry.time;
                    summaries[idx][quest_idx].part3_time += entry.time;
                }
                if entry.part == 1 {
                    summaries[idx][quest_idx].part1_time += entry.time;
                    summaries[idx][quest_idx].part2_time += entry.time;
                    summaries[idx][quest_idx].part3_time += entry.time;
                }
            }
        }

        for quest in 0..=max_quest {
            let mut quest_data = summaries
                .iter_mut()
                .filter(|participant| participant.len() > quest)
                .map(|participant| &mut participant[quest])
                .filter(|participant| participant.id != 0)
                .collect::<Vec<_>>();

            quest_data.sort_by_key(|a| a.part1_time);

            let (mut prev_time, mut prev_score) = (0, 0);
            for (idx, q) in quest_data.iter_mut().enumerate() {
                if q.part1_time == prev_time {
                    q.score += prev_score;
                } else {
                    prev_score = 50usize.saturating_sub(idx);
                    prev_time = q.part1_time;
                    q.score += prev_score;
                }
            }

            quest_data.sort_by(|a, b| {
                if a.completed_part2 && b.completed_part2 {
                    a.part2_time.cmp(&b.part2_time)
                } else if a.completed_part2 {
                    Ordering::Less
                } else if b.completed_part2 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            let (mut prev_time, mut prev_score) = (0, 0);
            for (idx, q) in quest_data
                .iter_mut()
                .filter(|q| q.completed_part2)
                .enumerate()
            {
                if q.part2_time == prev_time {
                    q.score += prev_score;
                } else {
                    prev_score = 100usize.saturating_sub(idx);
                    prev_time = q.part2_time;
                    q.score += prev_score;
                }
            }

            quest_data.sort_by(|a, b| {
                if a.completed_part3 && b.completed_part3 {
                    a.part3_time.cmp(&b.part3_time)
                } else if a.completed_part3 {
                    Ordering::Less
                } else if b.completed_part3 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            let (mut prev_time, mut prev_score) = (0, 0);
            for (idx, q) in quest_data
                .iter_mut()
                .filter(|q| q.completed_part3)
                .enumerate()
            {
                if q.part3_time == prev_time {
                    q.score += prev_score;
                } else {
                    prev_score = 150usize.saturating_sub(idx);
                    prev_time = q.part3_time;
                    q.score += prev_score;
                }
            }
        }

        let mut scores = vec![0; max_participants + 1];
        for quests in summaries {
            for summary in quests {
                if summary.id == 0 {
                    continue;
                }
                scores[summary.id - 1] += summary.score;
            }
        }
        scores.sort_by(|a, b| b.cmp(a));

        let (mut prev_score, mut prev_rank) = (0, 0);
        let mut total = 0;
        for (idx, score) in scores.iter().enumerate() {
            let rank = idx + 1;

            if prev_score == *score {
                total += prev_score * prev_rank;
            } else {
                total += score * rank;
                prev_score = *score;
                prev_rank = rank;
            }
        }
        total
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Summary {
    id: usize,
    quest: usize,
    part1_time: usize,
    part2_time: usize,
    part3_time: usize,
    completed_part2: bool,
    completed_part3: bool,
    score: usize,
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
        assert_eq!(27598, results.total_time_checksum());
    }

    #[test]
    fn test_part_2() {
        let data = std::fs::read_to_string("test-input/part2").expect("file");
        let results = Results::new(data.lines().map(|line| line.parse::<LogLine>().unwrap()));
        assert_eq!(6470, results.total_parts_and_time_checksum());
    }

    #[test]
    fn test_part_3a() {
        let data = std::fs::read_to_string("test-input/part3a").expect("file");
        let results = Results::new(data.lines().map(|line| line.parse::<LogLine>().unwrap()));
        assert_eq!(2998, results.total_ranking_points());
    }

    #[test]
    fn test_part_3b() {
        let data = std::fs::read_to_string("test-input/part3b").expect("file");
        let results = Results::new(data.lines().map(|line| line.parse::<LogLine>().unwrap()));
        assert_eq!(3004, results.total_ranking_points());
    }
}
