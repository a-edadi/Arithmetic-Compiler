mod errors;
mod lexer;
mod parser;
mod printer;

use lexer::{utils::lex_print_from_input, Lexer};
use printer::{print_ast, print_ast_with_values, print_evaluation, print_postfix};

fn ultimate_printer(input: &str) {
    lex_print_from_input(input);
    print_ast(input);
    print_postfix(input);
    print_ast_with_values(input);
    print_evaluation(input);
}

fn main() {
    let input2 = "sin(x) + e x - E+2";

    let input = "-(-sin(180 * 4*arctan(1) / 180) //term
    + log (exp(Y1))/log(e) {comment}+ sqrt(sqr(-_xY_2__z_))//term3
    {
    comment
    }
    -2^2^3+X div 10-y1 mod 3.5 +2.31+0.69+130) ";

    // ultimate_printer(input2);
    ultimate_printer(input);
    lex_print_from_input(input2);
}
