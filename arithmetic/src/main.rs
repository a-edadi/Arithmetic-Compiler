mod ast;
mod errors;
mod lexer;
mod parser;
mod utils;
use std::fs;

use utils::ultimate::{ultimate_ast_postfix_eval, ultimate_root_plot};

fn main() {
    let input = fs::read_to_string("inputs/input.txt").expect("Unable to read from file");
    let input2 = fs::read_to_string("inputs/plot.txt").expect("Unable to read from file");

    println!("###################### Part 1: Lexer, Parser(AST) , Postfix , Evaluation ######################\n");
    ultimate_ast_postfix_eval(&input);
    println!("\n\n ###################### Part 2: Roots and Plot ######################\n");
    ultimate_root_plot(&input2);
}
