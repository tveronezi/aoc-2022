use std::{collections::VecDeque, fmt::Display, str::FromStr};

use crate::error::Ooops;

#[derive(Debug, PartialEq, Eq)]
struct Stacks {
    lines: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct Crates {
    crates: Vec<String>,
}

impl Iterator for Stacks {
    type Item = Crates;

    fn next(&mut self) -> Option<Self::Item> {
        let mut crates = Crates { crates: vec![] };
        for l in self.lines.iter() {
            let value = &l[1..2].trim();
            if value.is_empty() {
                break;
            }
            crates.crates.push(value.to_string());
        }
        self.lines = self
            .lines
            .iter()
            .map(|l| l.chars().skip(4).collect::<String>())
            .collect();
        if crates.crates.is_empty() {
            None
        } else {
            Some(crates)
        }
    }
}

impl FromStr for Stacks {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split("\n\n").next().ok_or(Ooops(
            "the first split always has at least one element".to_owned(),
        ))?;
        Ok(Self {
            lines: s
                .lines()
                .rev()
                .skip(1)
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
        })
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.lines
                .iter()
                .map(|l| l.trim())
                .collect::<Vec<&str>>()
                .join("\n")
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CrateAction {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for CrateAction {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re =
            regex::Regex::new(r"move (?P<move>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
        let result = re
            .captures_iter(s)
            .next()
            .map(|value| Self {
                quantity: value["move"]
                    .parse()
                    .expect("the regex should block this from happening"),
                from: value["from"]
                    .parse::<usize>()
                    .expect("the regex should block this from happening"),
                to: value["to"]
                    .parse::<usize>()
                    .expect("the regex should block this from happening"),
            })
            .ok_or_else(|| Ooops(format!("invalid action > '{}'", s)));
        result
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ActionsLines {
    lines: VecDeque<String>,
}

impl FromStr for ActionsLines {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .split("\n\n")
            .skip(1)
            .next()
            .ok_or(Ooops("missing actions in the repo".to_owned()))?;
        Ok(Self {
            lines: s
                .lines()
                .filter(|l| !l.trim().is_empty())
                .map(|s| s.to_owned())
                .collect::<VecDeque<String>>(),
        })
    }
}

impl Iterator for ActionsLines {
    type Item = CrateAction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut action = None;
        loop {
            let maybe_action = self.lines.pop_front().map(|l| l.parse::<CrateAction>());
            if maybe_action.is_none() {
                break;
            }
            if let Ok(a) = maybe_action.unwrap() {
                action = Some(a);
                break;
            }
        }
        action
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_actions() {
        let parsed: ActionsLines = crate::input::DAY5.parse().unwrap();
        assert_eq!(
            &"move 2 from 5 to 9".to_string(),
            parsed.lines.iter().nth(0).unwrap()
        );
        assert_eq!(
            &"move 3 from 1 to 7".to_string(),
            parsed.lines.iter().nth(1).unwrap()
        );
        assert_eq!(
            &"move 2 from 3 to 9".to_string(),
            parsed.lines.iter().nth(2).unwrap()
        );
    }

    #[test]
    fn parse_action() {
        assert_eq!(
            CrateAction {
                from: 5,
                to: 9,
                quantity: 2
            },
            "move 2 from 5 to 9".parse().unwrap()
        );
        assert_eq!(
            Err(Ooops("invalid action > 'banana'".to_string())),
            "banana".parse::<CrateAction>()
        );
    }

    #[test]
    fn iterate_actions() {
        let mut parsed: ActionsLines = crate::input::DAY5.parse().unwrap();
        assert_eq!(
            CrateAction {
                from: 5,
                to: 9,
                quantity: 2
            },
            parsed.next().unwrap()
        );
        assert_eq!(
            CrateAction {
                from: 1,
                to: 7,
                quantity: 3
            },
            parsed.next().unwrap()
        );
        assert_eq!(
            CrateAction {
                from: 3,
                to: 9,
                quantity: 2
            },
            parsed.next().unwrap()
        );
        assert_eq!(
            CrateAction {
                from: 9,
                to: 5,
                quantity: 6
            },
            parsed.next().unwrap()
        );
    }

    #[test]
    fn parse_crates_lines() {
        assert_eq!(
            Stacks {
                lines: vec![
                    "[S] [N] [F] [G] [W] [B] [H] [F] [N]".to_string(),
                    "[Z] [V] [W] [J] [J] [C] [T] [S] [C]".to_string(),
                    "[P] [G] [B] [N] [L] [W] [P] [W] [R]".to_string(),
                    "[D] [P] [J] [F] [T] [G] [M] [T]    ".to_string(),
                    "[L] [H] [G] [L] [P] [F] [Q]        ".to_string(),
                    "[B] [W]     [W] [M] [S] [B]        ".to_string(),
                    "[F] [B]     [C] [S]     [W]        ".to_string(),
                    "[C]         [S] [H]                ".to_string()
                ]
            },
            crate::input::DAY5.parse().unwrap()
        )
    }

    #[test]
    fn iterate_crates() {
        let mut stacks: Stacks = crate::input::DAY5.parse().unwrap();
        assert_eq!(
            Crates {
                crates: vec![
                    "S".to_string(),
                    "Z".to_string(),
                    "P".to_string(),
                    "D".to_string(),
                    "L".to_string(),
                    "B".to_string(),
                    "F".to_string(),
                    "C".to_string()
                ]
            },
            stacks.next().unwrap()
        );
        assert_eq!(
            Crates {
                crates: vec![
                    "N".to_string(),
                    "V".to_string(),
                    "G".to_string(),
                    "P".to_string(),
                    "H".to_string(),
                    "W".to_string(),
                    "B".to_string()
                ]
            },
            stacks.next().unwrap()
        );
        assert_eq!(
            Crates {
                crates: vec![
                    "F".to_string(),
                    "W".to_string(),
                    "B".to_string(),
                    "J".to_string(),
                    "G".to_string()
                ]
            },
            stacks.next().unwrap()
        );
        let stacks: Stacks = crate::input::DAY5.parse().unwrap();
        assert_eq!(
            Crates {
                crates: vec!["N".to_string(), "C".to_string(), "R".to_string()]
            },
            stacks.skip(8).next().unwrap()
        );
    }
}
