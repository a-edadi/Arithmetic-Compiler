mod errors;
mod lexer;
mod parser;
mod printer;

use crate::printer::{ast_print, eval_print, lexer_print, postfix_print};
use lexer::Lexer;

fn ultimate_printer(input: &str) {
    lexer_print::lex_print_input(input);
    ast_print::print_ast(input);
    postfix_print::print_postfix(input);
    ast_print::print_ast_with_values(input);
    eval_print::print_evaluation(input);
}

fn main() {
    #[allow(unused_variables)]
    let input2 = "1 + 1 - ! 2 + 3";

    let input = "-(-sin(180 * 4*arctan(1) / 180) //term
    + log (exp(Y1))/log(e) {comment}+ sqrt(sqr(-_xY_2__z_))//term3
    {
    comment
    }
    -2^2^3+X div 10-y1 mod 3 +2.31+0.69+130) ";

    ultimate_printer(input2);
    let mut lexer = Lexer::new(input2, false);
    match lexer.lex_to_token_string() {
        Ok(tokens_string) => {
            println!("Tokens:\n{}", tokens_string); // Output the tokens string
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
