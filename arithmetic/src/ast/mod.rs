pub mod ast;
pub mod ast_postfix;
pub mod ast_string;
pub mod ast_wrapper;
pub mod eval;
pub mod function_plotter;
pub mod root_finder;
pub mod utils;
pub mod var_manager;

use crate::ast::utils::{
    prompt_input::get_and_parse_user_input, random_number::generate_random_4_digits,
};
use crate::errors::{
    eval::EvaluationError, plot::PlottingError, root_finder::RootFinderError, CompilerError,
};
use crate::lexer::{
    span::TextSpan,
    token::{Num, TokenKind},
    Lexer,
};
use crate::parser::Parser;
use ast::ASTNode;
use ast_wrapper::ASTWrapper;
use eval::Evaluator;
use function_plotter::FunctionPlotter;
use root_finder::RootFinder;
use var_manager::VariableManager;
