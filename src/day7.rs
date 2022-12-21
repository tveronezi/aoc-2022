use std::{rc::Rc, str::FromStr};

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

trait FsItem {
    fn size(&self) -> usize;
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FsDirectory {
    name: String,
    files: Vec<FsFile>,
    directories: Vec<Rc<FsDirectory>>,
}

impl FsItem for FsDirectory {
    fn size(&self) -> usize {
        let files: usize = self.files.iter().map(|c| c.size).sum();
        let directories: usize = self.directories.iter().map(|c| c.size()).sum();
        files + directories
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FsFile {
    size: usize,
    name: String,
}

impl FsItem for FsFile {
    fn size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_size() {
        assert_eq!(
            0,
            FsDirectory {
                name: "/".to_string(),
                files: vec![],
                directories: vec![]
            }
            .size()
        );
        assert_eq!(
            10,
            FsDirectory {
                name: "/".to_string(),
                files: vec![FsFile {
                    name: "a".to_string(),
                    size: 10
                }],
                directories: vec![]
            }
            .size()
        );
        assert_eq!(
            20,
            FsDirectory {
                name: "/".to_string(),
                files: vec![FsFile {
                    name: "a".to_string(),
                    size: 10
                }],
                directories: vec![Rc::new(FsDirectory {
                    name: "b".to_string(),
                    directories: vec![],
                    files: vec![FsFile {
                        name: "c".to_string(),
                        size: 10
                    }]
                })]
            }
            .size()
        );
        assert_eq!(
            20,
            FsDirectory {
                name: "/".to_string(),
                files: vec![FsFile {
                    name: "a".to_string(),
                    size: 10
                }],
                directories: vec![Rc::new(FsDirectory {
                    name: "b".to_string(),
                    directories: vec![Rc::new(FsDirectory {
                        name: "d".to_string(),
                        files: vec![],
                        directories: vec![Rc::new(FsDirectory {
                            name: "e".to_string(),
                            files: vec![],
                            directories: vec![]
                        })]
                    })],
                    files: vec![FsFile {
                        name: "c".to_string(),
                        size: 10
                    }]
                })]
            }
            .size()
        );
        assert_eq!(
            30,
            FsDirectory {
                name: "/".to_string(),
                files: vec![FsFile {
                    name: "a".to_string(),
                    size: 10
                }],
                directories: vec![Rc::new(FsDirectory {
                    name: "b".to_string(),
                    directories: vec![Rc::new(FsDirectory {
                        name: "d".to_string(),
                        files: vec![],
                        directories: vec![Rc::new(FsDirectory {
                            name: "e".to_string(),
                            files: vec![FsFile {
                                name: "f".to_string(),
                                size: 10
                            }],
                            directories: vec![]
                        })]
                    })],
                    files: vec![FsFile {
                        name: "c".to_string(),
                        size: 10
                    }]
                })]
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
