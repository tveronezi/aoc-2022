use std::str::FromStr;

use crate::error::Ooops;

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
            Self::Winner => 6,
            Self::Draw => 3,
            Self::Loser => 0,
        }
    }

    fn hand(&self, against: &Hand) -> Hand {
        match (self, against) {
            (Self::Winner, Hand::Rock) => Hand::Paper,
            (Self::Winner, Hand::Paper) => Hand::Scisor,
            (Self::Winner, Hand::Scisor) => Hand::Rock,
            (Self::Loser, Hand::Rock) => Hand::Scisor,
            (Self::Loser, Hand::Paper) => Hand::Rock,
            (Self::Loser, Hand::Scisor) => Hand::Paper,
            (Self::Draw, Hand::Rock) => Hand::Rock,
            (Self::Draw, Hand::Paper) => Hand::Paper,
            (Self::Draw, Hand::Scisor) => Hand::Scisor,
        }
    }
}

impl Hand {
    fn fight(&self, other: &Hand) -> RpsMatchResult {
        match (self, other) {
            (Self::Rock, Self::Rock) => RpsMatchResult::Draw,
            (Self::Paper, Self::Paper) => RpsMatchResult::Draw,
            (Self::Scisor, Self::Scisor) => RpsMatchResult::Draw,
            (Self::Rock, Self::Paper) => RpsMatchResult::Loser,
            (Self::Rock, Self::Scisor) => RpsMatchResult::Winner,
            (Self::Paper, Self::Rock) => RpsMatchResult::Winner,
            (Self::Paper, Self::Scisor) => RpsMatchResult::Loser,
            (Self::Scisor, Self::Rock) => RpsMatchResult::Loser,
            (Self::Scisor, Self::Paper) => RpsMatchResult::Winner,
        }
    }

    fn weight(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scisor => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct RpsMatch {
    mine: Hand,
    opponent: Hand,
}

impl RpsMatch {
    pub(crate) fn play(self) -> usize {
        self.mine.weight() + self.mine.fight(&self.opponent).value()
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CheatRpsMatch {
    opponent: Hand,
    result: RpsMatchResult,
}

impl CheatRpsMatch {
    pub(crate) fn play(self) -> usize {
        let my_hand = self.result.hand(&self.opponent);
        my_hand.weight() + self.result.value()
    }
}

impl FromStr for RpsMatchResult {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Loser),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Winner),
            _ => Err(Ooops(format!("[a] invalid s='{}'", s))),
        }
    }
}

impl FromStr for Hand {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scisor),
            _ => Err(Ooops(format!("[b] invalid s='{}'", s))),
        }
    }
}

impl FromStr for RpsMatch {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(' ').take(2);
        match (values.next(), values.next()) {
            (Some(opponents), Some(mine)) => Ok(Self {
                mine: mine.parse()?,
                opponent: opponents.parse()?,
            }),
            (_, _) => Err(Ooops(format!("[c] missing value '{}'", s))),
        }
    }
}

impl FromStr for CheatRpsMatch {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(' ').take(2);
        match (values.next(), values.next()) {
            (Some(opponents), Some(result)) => Ok(Self {
                opponent: opponents.parse()?,
                result: result.parse()?,
            }),
            (_, _) => Err(Ooops(format!("[e] missing value '{}'", s))),
        }
    }
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
        assert_eq!(4, "A Y".parse::<CheatRpsMatch>().unwrap().play());
        assert_eq!(1, "B X".parse::<CheatRpsMatch>().unwrap().play());
        assert_eq!(7, "C Z".parse::<CheatRpsMatch>().unwrap().play());
    }
}
