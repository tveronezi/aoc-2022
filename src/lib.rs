use std::{collections::HashSet, str::FromStr};

fn group_max(values: &'_ str) -> impl Iterator<Item = usize> + '_ {
    values
        .split("\n\n")
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| {
            v.split('\n')
                .map(|v| v.trim())
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<usize>().unwrap())
                .sum()
        })
}

pub fn max_calories(values: &str) -> usize {
    return group_max(values).fold(0, usize::max);
}

pub fn max_calories_top_3(values: &str) -> usize {
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
    pub fn value(self) -> usize {
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
    pub fn value(self) -> usize {
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
struct Rucksack {
    compartment_a: String,
    compartment_b: String,
    shared: HashSet<String>,
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

pub fn rps_result(values: &str) -> usize {
    values
        .trim()
        .split('\n')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<Play>().unwrap().value())
        .sum()
}

pub fn cheat_rps_result(values: &str) -> usize {
    values
        .trim()
        .split('\n')
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| v.parse::<CheatPlay>().unwrap().value())
        .sum()
}

pub fn total_priority_result(values: &str) -> usize {
    values
        .trim()
        .lines()
        .map(|v| v.trim())
        .filter(|v| !v.is_empty())
        .map(|v| {
            v.parse::<Rucksack>()
                .unwrap()
                .shared
                .iter()
                .map(priority)
                .filter_map(|v| v.ok())
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1_a() {
        let input = include_str!("day1.txt");
        assert_eq!(max_calories(input), 69693);
    }

    #[test]
    fn day_1_b() {
        let input = include_str!("day1.txt");
        assert_eq!(max_calories_top_3(input), 200945);
    }

    #[test]
    fn day_2_a() {
        let input = include_str!("day2.txt");
        assert_eq!(rps_result(input), 14827);
    }

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
    fn day_2_b() {
        let input = include_str!("day2.txt");
        assert_eq!(cheat_rps_result(input), 13889);
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
    fn day_3_a() {
        let input = include_str!("day3.txt");
        assert_eq!(total_priority_result(input), 8153);
    }
}
