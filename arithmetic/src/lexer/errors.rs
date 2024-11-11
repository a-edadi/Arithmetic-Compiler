use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidCharacter(char, usize),
    InvalidNumber(String, usize),
    UnexpectedError(usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidNumber(num, pos) => {
                write!(f, "Error: Invalid number '{}' at position {}", num, pos)
            }
            LexerError::InvalidCharacter(char, pos) => {
                write!(f, "Error: Invalid Char '{}' at position {}", char, pos)
            }
            LexerError::UnexpectedError(pos) => {
                write!(f, "Error: Unexpected Error Occurred at position : {}", pos)
            }
        }
    }
}
