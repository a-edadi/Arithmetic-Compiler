mod ast;
mod errors;
mod lexer;
mod parser;
use std::fs;

use ast::utils::ultimate::{ultimate_ast_postfix_eval, ultimate_root_plot};

fn main() {
    //Amirhossein Edadi, Amin OwrangPour, Amin Shiekh Azimi

    // files: input, input2, input3
    let input = fs::read_to_string("inputs/input.txt").expect("Unable to read from file");

    // file: plot
    let input2 = fs::read_to_string("inputs/plot.txt").expect("Unable to read from file");

    println!("Part 1: Lexer, Parser(AST) , Postfix , Evaluation \n");
    ultimate_ast_postfix_eval(&input);
    println!("\n\nPart 2: Roots and Plot \n");
    ultimate_root_plot(&input2);
}
