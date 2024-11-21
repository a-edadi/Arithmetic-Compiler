pub mod eval;
pub mod lexer;
pub mod parser;
pub mod plot;
pub mod root_finder;

use crate::lexer::token::TokenKind;
use eval::EvaluationError;
use lexer::LexerError;
use parser::ParserError;
use plot::PlottingError;
use root_finder::RootFinderError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CompilerError {
    Lex(LexerError),
    Parse(ParserError),
    Eval(EvaluationError),
    Root(RootFinderError),
    Plot(PlottingError),
    GenericError(usize, usize),
}

// Implement Display for CompilerError
impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::Lex(err) => write!(f, "{}", err),
            CompilerError::Parse(err) => write!(f, "{}", err),
            CompilerError::Eval(err) => write!(f, "{}", err),
            CompilerError::Root(err) => write!(f, "{}", err),
            CompilerError::Plot(err) => write!(f, "{}", err),

            CompilerError::GenericError(line, pos) => {
                write!(
                    f,
                    "Unexpected error occurred at line {}, position {}.",
                    line, pos
                )
            }
        }
    }
}
