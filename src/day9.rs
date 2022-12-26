#[derive(Debug, PartialEq, Eq, Default, Clone)]
struct Position {
    top: isize,
    left: isize,
}

#[derive(Debug, PartialEq, Eq)]
enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, PartialEq, Eq)]
struct Rope {
    head: Position,
    tail: Vec<Position>,
}

impl Default for Rope {
    fn default() -> Self {
        Self {
            head: Default::default(),
            tail: vec![Default::default()],
        }
    }
}

fn move_head(mut rope: Rope, movement: Movement) -> Rope {
    match movement {
        Movement::Up(steps) => {
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
        Movement::Down(steps) => {
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
        Movement::Left(steps) => {
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
        Movement::Right(steps) => {
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
    fn moving() {
        let mut rope: Rope = Default::default();
        rope = move_head(rope, Movement::Up(1));
        assert_eq!(
            Rope {
                head: Position { top: 1, left: 0 },
                tail: vec![Position { top: 0, left: 0 }]
            },
            rope
        );
        rope = move_head(rope, Movement::Up(1));
        assert_eq!(
            Rope {
                head: Position { top: 2, left: 0 },
                tail: vec![Position { top: 0, left: 0 }, Position { top: 1, left: 0 }]
            },
            rope
        );
        rope = move_head(rope, Movement::Up(1));
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
        rope = move_head(rope, Movement::Up(1));
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
        rope = move_head(rope, Movement::Up(2));
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
        rope = move_head(rope, Movement::Down(1));
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
        rope = move_head(rope, Movement::Down(1));
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
        rope = move_head(rope, Movement::Down(1));
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
        rope = move_head(rope, Movement::Down(1));
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
        rope = move_head(rope, Movement::Down(3));
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
        rope = move_head(rope, Movement::Left(1));
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
        rope = move_head(rope, Movement::Left(1));
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
        rope = move_head(rope, Movement::Left(1));
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
        rope = move_head(rope, Movement::Left(3));
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
        rope = move_head(rope, Movement::Right(1));
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
        rope = move_head(rope, Movement::Right(1));
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
        rope = move_head(rope, Movement::Right(4));
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
        rope = move_head(rope, Movement::Up(1));
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
        rope = move_head(rope, Movement::Up(1));
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
