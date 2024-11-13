mod errors;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::{ast::print_ast, Parser};
fn main() {
    let input = "1 mod 2 + x + log(x) + (-1) + pi + 2 - sin(y)- sqrt(2)";
    let mut lexer = Lexer::new(input);
    lexer.lex_print_tokens();
    lexer.reset();
    let mut parser = Parser::new(lexer).unwrap();
    match parser.parse_expression() {
        Ok(ast) => {
            println!("AST:");

            print_ast(&ast, "".to_string(), false);
        }
        Err(error) => println!("{}", error),
    }
}
