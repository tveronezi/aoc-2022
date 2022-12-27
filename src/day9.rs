use std::str::FromStr;

use crate::error::Ooops;

#[derive(Debug, PartialEq, Eq, Default, Clone, Hash)]
pub(crate) struct Position {
    top: isize,
    left: isize,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Motion {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl FromStr for Motion {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        match (split.next(), split.next()) {
            (Some(direction), Some(value)) if direction == "U" && value.parse::<u32>().is_ok() => {
                Ok(Motion::Up(value.parse::<u32>().unwrap()))
            }
            (Some(direction), Some(value)) if direction == "D" && value.parse::<u32>().is_ok() => {
                Ok(Motion::Down(value.parse::<u32>().unwrap()))
            }
            (Some(direction), Some(value)) if direction == "L" && value.parse::<u32>().is_ok() => {
                Ok(Motion::Left(value.parse::<u32>().unwrap()))
            }
            (Some(direction), Some(value)) if direction == "R" && value.parse::<u32>().is_ok() => {
                Ok(Motion::Right(value.parse::<u32>().unwrap()))
            }
            _ => Err(Ooops(format!("invalid movement '{}'", s))),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Rope {
    head: Position,
    pub(crate) tail: Vec<Position>,
}

impl Default for Rope {
    fn default() -> Self {
        Self {
            head: Default::default(),
            tail: vec![Default::default()],
        }
    }
}

pub(crate) fn move_head(mut rope: Rope, motion: Motion) -> Rope {
    match motion {
        Motion::Up(steps) => {
            for _ in 0..steps {
                rope.head.top += 1;
                let tail = rope.tail.last().expect("this list starts with one element");
                if rope.head.top - tail.top > 1 {
                    rope.tail.push(Position {
                        left: rope.head.left,
                        top: rope.head.top - 1,
                    });
                }
            }
        }
        Motion::Down(steps) => {
            for _ in 0..steps {
                rope.head.top -= 1;
                let tail = rope.tail.last().expect("this list starts with one element");
                if tail.top - rope.head.top > 1 {
                    rope.tail.push(Position {
                        left: rope.head.left,
                        top: rope.head.top + 1,
                    });
                }
            }
        }
        Motion::Left(steps) => {
            for _ in 0..steps {
                rope.head.left -= 1;
                let tail = rope.tail.last().expect("this list starts with one element");
                if tail.left - rope.head.left > 1 {
                    rope.tail.push(Position {
                        left: rope.head.left + 1,
                        top: rope.head.top,
                    });
                }
            }
        }
        Motion::Right(steps) => {
            for _ in 0..steps {
                rope.head.left += 1;
                let tail = rope.tail.last().expect("this list starts with one element");
                if rope.head.left - tail.left > 1 {
                    rope.tail.push(Position {
                        left: rope.head.left - 1,
                        top: rope.head.top,
                    });
                }
            }
        }
    }
    rope
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(Ok(Motion::Down(2)), "D 2".parse());
        assert_eq!(Ok(Motion::Up(2)), "U 2".parse());
        assert_eq!(Ok(Motion::Left(2)), "L 2".parse());
        assert_eq!(Ok(Motion::Right(2)), "R 2".parse());
        assert_eq!(
            Err(Ooops("invalid movement 'banana'".to_string())),
            "banana".parse::<Motion>()
        );
    }

    #[test]
    fn moving() {
        let mut rope: Rope = Default::default();
        rope = move_head(rope, Motion::Up(1));
        assert_eq!(
            Rope {
                head: Position { top: 1, left: 0 },
                tail: vec![Position { top: 0, left: 0 }]
            },
            rope
        );
        rope = move_head(rope, Motion::Up(1));
        assert_eq!(
            Rope {
                head: Position { top: 2, left: 0 },
                tail: vec![Position { top: 0, left: 0 }, Position { top: 1, left: 0 }]
            },
            rope
        );
        rope = move_head(rope, Motion::Up(1));
        assert_eq!(
            Rope {
                head: Position { top: 3, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Up(1));
        assert_eq!(
            Rope {
                head: Position { top: 4, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Up(2));
        assert_eq!(
            Rope {
                head: Position { top: 6, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Down(1));
        assert_eq!(
            Rope {
                head: Position { top: 5, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Down(1));
        assert_eq!(
            Rope {
                head: Position { top: 4, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Down(1));
        assert_eq!(
            Rope {
                head: Position { top: 3, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Down(1));
        assert_eq!(
            Rope {
                head: Position { top: 2, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Down(3));
        assert_eq!(
            Rope {
                head: Position { top: -1, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Left(1));
        assert_eq!(
            Rope {
                head: Position { top: -1, left: -1 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Left(1));
        assert_eq!(
            Rope {
                head: Position { top: -1, left: -2 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: -1, left: -1 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Left(1));
        assert_eq!(
            Rope {
                head: Position { top: -1, left: -3 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: -1, left: -1 },
                    Position { top: -1, left: -2 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Left(3));
        assert_eq!(
            Rope {
                head: Position { top: -1, left: -6 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: -1, left: -1 },
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -5 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Right(1));
        assert_eq!(
            Rope {
                head: Position { top: -1, left: -5 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: -1, left: -1 },
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -5 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Right(1));
        assert_eq!(
            Rope {
                head: Position { top: -1, left: -4 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: -1, left: -1 },
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -5 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Right(4));
        assert_eq!(
            Rope {
                head: Position { top: -1, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: -1, left: -1 },
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -5 },
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -1 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Up(1));
        assert_eq!(
            Rope {
                head: Position { top: 0, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: -1, left: -1 },
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -5 },
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -1 }
                ]
            },
            rope
        );
        rope = move_head(rope, Motion::Up(1));
        assert_eq!(
            Rope {
                head: Position { top: 1, left: 0 },
                tail: vec![
                    Position { top: 0, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 5, left: 0 },
                    Position { top: 4, left: 0 },
                    Position { top: 3, left: 0 },
                    Position { top: 2, left: 0 },
                    Position { top: 1, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: -1, left: -1 },
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -5 },
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -1 },
                    Position { top: 0, left: 0 }
                ]
            },
            rope
        );
    }
}
