use std::fmt;

#[derive(Debug, PartialEq)]
pub enum EvaluationError {
    UnsupportedBinaryOperator(String, usize, usize),
    UnsupportedUnaryOperator(String, usize, usize),
    DivisionByZero(usize, usize),
    IntegerOperatorWithFloatOperands(usize, usize),
    UnsupportedFunction(String, usize),
    InvalidConstant(usize, usize),
    InvalidMantissa(usize, usize),
}

// Implement Display for EvaluationError
impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluationError::UnsupportedBinaryOperator(op, line, pos) => {
                write!(
                    f,
                    "Runtime Error: Unsupported binary operator '{}' at line {}, position {}.",
                    op, line, pos
                )
            }
            EvaluationError::UnsupportedUnaryOperator(op, line, pos) => {
                write!(
                    f,
                    "Runtime Error: Unsupported unary operator '{}' at line {}, position {}.",
                    op, line, pos
                )
            }
            EvaluationError::DivisionByZero(line, pos) => {
                write!(
                    f,
                    "Runtime Error: Division by zero at line {}, position {}.",
                    line, pos
                )
            }
            EvaluationError::IntegerOperatorWithFloatOperands(line, pos) => {
                write!(
                    f,
                    "Runtime Error: Integer operator used with float operands at line {}, position {}.",
                    line, pos
                )
            }
            EvaluationError::UnsupportedFunction(func, line) => {
                write!(
                    f,
                    "Runtime Error: Unsupported function '{}' at line {}.",
                    func, line
                )
            }
            EvaluationError::InvalidConstant(line, pos) => {
                write!(
                    f,
                    "Runtime Error: Invalid constant at line {}, position {}.",
                    line, pos
                )
            }
            EvaluationError::InvalidMantissa(line, pos) => {
                write!(
                    f,
                    "Runtime Error: Invalid mantissa at line {}, position {}.",
                    line, pos
                )
            }
        }
    }
}
