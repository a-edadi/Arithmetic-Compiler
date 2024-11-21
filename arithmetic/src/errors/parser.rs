use super::TokenKind;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParserError {
    UnexpectedToken(TokenKind, usize, usize),
    MissingLParen(usize, usize),
    MissingRParen(usize, usize),
    MissingOperator(usize, usize),
}

// Implement Display for ParserError
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(kind, line, pos) => {
                write!(
                    f,
                    "Syntax Error: Unexpected token '{}' at line {}, position {}.",
                    kind, line, pos
                )
            }
            ParserError::MissingLParen(line, pos) => {
                write!(
                    f,
                    "Syntax Error: Missing '(' at line {}, position {}.",
                    line, pos
                )
            }
            ParserError::MissingRParen(line, pos) => {
                write!(
                    f,
                    "Syntax Error: Missing ')' at line {}, position {}.",
                    line, pos
                )
            }
            ParserError::MissingOperator(line, pos) => {
                write!(
                    f,
                    "Syntax Error: Missing operator at line {}, position {}.",
                    line, pos
                )
            }
        }
    }
}
