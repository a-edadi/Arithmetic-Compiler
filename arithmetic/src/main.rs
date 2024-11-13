mod errors;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::{ast::print_ast, Parser};
fn main() {
    // let input = "1 {comment}+ 2";
    let input = "-(- sin (x   * 4*arctan(1) /180) //term
    + log (exp(Y1))/log(e) {comment}+ sqrt(sqr(-_xY_2__z_))//term3 
    { 
    comment 
    } 
    -2^2^3+X div 10-y1 mod 3+2.31+0.69+1.3*E+2) ";
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
