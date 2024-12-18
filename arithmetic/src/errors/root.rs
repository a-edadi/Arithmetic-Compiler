#[derive(Debug, PartialEq)]
pub enum RootFinderError {
    InvalidInterval,
    NoRootInInterval,
    MaxIterationsReached,
}

// Implement Display for RootFinderError
impl std::fmt::Display for RootFinderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RootFinderError::InvalidInterval => {
                write!(f, "Invalid interval: The interval provided is not valid.")
            }
            RootFinderError::NoRootInInterval => {
                write!(f, "No root in interval: The function does not have a root in the specified interval.")
            }
            RootFinderError::MaxIterationsReached => {
                write!(f, "Maximum iterations reached: The root-finding algorithm did not converge within the maximum allowed iterations.")
            }
        }
    }
}

impl std::error::Error for RootFinderError {}
