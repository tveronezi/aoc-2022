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

#[cfg(test)]
mod tests {
    use super::*;

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
