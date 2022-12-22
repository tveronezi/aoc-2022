#[derive(Debug, PartialEq, Eq)]
struct Tree {
    top: usize,
    left: usize,
    height: usize,
}

struct Trees {
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

#[cfg(test)]
mod tests {
    use super::*;

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