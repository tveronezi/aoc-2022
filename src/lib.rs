#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::{collections::HashSet, fmt::Display, str::FromStr};

/// Input data for day 1
pub const DAY1: &str = include_str!("day1.txt");

/// Input data for day 2
pub const DAY2: &str = include_str!("day2.txt");

/// Input data for day 3
pub const DAY3: &str = include_str!("day3.txt");

/// Input data for day 4
pub const DAY4: &str = include_str!("day4.txt");

fn group_max(values: &'_ str) -> impl Iterator<Item = usize> + '_ {
    values
        .split("\n\n")
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| {
            v.lines()
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .filter_map(|v| v.parse::<usize>().ok())
                .sum()
        })
}

/// Part A -> <https://adventofcode.com/2022/day/1>
pub fn total_of_calories_with_the_elf_with_the_most_calories(values: &str) -> usize {
    return group_max(values).fold(0, usize::max);
}

/// Part B -> <https://adventofcode.com/2022/day/1>
pub fn total_of_calories_for_the_top_three_elfs(values: &str) -> usize {
    let mut values = group_max(values).collect::<Vec<usize>>();
    values.sort();
    values.reverse();
    let values = values.iter();
    values.take(3).sum()
}

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scisor,
}

#[derive(Debug, PartialEq)]
enum RpsMatchResult {
    Winner,
    Loser,
    Draw,
}

impl RpsMatchResult {
    fn value(&self) -> usize {
        match self {
            RpsMatchResult::Winner => 6,
            RpsMatchResult::Draw => 3,
            RpsMatchResult::Loser => 0,
        }
    }

    fn hand(&self, against: &Hand) -> Hand {
        match (self, against) {
            (RpsMatchResult::Winner, Hand::Rock) => Hand::Paper,
            (RpsMatchResult::Winner, Hand::Paper) => Hand::Scisor,
            (RpsMatchResult::Winner, Hand::Scisor) => Hand::Rock,
            (RpsMatchResult::Loser, Hand::Rock) => Hand::Scisor,
            (RpsMatchResult::Loser, Hand::Paper) => Hand::Rock,
            (RpsMatchResult::Loser, Hand::Scisor) => Hand::Paper,
            (RpsMatchResult::Draw, Hand::Rock) => Hand::Rock,
            (RpsMatchResult::Draw, Hand::Paper) => Hand::Paper,
            (RpsMatchResult::Draw, Hand::Scisor) => Hand::Scisor,
        }
    }
}

impl Hand {
    fn fight(&self, other: &Hand) -> RpsMatchResult {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => RpsMatchResult::Draw,
            (Hand::Paper, Hand::Paper) => RpsMatchResult::Draw,
            (Hand::Scisor, Hand::Scisor) => RpsMatchResult::Draw,
            (Hand::Rock, Hand::Paper) => RpsMatchResult::Loser,
            (Hand::Rock, Hand::Scisor) => RpsMatchResult::Winner,
            (Hand::Paper, Hand::Rock) => RpsMatchResult::Winner,
            (Hand::Paper, Hand::Scisor) => RpsMatchResult::Loser,
            (Hand::Scisor, Hand::Rock) => RpsMatchResult::Loser,
            (Hand::Scisor, Hand::Paper) => RpsMatchResult::Winner,
        }
    }

    fn weight(&self) -> usize {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scisor => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
struct RpsPlay {
    mine: Hand,
    opponent: Hand,
}

impl RpsPlay {
    fn value(self) -> usize {
        self.mine.weight() + self.mine.fight(&self.opponent).value()
    }
}

#[derive(Debug, PartialEq)]
struct CheatRpsPlay {
    opponent: Hand,
    result: RpsMatchResult,
}

impl CheatRpsPlay {
    fn fight(self) -> usize {
        let my_hand = self.result.hand(&self.opponent);
        my_hand.weight() + self.result.value()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Ooops(String);

impl std::error::Error for Ooops {}

impl Display for Ooops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RpsMatchResult {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RpsMatchResult::Loser),
            "Y" => Ok(RpsMatchResult::Draw),
            "Z" => Ok(RpsMatchResult::Winner),
            _ => Err(Ooops(format!("[a] invalid s='{}'", s))),
        }
    }
}

impl FromStr for Hand {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scisor),
            _ => Err(Ooops(format!("[b] invalid s='{}'", s))),
        }
    }
}

impl FromStr for RpsPlay {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(' ').take(2);
        let opponents = values.next();
        if opponents.is_none() {
            return Err(Ooops(format!("[c] missing value '{}'", s)));
        }
        let opponents = opponents.unwrap();
        let mine = values.next();
        if mine.is_none() {
            return Err(Ooops(format!("[d] missing value '{}'", s)));
        }
        let mine = mine.unwrap();
        Ok(Self {
            mine: mine.parse()?,
            opponent: opponents.parse()?,
        })
    }
}

impl FromStr for CheatRpsPlay {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(' ').take(2);
        let opponent = values.next();
        if opponent.is_none() {
            return Err(Ooops(format!("[f] missing value '{}'", s)));
        }
        let opponent = opponent.unwrap();
        let result = values.next();
        if result.is_none() {
            return Err(Ooops(format!("[e] missing value '{}'", s)));
        }
        let result = result.unwrap();
        Ok(Self {
            opponent: opponent.parse()?,
            result: result.parse()?,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rucksack {
    compartment_a: String,
    compartment_b: String,
    shared: HashSet<char>,
}

impl Rucksack {
    fn intersection(&self, others: Vec<&Rucksack>) -> HashSet<char> {
        let this = format!("{}{}", self.compartment_a, self.compartment_b)
            .chars()
            .collect::<HashSet<char>>();
        let mut result = this;

        for other in others {
            let other = format!("{}{}", other.compartment_a, other.compartment_b)
                .chars()
                .collect::<HashSet<char>>();
            result = result.intersection(&other).map(|v| v.to_owned()).collect();
        }
        result
    }
}

impl FromStr for Rucksack {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (compartment_a, compartment_b) = s.split_at(s.len() / 2);
        if compartment_a.len() != compartment_b.len() {
            return Err(Ooops(format!(
                "compartments don't have the same number of elements. {}:{}",
                compartment_a, compartment_b
            )));
        }
        let mut shared = HashSet::new();
        for c in compartment_a.chars() {
            if compartment_b.contains(c) {
                shared.insert(c);
            }
        }
        Ok(Self {
            compartment_a: compartment_a.into(),
            compartment_b: compartment_b.into(),
            shared,
        })
    }
}

fn priority(c: &char) -> Result<usize, Ooops> {
    let index = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(*c);
    if index.is_none() {
        return Err(Ooops(format!("{} is not valid", c)));
    }
    Ok(index.unwrap())
}

/// Part A -> <https://adventofcode.com/2022/day/2>
pub fn total_score_according_to_your_strategy_guide(values: &str) -> usize {
    values
        .trim()
        .split('\n')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.parse::<RpsPlay>().ok())
        .map(|v| v.value())
        .sum()
}

/// Part B -> <https://adventofcode.com/2022/day/2>
pub fn total_score_according_to_the_elfs_strategy_guide(values: &str) -> usize {
    values
        .trim()
        .split('\n')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.parse::<CheatRpsPlay>().ok())
        .map(|v| v.fight())
        .sum()
}

/// Part A -> <https://adventofcode.com/2022/day/3>
pub fn the_sum_of_the_priorities_for_shared_item_types(values: &str) -> usize {
    values
        .trim()
        .lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .filter_map(|v| v.parse::<Rucksack>().ok())
        .map(|v| {
            v.shared
                .iter()
                .map(priority)
                .filter_map(|v| v.ok())
                .sum::<usize>()
        })
        .sum()
}

/// Part B -> <https://adventofcode.com/2022/day/3>
pub fn the_sum_of_the_priorities_for_shared_item_types_in_three_elfs_group(values: &str) -> usize {
    let mut iter = values
        .trim()
        .lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<Rucksack>())
        .filter_map(|v| v.ok());

    let mut results = vec![];
    while let (Some(one), Some(two), Some(three)) = (iter.next(), iter.next(), iter.next()) {
        results.push(one.intersection(vec![&two, &three]));
    }
    results
        .iter()
        .map(|v| v.iter().filter_map(|v| priority(v).ok()).sum::<usize>())
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct AssignmentRange {
    start: usize,
    end: usize,
}

impl AssignmentRange {
    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

impl FromStr for AssignmentRange {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split('-');
        let values = values.collect::<Vec<&str>>();
        let mut values = values.iter();
        let start = values.next();
        let end = values.next();
        if start.is_none() || end.is_none() {
            return Err(Ooops(format!("bad range {}", s)));
        }
        let start: usize = start
            .unwrap()
            .parse()
            .map_err(|e| Ooops(format!("{}", e)))?;
        let end: usize = end.unwrap().parse().map_err(|e| Ooops(format!("{}", e)))?;
        Ok(AssignmentRange { start, end })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct AssignmentPair {
    a: AssignmentRange,
    b: AssignmentRange,
}

impl FromStr for AssignmentPair {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(',');
        let values = values.collect::<Vec<&str>>();
        let mut values = values.iter();
        let a = values.next();
        let b = values.next();
        if a.is_none() || b.is_none() {
            return Err(Ooops(format!("bad pair {}", s)));
        }
        Ok(Self {
            a: a.unwrap().parse()?,
            b: b.unwrap().parse()?,
        })
    }
}

/// Part A -> <https://adventofcode.com/2022/day/4>
pub fn how_many_pairs_does_one_fully_contain_the_other(values: &str) -> usize {
    values
        .trim()
        .lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<AssignmentPair>())
        .filter_map(|v| v.ok())
        .filter(|v| v.a.fully_contains(&v.b) || v.b.fully_contains(&v.a))
        .count()
}

/// Part B -> <https://adventofcode.com/2022/day/4>
pub fn how_many_pairs_do_ranges_overlap(values: &str) -> usize {
    values
        .trim()
        .lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<AssignmentPair>())
        .filter_map(|v| v.ok())
        .filter(|v| v.a.overlaps(&v.b) || v.b.overlaps(&v.a))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_result_parse() {
        assert_eq!(Ok(RpsMatchResult::Loser), "X".parse());
        assert_eq!(Ok(RpsMatchResult::Draw), "Y".parse());
        assert_eq!(Ok(RpsMatchResult::Winner), "Z".parse());
    }

    #[test]
    fn cheat_play_calculation() {
        assert_eq!(4, "A Y".parse::<CheatRpsPlay>().unwrap().fight());
        assert_eq!(1, "B X".parse::<CheatRpsPlay>().unwrap().fight());
        assert_eq!(7, "C Z".parse::<CheatRpsPlay>().unwrap().fight());
    }

    #[test]
    fn split_rucksack() {
        assert_eq!(
            Rucksack {
                compartment_a: "vJrwpWtwJgWr".into(),
                compartment_b: "hcsFMMfFFhFp".into(),
                shared: HashSet::from(['p'])
            },
            "vJrwpWtwJgWrhcsFMMfFFhFp".parse().unwrap()
        );
        assert_eq!(
            Rucksack {
                compartment_a: "jqHRNqRjqzjGDLGL".into(),
                compartment_b: "rsFMfFZSrLrFZsSL".into(),
                shared: HashSet::from(['L']),
            },
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".parse().unwrap()
        );
        assert_eq!(
            Rucksack {
                compartment_a: "PmmdzqPrV".into(),
                compartment_b: "vPwwTWBwg".into(),
                shared: HashSet::from(['P'])
            },
            "PmmdzqPrVvPwwTWBwg".parse().unwrap()
        );
        assert_eq!(
            HashSet::from(['v']),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                .parse::<Rucksack>()
                .unwrap()
                .shared
        );
        assert_eq!(
            HashSet::from(['t']),
            "ttgJtRGJQctTZtZT".parse::<Rucksack>().unwrap().shared
        );
        assert_eq!(
            HashSet::from(['s']),
            "CrZsJsPPZsGzwwsLwLmpwMDw"
                .parse::<Rucksack>()
                .unwrap()
                .shared
        );
    }

    #[test]
    fn calculate_priority() {
        assert_eq!(Ok(1), priority(&'a'));
        assert_eq!(Ok(26), priority(&'z'));
        assert_eq!(Ok(27), priority(&'A'));
        assert_eq!(Ok(52), priority(&'Z'));
        assert_eq!(Ok(16), priority(&'p'));
        assert_eq!(Ok(38), priority(&'L'));
        assert_eq!(Ok(42), priority(&'P'));
        assert_eq!(Ok(22), priority(&'v'));
        assert_eq!(Ok(20), priority(&'t'));
        assert_eq!(Ok(19), priority(&'s'));
    }

    #[test]
    fn calculate_shared_intersection() {
        let one = "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Rucksack>().unwrap();
        let two = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
            .parse::<Rucksack>()
            .unwrap();
        let three = "PmmdzqPrVvPwwTWBwg".parse::<Rucksack>().unwrap();
        assert_eq!(HashSet::from(['r']), one.intersection(vec![&two, &three]))
    }

    #[test]
    fn calculate_shared_intersection_2() {
        assert_eq!(
            HashSet::from(['Z']),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                .parse::<Rucksack>()
                .unwrap()
                .intersection(vec![
                    &"ttgJtRGJQctTZtZT".parse().unwrap(),
                    &"CrZsJsPPZsGzwwsLwLmpwMDw".parse().unwrap()
                ])
        )
    }

    #[test]
    fn calculate_shared_intersection_3() {
        assert_eq!(
            52,
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                .parse::<Rucksack>()
                .unwrap()
                .intersection(vec![
                    &"ttgJtRGJQctTZtZT".parse().unwrap(),
                    &"CrZsJsPPZsGzwwsLwLmpwMDw".parse().unwrap()
                ])
                .iter()
                .filter_map(|v| priority(v).ok())
                .sum::<usize>()
        )
    }

    #[test]
    fn calculate_shared_intersection_4() {
        assert_eq!(
            18,
            "vJrwpWtwJgWrhcsFMMfFFhFp"
                .parse::<Rucksack>()
                .unwrap()
                .intersection(vec![
                    &"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".parse().unwrap(),
                    &"PmmdzqPrVvPwwTWBwg".parse().unwrap()
                ])
                .iter()
                .filter_map(|v| priority(v).ok())
                .sum::<usize>()
        )
    }

    #[test]
    fn parse_assignment_range() {
        assert_eq!(Ok(AssignmentRange { start: 2, end: 4 }), "2-4".parse());
        assert_eq!(Ok(AssignmentRange { start: 6, end: 8 }), "6-8".parse());
    }

    #[test]
    fn assignment_range_contains() {
        assert!(AssignmentRange { start: 2, end: 4 }
            .fully_contains(&AssignmentRange { start: 2, end: 4 }));
        assert!(AssignmentRange { start: 2, end: 4 }
            .fully_contains(&AssignmentRange { start: 3, end: 4 }));
        assert!(AssignmentRange { start: 2, end: 4 }
            .fully_contains(&AssignmentRange { start: 4, end: 4 }));
        assert!(AssignmentRange { start: 2, end: 4 }
            .fully_contains(&AssignmentRange { start: 2, end: 3 }));
        assert!(AssignmentRange { start: 2, end: 4 }
            .fully_contains(&AssignmentRange { start: 2, end: 2 }));
        assert!(!AssignmentRange { start: 2, end: 4 }
            .fully_contains(&AssignmentRange { start: 1, end: 4 }));
    }

    #[test]
    fn assignment_range_overlaps() {
        assert!(
            AssignmentRange { start: 2, end: 4 }.overlaps(&AssignmentRange { start: 2, end: 4 })
        );
        assert!(
            AssignmentRange { start: 2, end: 4 }.overlaps(&AssignmentRange { start: 3, end: 4 })
        );
        assert!(
            AssignmentRange { start: 2, end: 4 }.overlaps(&AssignmentRange { start: 0, end: 4 })
        );
        assert!(
            AssignmentRange { start: 2, end: 4 }.overlaps(&AssignmentRange { start: 0, end: 3 })
        );
        assert!(
            AssignmentRange { start: 2, end: 4 }.overlaps(&AssignmentRange { start: 4, end: 6 })
        );
        assert!(
            !AssignmentRange { start: 2, end: 4 }.overlaps(&AssignmentRange { start: 0, end: 1 })
        );
        assert!(
            !AssignmentRange { start: 2, end: 4 }.overlaps(&AssignmentRange { start: 6, end: 10 })
        );
    }
}
