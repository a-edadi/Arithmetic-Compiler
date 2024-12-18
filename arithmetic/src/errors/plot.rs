#[derive(Debug, PartialEq)]
pub enum PlottingError {
    FileCreationError,
    GenericError,
}

impl std::fmt::Display for PlottingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PlottingError::FileCreationError => {
                write!(
                    f,
                    "File creation error: Failed to create or write to the specified file."
                )
            }
            PlottingError::GenericError => {
                write!(f, "Generic plotting error: An unspecified error occurred during the plotting process.")
            }
        }
    }
}

impl std::error::Error for PlottingError {}
