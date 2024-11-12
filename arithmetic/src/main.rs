mod errors;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::{ast::print_ast, Parser};
fn main() {
    let input = "1 + 2";
    let  lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    match parser.parse_expression() {
        Ok(ast) => {
            println!("AST:");

            print_ast(&ast, "".to_string(), false);
        }
        Err(error) => println!("{}", error),
    }
}
