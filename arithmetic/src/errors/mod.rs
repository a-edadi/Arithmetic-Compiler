use super::lexer::token::TokenKind;
use std::{fmt, usize};

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    InvalidCharacter(char, usize, usize),
    InvalidNumber(String, usize, usize),
    UnexpectedError(usize, usize),
    InvalidIdentifierStart(usize, usize),

    UnexpectedToken(TokenKind, usize, usize),
    MissingToken(String),
    DivisionByZero(usize),
    MissingOperator(Option<String>),
}
// Implement Display for CompileError to provide custom error messages
impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::InvalidNumber(num, line, pos) => {
                write!(
                    f,
                    "Error: Invalid number '{}', In line: {}, at position {}",
                    num, line, pos
                )
            }
            CompilerError::InvalidCharacter(char, line, pos) => {
                write!(
                    f,
                    "Error: Invalid Char '{}', In line: {}, at position {}",
                    char, line, pos
                )
            }
            CompilerError::UnexpectedError(line, pos) => {
                write!(
                    f,
                    "Error: Unexpected Error Occurred in Line: {}, at position: {}",
                    line, pos
                )
            }
            CompilerError::InvalidIdentifierStart(line, pos) => {
                write!(
                    f,
                    "Error: Identifier cannot start with a number found at at line {}, position {}",
                    line, pos
                )
            }
            CompilerError::UnexpectedToken(kind, line, pos) => {
                write!(
                    f,
                    "Compile Error: Unexpected token '{}' detected at line: {}, position: {}",
                    kind, line, pos
                )
            }
            CompilerError::MissingToken(token) => {
                write!(f, "Compile Error: Expected token '{}' is missing", token)
            }
            CompilerError::DivisionByZero(pos) => {
                write!(
                    f,
                    "Compile Error: Attempted division by zero at position {}",
                    pos
                )
            }
            CompilerError::MissingOperator(string) => {
                write!(f, "Compile Error: Missing Operator. {:?}", string)
            }
        }
    }
}
