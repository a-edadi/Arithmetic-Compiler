mod errors;
mod lexer;
mod parser;

use lexer::{utils::lex_print_from_input, Lexer};
use parser::utils::{print_ast, print_ast_with_values, print_postfix};

fn ultimate_printer(input: &str) {
    lex_print_from_input(input);
    print_ast(input);
    print_postfix(input);
    // print_evaluation(input);
}

fn main() {
    let input2 = "2^2^3";

    let input = "-(- sin (x   * 4*arctan(1) /180) //term
    + log (exp(Y1))/log(e) {comment}+ sqrt(sqr(-_xY_2__z_))//term3
    {
    comment
    }
    -2^2^3+X div 10-y1 mod 3+2.31+0.69+1.3*E+2) ";
    ultimate_printer(input2);
    println!("---------------------------------------------");
    print_ast_with_values(input)
}
