use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidCharacter(char, usize, usize),
    InvalidNumber(String, usize, usize),
    UnexpectedError(usize, usize),
    InvalidIdentifierStart(usize, usize, char),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidNumber(num, line, pos) => {
                write!(
                    f,
                    "Error: Invalid number '{}', In line: {}, at position {}",
                    num, line, pos
                )
            }
            LexerError::InvalidCharacter(char, line, pos) => {
                write!(
                    f,
                    "Error: Invalid Char '{}', In line: {}, at position {}",
                    char, line, pos
                )
            }
            LexerError::UnexpectedError(line, pos) => {
                write!(
                    f,
                    "Error: Unexpected Error Occurred in Line: {}, at position: {}",
                    line, pos
                )
            }
            LexerError::InvalidIdentifierStart(line, pos, c) => {
                write!(
                    f,
                    "Error: Identifier cannot start with a number. Found '{}' at line {}, position {}",
                    c, line, pos
                )
            }
        }
    }
}
