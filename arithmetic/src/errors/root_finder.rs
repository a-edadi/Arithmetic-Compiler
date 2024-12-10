#[derive(Debug, PartialEq)]
pub enum RootFinderError {
    InvalidInterval,
    NoRootInInterval,
    MaxIterationsReached,
}

impl std::fmt::Display for RootFinderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
