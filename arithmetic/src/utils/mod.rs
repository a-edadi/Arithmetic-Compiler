pub mod lex_parse_input;
pub mod print;
pub mod prompt_input;
pub mod random_number;
pub mod ultimate;

use crate::ast::ASTNode;
use crate::ast::wrapper::ASTWrapper;
use crate::lexer::Lexer;
use crate::parser::Parser;
use lex_parse_input::{lex_parse_input, lex_print_parse_input};
use prompt_input::get_and_parse_user_input;
