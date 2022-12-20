use std::str::FromStr;

use crate::error::Ooops;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AssignmentRange {
    start: usize,
    end: usize,
}

impl AssignmentRange {
    pub(crate) fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub(crate) fn overlaps(&self, other: &Self) -> bool {
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
        let map_err = |e| Ooops(format!("{}", e));
        match (values.next(), values.next()) {
            (Some(start), Some(end)) => Ok(AssignmentRange {
                start: start.parse().map_err(map_err)?,
                end: end.parse().map_err(map_err)?,
            }),
            (_, _) => Err(Ooops(format!("bad range {}", s))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AssignmentPair {
    pub(crate) a: AssignmentRange,
    pub(crate) b: AssignmentRange,
}

impl FromStr for AssignmentPair {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(',');
        let values = values.collect::<Vec<&str>>();
        let mut values = values.iter();
        match (values.next(), values.next()) {
            (Some(a), Some(b)) => Ok(Self {
                a: a.parse()?,
                b: b.parse()?,
            }),
            (_, _) => Err(Ooops(format!("bad pair {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
