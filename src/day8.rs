#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub(crate) struct Tree {
    pub(crate) top: usize,
    pub(crate) left: usize,
    pub(crate) height: usize,
}

#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) struct Trees {
    field: String,
    top: usize,
    left: usize,
}

impl From<&str> for Trees {
    fn from(s: &str) -> Self {
        Self {
            field: s.to_string(),
            top: 0,
            left: 0,
        }
    }
}

impl Iterator for Trees {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.field.lines().count() < self.top + 1 {
                return None;
            }
            let line = self.field.lines().nth(self.top);
            if line.is_none() {
                self.top += 1;
                continue;
            }
            let line = line.expect("just checked this line exists");
            let height = line.chars().nth(self.left);
            if height.is_none() {
                self.left = 0;
                self.top += 1;
                continue;
            }
            let height = height.expect("just checked this height exists");
            let top = self.top;
            let left = self.left;
            self.left += 1;
            return Some(Tree {
                top,
                left,
                height: height.to_digit(10).unwrap_or(0) as usize,
            });
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) struct TopTrees {
    pub(crate) field: String,
    pub(crate) from_top: usize,
    pub(crate) from_left: usize,
}

impl Iterator for TopTrees {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if self.from_top < 1 {
            return None;
        }
        let line = self.field.lines().nth(self.from_top - 1);
        line?;
        self.from_top -= 1;
        let line = line.expect("just checked this line exists");
        let neighbour_height = line.chars().nth(self.from_left);
        neighbour_height?;
        let neighbour_height = neighbour_height.expect("just checked this height exists");
        let top = self.from_top;
        Some(Tree {
            top,
            left: self.from_left,
            height: neighbour_height.to_digit(10).unwrap_or(0) as usize,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) struct BottomTrees {
    pub(crate) field: String,
    pub(crate) from_top: usize,
    pub(crate) from_left: usize,
}

impl Iterator for BottomTrees {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        self.from_top += 1;
        let line = self.field.lines().nth(self.from_top);
        line?;
        let line = line.expect("just checked this line exists");
        let neighbour_height = line.chars().nth(self.from_left);
        neighbour_height?;
        let neighbour_height = neighbour_height.expect("just checked this height exists");
        Some(Tree {
            top: self.from_top,
            left: self.from_left,
            height: neighbour_height.to_digit(10).unwrap_or(0) as usize,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) struct LeftTrees {
    pub(crate) field: String,
    pub(crate) from_left: usize,
    pub(crate) from_top: usize,
}

impl Iterator for LeftTrees {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if self.from_left < 1 {
            return None;
        }
        self.from_left -= 1;
        let line = self.field.lines().nth(self.from_top);
        line?;
        let line = line.expect("just checked this line exists");
        let neighbour_height = line.chars().nth(self.from_left);
        neighbour_height?;
        let neighbour_height = neighbour_height.expect("just checked this height exists");
        Some(Tree {
            top: self.from_top,
            left: self.from_left,
            height: neighbour_height.to_digit(10).unwrap_or(0) as usize,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) struct RightTrees {
    pub(crate) field: String,
    pub(crate) from_left: usize,
    pub(crate) from_top: usize,
}

impl Iterator for RightTrees {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.field.lines().nth(self.from_top);
        line?;
        let line = line.expect("just checked this line exists");
        self.from_left += 1;
        let neighbour_height = line.chars().nth(self.from_left);
        neighbour_height?;
        let neighbour_height = neighbour_height.expect("just checked this height exists");
        Some(Tree {
            top: self.from_top,
            left: self.from_left,
            height: neighbour_height.to_digit(10).unwrap_or(0) as usize,
        })
    }
}

pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn calculate_viewing_distance<I: Iterator<Item = Tree>>(trees: I, tree: &Tree) -> usize {
    let mut count = 0;
    for t in trees {
        count += 1;
        if t.height >= tree.height {
            break;
        }
    }
    count
}

pub(crate) fn viewing_distance(field: String, tree: &Tree, direction: Direction) -> usize {
    match direction {
        Direction::Up => calculate_viewing_distance(
            TopTrees {
                field,
                from_left: tree.left,
                from_top: tree.top,
            },
            tree,
        ),
        Direction::Down => calculate_viewing_distance(
            BottomTrees {
                field,
                from_left: tree.left,
                from_top: tree.top,
            },
            tree,
        ),
        Direction::Left => calculate_viewing_distance(
            LeftTrees {
                field,
                from_left: tree.left,
                from_top: tree.top,
            },
            tree,
        ),
        Direction::Right => calculate_viewing_distance(
            RightTrees {
                field,
                from_left: tree.left,
                from_top: tree.top,
            },
            tree,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewing_distance() {
        let field = vec!["30373", "25512", "65332", "33549", "35390"].join("\n");
        let tree = Tree {
            top: 1,
            left: 2,
            height: 5,
        };
        assert_eq!(1, viewing_distance(field.clone(), &tree, Direction::Up));
        assert_eq!(1, viewing_distance(field.clone(), &tree, Direction::Left));
        assert_eq!(2, viewing_distance(field.clone(), &tree, Direction::Right));
        assert_eq!(2, viewing_distance(field, &tree, Direction::Down));
    }

    #[test]
    fn test_viewing_distance_better() {
        let field = vec!["30373", "25512", "65332", "33549", "35390"].join("\n");
        let tree = Tree {
            top: 3,
            left: 2,
            height: 5,
        };
        assert_eq!(2, viewing_distance(field.clone(), &tree, Direction::Up));
        assert_eq!(2, viewing_distance(field.clone(), &tree, Direction::Left));
        assert_eq!(1, viewing_distance(field.clone(), &tree, Direction::Down));
        assert_eq!(2, viewing_distance(field, &tree, Direction::Right));
    }

    #[test]
    fn iterate_horizontal_left_trees() {
        let mut trees = LeftTrees {
            field: crate::input::DAY8.to_string(),
            from_top: 1,
            from_left: 1,
        };
        assert_eq!(
            Some(Tree {
                height: 3,
                left: 0,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(None, trees.next());
    }

    #[test]
    fn iterate_horizontal_left_trees_two_levels_down() {
        let mut trees = LeftTrees {
            field: crate::input::DAY8.to_string(),
            from_top: 1,
            from_left: 5,
        };
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 4,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 3,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 3,
                left: 2,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 3,
                left: 1,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 3,
                left: 0,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(None, trees.next());
    }

    #[test]
    fn iterate_horizontal_right_trees() {
        let mut trees = RightTrees {
            field: crate::input::DAY8.to_string(),
            from_top: 1,
            from_left: 1,
        };
        assert_eq!(
            Some(Tree {
                height: 3,
                left: 2,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 3,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 4,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 1,
                left: 5,
                top: 1
            }),
            trees.next()
        );
    }

    #[test]
    fn iterate_vertical_top_trees() {
        let mut vertical_trees = TopTrees {
            field: crate::input::DAY8.to_string(),
            from_left: 1,
            from_top: 1,
        };
        assert_eq!(
            Some(Tree {
                height: 2,
                left: 1,
                top: 0
            }),
            vertical_trees.next()
        );
        assert_eq!(None, vertical_trees.next());
    }

    #[test]
    fn iterate_vertical_top_trees_two_levels_down() {
        let mut vertical_trees = TopTrees {
            field: crate::input::DAY8.to_string(),
            from_left: 1,
            from_top: 4,
        };
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 1,
                top: 3
            }),
            vertical_trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 1,
                top: 2
            }),
            vertical_trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 3,
                left: 1,
                top: 1
            }),
            vertical_trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 2,
                left: 1,
                top: 0
            }),
            vertical_trees.next()
        );
        assert_eq!(None, vertical_trees.next());
    }

    #[test]
    fn iterate_vertical_bottom_trees() {
        let mut vertical_trees = BottomTrees {
            field: crate::input::DAY8.to_string(),
            from_left: 1,
            from_top: 1,
        };
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 1,
                top: 2
            }),
            vertical_trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 1,
                top: 3
            }),
            vertical_trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 1,
                top: 4
            }),
            vertical_trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 2,
                left: 1,
                top: 5
            }),
            vertical_trees.next()
        );
    }

    #[test]
    fn iterate_trees() {
        let mut trees: Trees = "0233\n4177".into();
        assert_eq!(
            Some(Tree {
                height: 0,
                left: 0,
                top: 0
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 2,
                left: 1,
                top: 0
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 3,
                left: 2,
                top: 0
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 3,
                left: 3,
                top: 0
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 4,
                left: 0,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 1,
                left: 1,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 7,
                left: 2,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(
            Some(Tree {
                height: 7,
                left: 3,
                top: 1
            }),
            trees.next()
        );
        assert_eq!(None, trees.next());
    }
}
