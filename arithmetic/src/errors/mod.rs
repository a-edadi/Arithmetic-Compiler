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
    EvalDivisionByZero,
    IntegerOperatorWithFloatOperands,
    UnsupportedBinaryOperator(String),
    UnsupportedUnaryOperator(String),
    UnsupportedFunction(String),
    TryEvalUnreachable(String),
}

// Implement Display for CompilerError to provide descriptive error messages
impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            CompilerError::EvalDivisionByZero => {
                write!(
                    f,
                    "Runtime Error: Cannot evaluate expression due to division by zero."
                )
            }
            CompilerError::IntegerOperatorWithFloatOperands => {
                write!(
                    f,
                    "Runtime Error: Integer operators (Div or Mod) require integer operands only."
                )
            }
            CompilerError::UnsupportedBinaryOperator(op) => {
                write!(f, "Syntax Error: Unsupported binary operator '{}'.", op)
            }
            CompilerError::UnsupportedUnaryOperator(op) => {
                write!(f, "Syntax Error: Unsupported unary operator '{}'.", op)
            }
            CompilerError::UnsupportedFunction(func) => {
                write!(f, "Syntax Error: Unsupported function '{}'.", func)
            }
            CompilerError::TryEvalUnreachable(s) => {
                write!(
                    f,
                    "Evaluation Error: Unable to evaluate expression without values. {}",
                    s
                )
            }
        }
    }
}
