use super::lexer::token::TokenKind;
use std::{fmt, usize};

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    // Lexer Stage Errors
    InvalidNumber(String, usize, usize),
    InvalidCharacter(char, usize, usize),
    InvalidIdentifier(usize, usize),

    // Parser Stage Errors
    UnexpectedToken(TokenKind, usize, usize),
    MissingLParen(usize, usize),
    MissingRParen(usize, usize),
    MissingOperator(usize, usize),

    // Evaluation Stage errors
    UnsupportedBinaryOperator(String, usize, usize),
    UnsupportedUnaryOperator(String, usize, usize),
    DivisionByZero(usize, usize),
    IntegerOperatorWithFloatOperands(usize, usize),
    UnsupportedFunction(String, usize),
    InValidConstant(usize, usize),
    InvalidMantissa(usize, usize),

    // Generic error
    UnexpectedError(usize, usize),
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

            CompilerError::InvalidIdentifier(line, pos) => {
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
            CompilerError::DivisionByZero(line, pos) => {
                write!(
                    f,
                    "Runtime Error: Division by zero attempted at line {}, position {}.",
                    line, pos
                )
            }
            CompilerError::MissingOperator(line, pos) => {
                write!(
                    f,
                    "Syntax Error: Missing operator at line {}, position {}.",
                    line, pos
                )
            }
            CompilerError::IntegerOperatorWithFloatOperands(line, column) => {
                write!(
                    f,
                    "Runtime Error: Integer operators (Div or Mod) require integer operands only.  at line: {}, column: {}" , line , column
                )
            }
            CompilerError::UnsupportedBinaryOperator(op, line, pos) => {
                write!(
                    f,
                    "Syntax Error: Unsupported binary operator '{}'. at line: {}, position: {}",
                    op, line, pos
                )
            }
            CompilerError::UnsupportedUnaryOperator(op, line, pos) => {
                write!(
                    f,
                    "Syntax Error: Unsupported unary operator '{}'. at line {}, at position {}",
                    op, line, pos
                )
            }
            CompilerError::UnsupportedFunction(func, line) => {
                write!(
                    f,
                    "Syntax Error: Unsupported function '{}'. at line {}",
                    func, line
                )
            }
            CompilerError::InValidConstant(line, pos) => {
                write!(f, "Invalid Constant '{}'. at line {}", line, pos)
            }
            CompilerError::InvalidMantissa(line, pos) => {
                write!(f, "Invalid Mantissa'{}'. at line {}", line, pos)
            }
            CompilerError::UnexpectedError(line, pos) => {
                write!(
                    f,
                    "Unexpected Error: Occurred at line {}, position {}.",
                    line, pos
                )
            }
        }
    }
}
