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
    nodes: Vec<Position>,
    pub(crate) tail_positions: Vec<Position>,
}

impl Default for Rope {
    fn default() -> Self {
        let init: Position = Default::default();
        Self {
            nodes: vec![init.clone(), init.clone()],
            tail_positions: vec![init],
        }
    }
}

impl Rope {
    pub(crate) fn new(size: usize) -> Self {
        let init: Position = Default::default();
        Self {
            nodes: (0..size).map(|_| init.clone()).collect(),
            tail_positions: vec![init],
        }
    }
}

pub(crate) fn move_head(mut rope: Rope, motion: Motion) -> Rope {
    match motion {
        Motion::Up(steps) => {
            for _ in 0..steps {
                let mut previous = rope
                    .nodes
                    .first()
                    .expect("this list starts with one element")
                    .clone();
                previous.top += 1;
                let mut new_nodes = vec![previous.clone()];
                for current in rope.nodes.iter().skip(1) {
                    if previous.top - current.top > 1 {
                        new_nodes.push(Position {
                            top: previous.top - 1,
                            left: previous.left,
                        });
                    } else {
                        new_nodes.push(current.clone());
                    }
                    previous = current.clone();
                    previous.top += 1;
                }
                let last = new_nodes.last().expect("the rope has always a tail");
                if rope
                    .tail_positions
                    .last()
                    .expect("the tail has always one last position")
                    != last
                {
                    rope.tail_positions.push(last.clone())
                }
                rope.nodes = new_nodes;
            }
        }
        Motion::Down(steps) => {
            for _ in 0..steps {
                let mut previous = rope
                    .nodes
                    .first()
                    .expect("this list starts with one element")
                    .clone();
                previous.top -= 1;
                let mut new_nodes = vec![previous.clone()];
                for current in rope.nodes.iter().skip(1) {
                    if current.top - previous.top > 1 {
                        new_nodes.push(Position {
                            top: previous.top + 1,
                            left: previous.left,
                        });
                    } else {
                        new_nodes.push(current.clone());
                    }
                    previous = current.clone();
                    previous.top -= 1;
                }
                let last = new_nodes.last().expect("the rope has always a tail");
                if rope
                    .tail_positions
                    .last()
                    .expect("the tail has always one last position")
                    != last
                {
                    rope.tail_positions.push(last.clone())
                }
                rope.nodes = new_nodes;
            }
        }
        Motion::Left(steps) => {
            for _ in 0..steps {
                let mut previous = rope
                    .nodes
                    .first()
                    .expect("this list starts with one element")
                    .clone();
                previous.left -= 1;
                let mut new_nodes = vec![previous.clone()];
                for current in rope.nodes.iter_mut().skip(1) {
                    if current.left - previous.left > 1 {
                        new_nodes.push(Position {
                            top: previous.top,
                            left: previous.left + 1,
                        });
                    } else {
                        new_nodes.push(current.clone());
                    }
                    previous = current.clone();
                    previous.left -= 1;
                }
                let last = new_nodes.last().expect("the rope has always a tail");
                if rope
                    .tail_positions
                    .last()
                    .expect("the tail has always one last position")
                    != last
                {
                    rope.tail_positions.push(last.clone())
                }
                rope.nodes = new_nodes;
            }
        }
        Motion::Right(steps) => {
            for _ in 0..steps {
                let mut previous = rope
                    .nodes
                    .first()
                    .expect("this list starts with one element")
                    .clone();
                previous.left += 1;
                let mut new_nodes = vec![previous.clone()];
                for current in rope.nodes.iter_mut().skip(1) {
                    if previous.left - current.left > 1 {
                        new_nodes.push(Position {
                            top: previous.top,
                            left: previous.left - 1,
                        });
                    } else {
                        new_nodes.push(current.clone());
                    }
                    previous = current.clone();
                    previous.left += 1;
                }
                let last = new_nodes.last().expect("the rope has always a tail");
                if rope
                    .tail_positions
                    .last()
                    .expect("the tail has always one last position")
                    != last
                {
                    rope.tail_positions.push(last.clone())
                }
                rope.nodes = new_nodes;
            }
        }
    }
    rope
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn b_init_len() {
        let rope = Rope::new(11);
        assert_eq!(11, rope.nodes.len())
    }

    #[test]
    fn b_r4() {
        let rope = move_head(Rope::new(11), Motion::Right(4));
        let expected: Vec<Position> = vec![
            Position { top: 0, left: 4 }, // H
            Position { top: 0, left: 3 }, // 1
            Position { top: 0, left: 2 }, // 2
            Position { top: 0, left: 1 }, // 3
            Position { top: 0, left: 0 }, // 4
            Position { top: 0, left: 0 }, // 5
            Position { top: 0, left: 0 }, // 6
            Position { top: 0, left: 0 }, // 7
            Position { top: 0, left: 0 }, // 8
            Position { top: 0, left: 0 }, // 9
            Position { top: 0, left: 0 }, // s
        ];
        assert_eq!(expected, rope.nodes)
    }

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
    fn moving_sized() {
        let mut rope = Rope::new(10);
        rope = move_head(rope, Motion::Right(5));
        assert_eq!(
            Rope {
                nodes: vec![
                    Position { top: 0, left: 5 },
                    Position { top: 0, left: 4 },
                    Position { top: 0, left: 3 },
                    Position { top: 0, left: 2 },
                    Position { top: 0, left: 1 },
                    Position { top: 0, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: 0, left: 0 },
                    Position { top: 0, left: 0 }
                ],
                tail_positions: vec![Position { top: 0, left: 0 }]
            },
            rope
        );
        rope = move_head(rope, Motion::Up(8));
        assert_eq!(
            Rope {
                nodes: vec![
                    Position { top: 8, left: 5 },
                    Position { top: 7, left: 5 },
                    Position { top: 6, left: 5 },
                    Position { top: 5, left: 5 },
                    Position { top: 4, left: 5 },
                    Position { top: 3, left: 5 },
                    Position { top: 2, left: 5 },
                    Position { top: 1, left: 5 },
                    Position { top: 0, left: 0 },
                    Position { top: 0, left: 0 }
                ],
                tail_positions: vec![Position { top: 0, left: 0 }]
            },
            rope
        );
    }

    #[test]
    fn moving() {
        let mut rope: Rope = Default::default();
        rope = move_head(rope, Motion::Up(1));
        assert_eq!(
            Rope {
                nodes: vec![Position { top: 1, left: 0 }, Position { top: 0, left: 0 }],
                tail_positions: vec![Position { top: 0, left: 0 }]
            },
            rope
        );
        rope = move_head(rope, Motion::Up(1));
        assert_eq!(
            Rope {
                nodes: vec![Position { top: 2, left: 0 }, Position { top: 1, left: 0 }],
                tail_positions: vec![Position { top: 0, left: 0 }, Position { top: 1, left: 0 }]
            },
            rope
        );
        rope = move_head(rope, Motion::Up(1));
        assert_eq!(
            Rope {
                nodes: vec![Position { top: 3, left: 0 }, Position { top: 2, left: 0 }],
                tail_positions: vec![
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
                nodes: vec![Position { top: 4, left: 0 }, Position { top: 3, left: 0 }],
                tail_positions: vec![
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
                nodes: vec![Position { top: 6, left: 0 }, Position { top: 5, left: 0 }],
                tail_positions: vec![
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
                nodes: vec![Position { top: 5, left: 0 }, Position { top: 5, left: 0 }],
                tail_positions: vec![
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
                nodes: vec![Position { top: 4, left: 0 }, Position { top: 5, left: 0 }],
                tail_positions: vec![
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
                nodes: vec![Position { top: 3, left: 0 }, Position { top: 4, left: 0 }],
                tail_positions: vec![
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
                nodes: vec![Position { top: 2, left: 0 }, Position { top: 3, left: 0 }],
                tail_positions: vec![
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
                nodes: vec![Position { top: -1, left: 0 }, Position { top: 0, left: 0 }],
                tail_positions: vec![
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
                nodes: vec![Position { top: -1, left: -1 }, Position { top: 0, left: 0 }],
                tail_positions: vec![
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
                nodes: vec![
                    Position { top: -1, left: -2 },
                    Position { top: -1, left: -1 }
                ],
                tail_positions: vec![
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
                nodes: vec![
                    Position { top: -1, left: -3 },
                    Position { top: -1, left: -2 }
                ],
                tail_positions: vec![
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
                nodes: vec![
                    Position { top: -1, left: -6 },
                    Position { top: -1, left: -5 }
                ],
                tail_positions: vec![
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
                nodes: vec![
                    Position { top: -1, left: -5 },
                    Position { top: -1, left: -5 }
                ],
                tail_positions: vec![
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
                nodes: vec![
                    Position { top: -1, left: -4 },
                    Position { top: -1, left: -5 }
                ],
                tail_positions: vec![
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
                nodes: vec![
                    Position { top: -1, left: 0 },
                    Position { top: -1, left: -1 }
                ],
                tail_positions: vec![
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
                nodes: vec![Position { top: 0, left: 0 }, Position { top: -1, left: -1 }],
                tail_positions: vec![
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
                nodes: vec![Position { top: 1, left: 0 }, Position { top: 0, left: 0 }],
                tail_positions: vec![
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
