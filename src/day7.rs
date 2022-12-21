use std::str::FromStr;

use crate::error::Ooops;

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Cd(String),
    Ls,
    Dir(String),
    File { size: usize, name: String },
}

impl FromStr for Line {
    type Err = Ooops;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        match (split.next(), split.next(), split.next()) {
            (Some(prompt), Some(command), Some(param)) if prompt == "$" && command == "cd" => {
                Ok(Line::Cd(param.to_string()))
            }
            (Some(prompt), Some(command), None) if prompt == "$" && command == "ls" => Ok(Line::Ls),
            (Some(dir), Some(name), None) if dir == "dir" => Ok(Line::Dir(name.to_string())),
            (Some(size), Some(name), None) if size.parse::<usize>().is_ok() => Ok(Line::File {
                name: name.to_string(),
                size: size.parse::<usize>().unwrap(),
            }),
            _ => Err(Ooops(format!("invalid line '{}'", s))),
        }
    }
}

enum Item {
    Directory { name: String, children: Vec<Item> },
    File { size: usize, name: String },
}

impl Item {
    fn size(&self) -> usize {
        match self {
            Item::Directory { name: _, children } => children
                .iter()
                .map(|c| match c {
                    Item::Directory {
                        name: _,
                        children: _,
                    } => c.size(),
                    Item::File { size, name: _ } => *size,
                })
                .sum(),
            Item::File { size, name: _ } => *size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_size() {
        assert_eq!(
            0,
            Item::Directory {
                name: "/".to_string(),
                children: vec![]
            }
            .size()
        );
        assert_eq!(
            2000,
            Item::Directory {
                name: "/".to_string(),
                children: vec![Item::Directory {
                    name: "a".to_string(),
                    children: vec![Item::File {
                        size: 2000,
                        name: "my_file".to_string()
                    }]
                }]
            }
            .size()
        );
        assert_eq!(
            2000,
            Item::Directory {
                name: "/".to_string(),
                children: vec![Item::Directory {
                    name: "a".to_string(),
                    children: vec![
                        Item::File {
                            size: 1000,
                            name: "my_file".to_string()
                        },
                        Item::File {
                            size: 1000,
                            name: "my_file_2".to_string()
                        }
                    ]
                }]
            }
            .size()
        );
        assert_eq!(
            2000,
            Item::Directory {
                name: "/".to_string(),
                children: vec![Item::Directory {
                    name: "a".to_string(),
                    children: vec![
                        Item::File {
                            size: 1000,
                            name: "my_file".to_string()
                        },
                        Item::Directory {
                            name: "b".to_string(),
                            children: vec![Item::File {
                                size: 1000,
                                name: "my_file_2".to_string()
                            }]
                        }
                    ]
                }]
            }
            .size()
        );
    }

    #[test]
    fn parse_lines() {
        assert_eq!(Line::Cd("/".to_string()), "$ cd /".parse::<Line>().unwrap());
        assert_eq!(Line::Ls, "$ ls".parse::<Line>().unwrap());
        assert_eq!(
            Line::Dir("dtcfhsm".to_string()),
            "dir dtcfhsm".parse::<Line>().unwrap()
        );
        assert_eq!(
            Line::File {
                name: "jrfpjdpw.znd".to_string(),
                size: 35442
            },
            "35442 jrfpjdpw.znd".parse::<Line>().unwrap()
        );
        assert_eq!(
            Err(Ooops("invalid line 'banana'".to_string())),
            "banana".parse::<Line>()
        );
    }
}
