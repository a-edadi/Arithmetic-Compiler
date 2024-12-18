pub mod print;
pub mod prompt;
pub mod rand;
pub mod ultimate;

use crate::ast::wrapper::ASTWrapper;
use crate::ast::ASTNode;
use crate::lexer::Lexer;
use crate::parser::Parser;
use print::lex_parse_input;
use prompt::get_and_parse_user_input;
