#[derive(Debug, PartialEq)]
pub enum PlottingError {
    FileCreationError,
    GenericPlottingError,
}

impl std::fmt::Display for PlottingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
