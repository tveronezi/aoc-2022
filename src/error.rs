use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Ooops(pub(crate) String);

impl std::error::Error for Ooops {}

impl Display for Ooops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
