use std::{collections::HashSet, str::FromStr};

use crate::error::Ooops;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Rucksack {
    compartment_a: String,
    compartment_b: String,
    pub(crate) shared: HashSet<char>,
}

impl Rucksack {
    pub(crate) fn intersection(&self, others: Vec<&Rucksack>) -> HashSet<char> {
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

pub(crate) fn priority(c: &char) -> Result<usize, Ooops> {
    let index = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".find(*c);
    if index.is_none() {
        return Err(Ooops(format!("{} is not valid", c)));
    }
    Ok(index.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
