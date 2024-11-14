pub mod ast_print;
pub mod eval_print;
pub mod lexer_print;
pub mod postfix_print;

use crate::lexer::{token::Num, Lexer};
use crate::parser::{ast::ASTNode, eval::evaluate_ast, postfix::ast_to_postfix, Parser};
