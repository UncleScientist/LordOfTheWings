use std::{convert::Infallible, str::FromStr};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Clockwise,
    CounterClockwise,
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(match line {
            "CW" => Self::Clockwise,
            "CCW" => Self::CounterClockwise,
            _ => panic!("Invalid direction {line}"),
        })
    }
}

#[derive(Debug)]
struct Hand {
    positions: usize,
    direction: Direction,
    start: usize,
    time_remaining: usize,
}

impl Hand {
    fn new(pos: &str, dir: &str, start: &str, time: &str) -> Self {
        Self {
            positions: pos.parse().unwrap(),
            direction: dir.parse().unwrap(),
            start: start.parse().unwrap(),
            time_remaining: time.parse().unwrap(),
        }
    }

    fn get_rotation_at_time(&self, lcm: usize, req_sec: usize) -> Rotation {
        let units = lcm / self.positions;

        let sec = req_sec.min(self.time_remaining);
        let spin = ((sec - 1) * units) % lcm;
        let amount = match self.direction {
            Direction::Clockwise => spin % lcm,
            Direction::CounterClockwise => (lcm - spin) % lcm,
        };
        let start = (self.start * units + amount) % lcm;

        let end = match self.direction {
            Direction::Clockwise => (start + units) % lcm,
            Direction::CounterClockwise => (lcm - units + start) % lcm,
        };
        if req_sec > self.time_remaining {
            Rotation::Finished(end)
        } else {
            Rotation::Move { start, end }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Rotation {
    Move { start: usize, end: usize },
    Finished(usize),
}

impl Rotation {
    fn crosses(&self, hand2: &Rotation) -> usize {
        if let (Rotation::Finished(h1), Rotation::Finished(h2)) = (self, hand2) {
            (h1 == h2) as usize
        } else if let (Rotation::Finished(h1), Rotation::Move { start, end }) = (self, hand2) {
            (if start > end {
                start > h1 && end < h1
            } else {
                end > h1 && start < h1
            }) as usize
        } else if let (Rotation::Move { start, end }, Rotation::Finished(h2)) = (self, hand2) {
            (if start > end {
                start > h2 && end < h2
            } else {
                end > h2 && start < h2
            }) as usize
        } else {
            let Rotation::Move {
                start: h1start,
                end: h1end,
            } = self
            else {
                panic!("oops");
            };
            let Rotation::Move {
                start: _h2start,
                end: h2end,
            } = hand2
            else {
                panic!("oops");
            };
            if h1end == h2end {
                return 1;
            }
            let result = if h1end > h1start {
                (h1start < h2end && h1end > h2end) as usize
            } else {
                (h1end < h2end && h1start > h2end) as usize
            };
            println!("> self = {h1start},{h1end}, other = {_h2start},{h2end} -> {result}");
            result
        }
    }

    fn is_finished(&self) -> bool {
        matches!(self, Rotation::Finished(..))
    }
}

#[derive(Debug)]
struct Clock {
    hands: Vec<Hand>,
    lcm: usize,
}

impl Clock {
    fn new<'a, I>(lines: I) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut iter = lines.into_iter();

        let hands = iter.next().unwrap();
        let direction = iter.next().unwrap();
        let current = iter.next().unwrap();
        let movement = iter.next().unwrap();

        let positions = hands.split_once(' ').unwrap().1;
        let direction = direction.split_once(' ').unwrap().1;
        let start = current.split_once(' ').unwrap().1;
        let time_remaining = movement.split_once(' ').unwrap().1;

        let mut hands = Vec::new();
        for (pos, (dir, (start, time))) in positions.split(':').zip(
            direction
                .split(':')
                .zip(start.split(':').zip(time_remaining.split(':'))),
        ) {
            hands.push(Hand::new(pos, dir, start, time))
        }

        let lcm = hands
            .iter()
            .map(|hand| hand.positions)
            .collect::<Vec<_>>()
            .lcm()
            .unwrap();

        Clock { hands, lcm }
    }

    fn get_crossings_at_time(&self, time: usize) -> ClockCrossing {
        let hand1 = self.hands[0].get_rotation_at_time(self.lcm, time);
        let hand2 = self.hands[1].get_rotation_at_time(self.lcm, time);
        let hand3 = self.hands[2].get_rotation_at_time(self.lcm, time);

        if hand1.is_finished() && hand2.is_finished() && hand3.is_finished() {
            println!("{hand1:?} / {hand2:?} / {hand3:?}");
            ClockCrossing::Done(
                (hand1 == hand2) as usize + (hand1 == hand3) as usize + (hand2 == hand3) as usize,
            )
        } else {
            let h1h2 = hand1.crosses(&hand2);
            let h1h3 = hand1.crosses(&hand3);
            let h2h3 = hand2.crosses(&hand3);

            println!("{time:3} | {h1h2},{h1h3},{h2h3}");
            ClockCrossing::Continue(h1h2 + h1h3 + h2h3)
        }
    }

    fn calibrate_dial(&self) -> usize {
        println!("lcm = {}", self.lcm);

        let mut total = 0;
        for time in 1.. {
            match self.get_crossings_at_time(time) {
                ClockCrossing::Continue(val) => total += val,
                ClockCrossing::Done(val) => {
                    total += val;
                    break;
                }
            }
        }

        total
    }
}

enum ClockCrossing {
    Continue(usize),
    Done(usize),
}

pub trait Mathemagic<T> {
    type Output;
    fn gcd(self) -> Option<Self::Output>;
    fn lcm(self) -> Option<Self::Output>;
}

impl<T> Mathemagic<T> for &[T]
where
    T: Copy
        + std::default::Default
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    type Output = T;

    fn gcd(self) -> Option<Self::Output> {
        self.iter().copied().reduce(gcd)
    }

    fn lcm(self) -> Option<Self::Output> {
        self.iter().copied().reduce(lcm)
    }
}

pub fn gcd<T>(x: T, y: T) -> T
where
    T: Copy
        + std::default::Default
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>,
{
    // let zero: T = 0_u8.try_into().unwrap();
    if y == T::default() { x } else { gcd(y, x % y) }
}

pub fn lcm<T>(x: T, y: T) -> T
where
    T: Copy
        + std::default::Default
        + std::cmp::PartialOrd
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>,
{
    x * y / gcd(x, y)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_hand_cw() {
        let hand = Hand::new("6", "CW", "2", "9");
        assert_eq!(Rotation::Finished(20), hand.get_rotation_at_time(24, 10));
    }

    #[test]
    fn test_single_hand_ccw() {
        let hand = Hand::new("8", "CCW", "4", "7");
        assert_eq!(Rotation::Finished(15), hand.get_rotation_at_time(24, 8));
    }

    #[test]
    fn test_part1a() {
        let data = include_str!("../test-input/part1a");
        let clock = Clock::new(data.lines());
        assert_eq!(5, clock.calibrate_dial());
    }

    #[test]
    fn test_part1b() {
        let data = include_str!("../test-input/part1b");
        let clock = Clock::new(data.lines());
        assert_eq!(7, clock.calibrate_dial());
    }

    #[test]
    fn test_part1c() {
        let data = include_str!("../test-input/part1c");
        let clock = Clock::new(data.lines());
        assert_eq!(9, clock.calibrate_dial());
    }
}
