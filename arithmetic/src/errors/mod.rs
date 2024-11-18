use super::lexer::token::TokenKind;
use std::{fmt, usize};

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    InvalidCharacter(char, usize, usize),
    InvalidNumber(String, usize, usize),
    UnexpectedError(usize, usize),
    InvalidIdentifierStart(usize, usize),

    // Parser Errors
    UnexpectedToken(TokenKind, usize, usize),
    MissingLParen(usize, usize),
    MissingRParen(usize, usize),
    DivisionByZero(usize),
    MissingOperator(usize, usize),

    // Evaluation errors
    EvalDivisionByZero(usize),
    IntegerOperatorWithFloatOperands(usize, usize),
    UnsupportedBinaryOperator(String, usize),
    UnsupportedUnaryOperator(String, usize),
    UnsupportedFunction(String, usize),
    Unexpected(),
}

// Implement Display for CompilerError to provide descriptive error messages
impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Unexpected() => {
                write!(f, "something unexpected happened at eval stage")
            }
            CompilerError::InvalidNumber(num, line, pos) => {
                write!(
                    f,
                    "Syntax Error: Invalid number '{}', found at line {}, position {}.",
                    num, line, pos
                )
            }
            CompilerError::InvalidCharacter(char, line, pos) => {
                write!(
                    f,
                    "Syntax Error: Invalid character '{}', found at line {}, position {}.",
                    char, line, pos
                )
            }
            CompilerError::UnexpectedError(line, pos) => {
                write!(
                    f,
                    "Unexpected Error: Occurred at line {}, position {}.",
                    line, pos
                )
            }
            CompilerError::InvalidIdentifierStart(line, pos) => {
                write!(
                    f,
                    "Syntax Error: Identifier cannot start with a number, found at line {}, position {}.",
                    line, pos
                )
            }
            CompilerError::UnexpectedToken(kind, line, pos) => {
                write!(
                    f,
                    "Syntax Error: Unexpected token '{}' at line {}, position {}.",
                    kind, line, pos
                )
            }
            CompilerError::MissingLParen(line, pos) => {
                write!(
                    f,
                    "Syntax Error: Missing '(' at line {}, position {}.",
                    line, pos
                )
            }
            CompilerError::MissingRParen(line, pos) => {
                write!(
                    f,
                    "Syntax Error: Missing ')' at line {}, position {}.",
                    line, pos
                )
            }
            CompilerError::DivisionByZero(pos) => {
                write!(
                    f,
                    "Runtime Error: Division by zero attempted at position {}.",
                    pos
                )
            }
            CompilerError::MissingOperator(line, pos) => {
                write!(
                    f,
                    "Syntax Error: Missing operator at line {}, position {}.",
                    line, pos
                )
            }
            CompilerError::EvalDivisionByZero(line) => {
                write!(
                    f,
                    "Runtime Error: Cannot evaluate expression due to division by zero.  at line: {}" , line
                )
            }
            CompilerError::IntegerOperatorWithFloatOperands(line, column) => {
                write!(
                    f,
                    "Runtime Error: Integer operators (Div or Mod) require integer operands only.  at line: {}, column: {}" , line , column
                )
            }
            CompilerError::UnsupportedBinaryOperator(op, line) => {
                write!(
                    f,
                    "Syntax Error: Unsupported binary operator '{}'. at line: {}",
                    op, line
                )
            }
            CompilerError::UnsupportedUnaryOperator(op, line) => {
                write!(
                    f,
                    "Syntax Error: Unsupported unary operator '{}'. at line {}",
                    op, line
                )
            }
            CompilerError::UnsupportedFunction(func, line) => {
                write!(
                    f,
                    "Syntax Error: Unsupported function '{}'. at line {}",
                    func, line
                )
            }
        }
    }
}
