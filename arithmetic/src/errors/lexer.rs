use std::fmt;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidNumber(String, usize, usize),
    InvalidCharacter(char, usize, usize),
    InvalidIdentifier(usize, usize),
}

// Implement Display for LexerError
impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::InvalidNumber(num, line, pos) => {
                write!(
                    f,
                    "Syntax Error: Invalid number '{}' at line {}, position {}.",
                    num, line, pos
                )
            }
            LexerError::InvalidCharacter(ch, line, pos) => {
                write!(
                    f,
                    "Syntax Error: Invalid character '{}' at line {}, position {}.",
                    ch, line, pos
                )
            }
            LexerError::InvalidIdentifier(line, pos) => {
                write!(
                    f,
                    "Syntax Error: Invalid identifier at line {}, position {}.",
                    line, pos
                )
            }
        }
    }
}

impl std::error::Error for LexerError {}
