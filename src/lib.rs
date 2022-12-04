#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::{collections::HashSet, str::FromStr};

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
enum Value {
    Rock,
    Paper,
    Scisor,
}

#[derive(Debug, PartialEq)]
struct Play {
    mine: Value,
    opponents: Value,
}

impl Play {
    fn value(self) -> usize {
        match (self.mine, self.opponents) {
            (Value::Rock, Value::Rock) => 1 + 3,
            (Value::Rock, Value::Paper) => 1,
            (Value::Rock, Value::Scisor) => 1 + 6,

            (Value::Paper, Value::Rock) => 2 + 6,
            (Value::Paper, Value::Paper) => 2 + 3,
            (Value::Paper, Value::Scisor) => 2,

            (Value::Scisor, Value::Rock) => 3,
            (Value::Scisor, Value::Paper) => 3 + 6,
            (Value::Scisor, Value::Scisor) => 3 + 3,
        }
    }
}

#[derive(Debug, PartialEq)]
enum PlayResult {
    Lose,
    Win,
    Draw,
}

#[derive(Debug, PartialEq)]
struct CheatPlay {
    opponent: Value,
    result: PlayResult,
}

impl CheatPlay {
    fn value(self) -> usize {
        match (self.opponent, self.result) {
            (Value::Rock, PlayResult::Win) => 2 + 6, // we play paper (+2)
            (Value::Rock, PlayResult::Draw) => 1 + 3, // we play rock (+1)
            (Value::Rock, PlayResult::Lose) => 3,    // we play scissors (+3)

            (Value::Paper, PlayResult::Win) => 3 + 6, // we play scissors (+3)
            (Value::Paper, PlayResult::Draw) => 2 + 3, // we play paper (+2)
            (Value::Paper, PlayResult::Lose) => 1,    // we play rock (+1)

            (Value::Scisor, PlayResult::Win) => 1 + 6, // we play rock (+1)
            (Value::Scisor, PlayResult::Draw) => 3 + 3, // we play scissors (+3)
            (Value::Scisor, PlayResult::Lose) => 2,    // we play paper (+2)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Ooops(String);

impl FromStr for PlayResult {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(PlayResult::Lose),
            "Y" => Ok(PlayResult::Draw),
            "Z" => Ok(PlayResult::Win),
            _ => Err(Ooops(format!("[a] invalid s='{}'", s))),
        }
    }
}

impl FromStr for Value {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s == "A" || s == "X" => Ok(Value::Rock),
            s if s == "B" || s == "Y" => Ok(Value::Paper),
            s if s == "C" || s == "Z" => Ok(Value::Scisor),
            _ => Err(Ooops(format!("[b] invalid s='{}'", s))),
        }
    }
}

impl FromStr for Play {
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
            opponents: opponents.parse()?,
        })
    }
}

impl FromStr for CheatPlay {
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
    shared: HashSet<String>,
}

impl Rucksack {
    fn intersection(&self, others: Vec<&Rucksack>) -> HashSet<String> {
        let this = format!("{}{}", self.compartment_a, self.compartment_b)
            .chars()
            .map(|v| v.to_string())
            .collect::<HashSet<String>>();
        let mut result = this;

        for other in others {
            let other = format!("{}{}", other.compartment_a, other.compartment_b)
                .chars()
                .map(|v| v.to_string())
                .collect::<HashSet<String>>();
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
                shared.insert(c.into());
            }
        }
        Ok(Self {
            compartment_a: compartment_a.into(),
            compartment_b: compartment_b.into(),
            shared,
        })
    }
}

fn priority(c: impl Into<String>) -> Result<usize, Ooops> {
    let c = c.into();
    let c = c.chars().next().unwrap();
    let index = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(c);
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
        .filter_map(|v| v.parse::<Play>().ok())
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
        .filter_map(|v| v.parse::<CheatPlay>().ok())
        .map(|v| v.value())
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
        assert_eq!(Ok(PlayResult::Lose), "X".parse());
        assert_eq!(Ok(PlayResult::Draw), "Y".parse());
        assert_eq!(Ok(PlayResult::Win), "Z".parse());
    }

    #[test]
    fn cheat_play_calculation() {
        assert_eq!(4, "A Y".parse::<CheatPlay>().unwrap().value());
        assert_eq!(1, "B X".parse::<CheatPlay>().unwrap().value());
        assert_eq!(7, "C Z".parse::<CheatPlay>().unwrap().value());
    }

    #[test]
    fn split_rucksack() {
        assert_eq!(
            Rucksack {
                compartment_a: "vJrwpWtwJgWr".into(),
                compartment_b: "hcsFMMfFFhFp".into(),
                shared: HashSet::from(["p".to_string()])
            },
            "vJrwpWtwJgWrhcsFMMfFFhFp".parse().unwrap()
        );
        assert_eq!(
            Rucksack {
                compartment_a: "jqHRNqRjqzjGDLGL".into(),
                compartment_b: "rsFMfFZSrLrFZsSL".into(),
                shared: HashSet::from(["L".to_string()]),
            },
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".parse().unwrap()
        );
        assert_eq!(
            Rucksack {
                compartment_a: "PmmdzqPrV".into(),
                compartment_b: "vPwwTWBwg".into(),
                shared: HashSet::from(["P".to_string()])
            },
            "PmmdzqPrVvPwwTWBwg".parse().unwrap()
        );
        assert_eq!(
            HashSet::from(["v".to_string()]),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
                .parse::<Rucksack>()
                .unwrap()
                .shared
        );
        assert_eq!(
            HashSet::from(["t".to_string()]),
            "ttgJtRGJQctTZtZT".parse::<Rucksack>().unwrap().shared
        );
        assert_eq!(
            HashSet::from(["s".to_string()]),
            "CrZsJsPPZsGzwwsLwLmpwMDw"
                .parse::<Rucksack>()
                .unwrap()
                .shared
        );
    }

    #[test]
    fn calculate_priority() {
        assert_eq!(Ok(1), priority('a'));
        assert_eq!(Ok(26), priority('z'));
        assert_eq!(Ok(27), priority('A'));
        assert_eq!(Ok(52), priority('Z'));
        assert_eq!(Ok(16), priority('p'));
        assert_eq!(Ok(38), priority('L'));
        assert_eq!(Ok(42), priority('P'));
        assert_eq!(Ok(22), priority('v'));
        assert_eq!(Ok(20), priority('t'));
        assert_eq!(Ok(19), priority('s'));
    }

    #[test]
    fn calculate_shared_intersection() {
        let one = "vJrwpWtwJgWrhcsFMMfFFhFp".parse::<Rucksack>().unwrap();
        let two = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
            .parse::<Rucksack>()
            .unwrap();
        let three = "PmmdzqPrVvPwwTWBwg".parse::<Rucksack>().unwrap();
        assert_eq!(
            HashSet::from(["r".to_string()]),
            one.intersection(vec![&two, &three])
        )
    }

    #[test]
    fn calculate_shared_intersection_2() {
        let one = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
            .parse::<Rucksack>()
            .unwrap();
        let two = "ttgJtRGJQctTZtZT".parse::<Rucksack>().unwrap();
        let three = "CrZsJsPPZsGzwwsLwLmpwMDw".parse::<Rucksack>().unwrap();
        assert_eq!(
            HashSet::from(["Z".to_string()]),
            one.intersection(vec![&two, &three])
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
                    &"ttgJtRGJQctTZtZT".parse::<Rucksack>().unwrap(),
                    &"CrZsJsPPZsGzwwsLwLmpwMDw".parse::<Rucksack>().unwrap()
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
                    &"jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
                        .parse::<Rucksack>()
                        .unwrap(),
                    &"PmmdzqPrVvPwwTWBwg".parse::<Rucksack>().unwrap()
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
