#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub(crate) struct Tree {
    top: usize,
    left: usize,
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

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum VerticalTreesPosition {
    Top,
    Bottom,
}

impl Default for VerticalTreesPosition {
    fn default() -> Self {
        Self::Top
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) struct VerticalTrees {
    pub(crate) field: String,
    pub(crate) tree: Tree,
    pub(crate) top: usize,
    pub(crate) position: VerticalTreesPosition,
}

impl Iterator for VerticalTrees {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == VerticalTreesPosition::Top && self.top >= self.tree.top {
            return None;
        }
        if self.position == VerticalTreesPosition::Bottom && self.top <= self.tree.top {
            self.top = self.tree.top + 1;
        }
        if self.top == self.tree.top {
            self.top += 1;
        }
        let line = self.field.lines().nth(self.top);
        line?;
        let line = line.expect("just checked this line exists");
        let neighbour_height = line.chars().nth(self.tree.left);
        neighbour_height?;
        let neighbour_height = neighbour_height.expect("just checked this height exists");
        let top = self.top;
        self.top += 1;
        Some(Tree {
            top,
            left: self.tree.left,
            height: neighbour_height.to_digit(10).unwrap_or(0) as usize,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum HorizontalTreesPosition {
    Left,
    Right,
}

impl Default for HorizontalTreesPosition {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub(crate) struct HorizontalTrees {
    pub(crate) field: String,
    pub(crate) tree: Tree,
    pub(crate) left: usize,
    pub(crate) position: HorizontalTreesPosition,
}

impl Iterator for HorizontalTrees {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position == HorizontalTreesPosition::Left && self.left >= self.tree.left {
            return None;
        }
        if self.position == HorizontalTreesPosition::Right && self.left <= self.tree.left {
            self.left = self.tree.left + 1;
        }
        if self.left == self.tree.left {
            self.left += 1;
        }
        let line = self.field.lines().nth(self.tree.top);
        line?;
        let line = line.expect("just checked this line exists");
        let neighbour_height = line.chars().nth(self.left);
        neighbour_height?;
        let neighbour_height = neighbour_height.expect("just checked this height exists");
        let left = self.left;
        self.left += 1;
        Some(Tree {
            top: self.tree.top,
            left,
            height: neighbour_height.to_digit(10).unwrap_or(0) as usize,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_horizontal_left_trees() {
        let mut trees = HorizontalTrees {
            field: crate::input::DAY8.to_string(),
            tree: Tree {
                top: 1,
                left: 1,
                height: 3,
            },
            position: HorizontalTreesPosition::Left,
            ..Default::default()
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
    fn iterate_horizontal_right_trees() {
        let mut trees = HorizontalTrees {
            field: crate::input::DAY8.to_string(),
            tree: Tree {
                top: 1,
                left: 1,
                height: 3,
            },
            position: HorizontalTreesPosition::Right,
            ..Default::default()
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
        let mut vertical_trees = VerticalTrees {
            field: crate::input::DAY8.to_string(),
            tree: Tree {
                top: 1,
                left: 1,
                height: 3,
            },
            position: VerticalTreesPosition::Top,
            ..Default::default()
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
    fn iterate_vertical_bottom_trees() {
        let mut vertical_trees = VerticalTrees {
            field: crate::input::DAY8.to_string(),
            tree: Tree {
                top: 1,
                left: 1,
                height: 3,
            },
            position: VerticalTreesPosition::Bottom,
            ..Default::default()
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
