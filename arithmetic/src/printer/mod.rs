mod ast;
use crate::lexer::Lexer;
use crate::parser::{ast::ASTNode, eval::evaluate_ast, postfix::ast_to_postfix, Parser};
use ast::print_ast_tree;

/// Wrappers to make printing to terminal IO easier.

pub fn print_ast(input: &str) {
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer).unwrap();
    let ast = match parser.parse_expression() {
        Ok(ast) => ast,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    println!("Ast Tree:");
    print_ast_tree(&ast, "".to_string(), false);
}

pub fn print_ast_with_values(input: &str) {
    let lexer = Lexer::with_set_values(input, true);

    let mut parser = Parser::new(lexer).unwrap();
    let ast = match parser.parse_expression() {
        Ok(ast) => ast,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    println!("Ast Tree:");
    print_ast_tree(&ast, "".to_string(), false);
}

pub fn print_postfix(input: &str) {
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer).unwrap();
    let ast = match parser.parse_expression() {
        Ok(ast) => ast,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    println!("Postfix Notation:");
    let postfix = ast_to_postfix(&ast);
    println!("{}", postfix);
}

pub fn print_evaluation(input: &str) {
    let lexer = Lexer::with_set_values(input, true);

    let mut parser = Parser::new(lexer).unwrap();
    let ast = match parser.parse_expression() {
        Ok(ast) => ast,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    let r = evaluate_ast(&ast);
    println!("Final result is: {:?}", r);
}
