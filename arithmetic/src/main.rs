mod ast;
mod errors;
mod lexer;
mod parser;

use ast::utils::ultimate::{ultimate_ast_postfix_eval, ultimate_root_plot};

fn main() {
    let input = "-(- sIn (x  * 4*arctan(1)/ 180) //term
    + log (exp(Y1))/log(e) {comment}+ sqrt(sqr(-_xY_2__z_))//term3
    {
        comment
    }
    -2^2^3+X div 10-y1 mod  3+2.31+0.69+1.3E+2)";

    // let input2 = "x^2 - 4";
    let input2 = "sin(x)";

    ultimate_ast_postfix_eval(input);
    println!("\n\n\n Pause \n\n\n");
    ultimate_root_plot(input2);
}
