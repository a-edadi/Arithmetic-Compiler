mod errors;
mod lexer;
mod parser;

use lexer::print as lexer_print;
use parser::print as parser_print;

fn ultimate_printer(input: &str) {
    lexer_print::lex_print(input);
    parser_print::print_ast(input);
    parser_print::print_postfix(input);
    parser_print::print_evaluation(input);
}

fn main() {
    let input = "-(- sIn (x  * 4*arctan(1)/ 180) //term
    + log (exp(Y1))/log(e) {comment}+ sqrt(sqr(-_xY_2__z_))//term3
    {
        comment
    }
    -2^2^3+X div 10-y1 mod  3+2.31+0.69+1.3E+2)";

    let input2 = "x^2 -4 ";

    ultimate_printer(input);
    parser_print::print_roots(input2);
    parser_print::print_plot(input2);
}
