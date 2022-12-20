use std::fmt::Display;

/// Lib errors
#[derive(Debug, PartialEq, Eq)]
pub struct Ooops(pub String);

impl std::error::Error for Ooops {}

impl Display for Ooops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
