#![allow(dead_code)]
use super::{ASTNode, ASTWrapper, Lexer, Parser};

/// Lex, Parse input and return AST
pub fn lex_parse_input(input: &str) -> Result<ASTNode, String> {
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer).unwrap();
    match parser.parse_expression() {
        Ok(ast) => Ok(ast),
        Err(error) => Err(format!("{}", error)),
    }
}

/// print lexer output
pub fn print_lexer(input: &str) {
    let mut lexer = Lexer::new(input);

    match lexer.stringify() {
        Ok(tokens_string) => {
            println!("Lexer Output. Token Stream:\n{}", tokens_string);
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }
}

/// Prints AST from the input
pub fn print_ast(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => println!("Ast Tree:\n{}", ast.stringify("".to_string(), false)),
        Err(error) => eprintln!(" {}", error),
    }
}

/// Prints the Postfix notation of the AST Tree
pub fn print_postfix(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => println!("Postfix Notation: {}", ast.postfix()),
        Err(error) => eprintln!("{}", error),
    }
}

/// Print the result of the evaluation. gets input from user for variables.
pub fn print_evaluation(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => {
            let mut ast_wrapper = ASTWrapper::new(ast);

            match ast_wrapper.eval_ast() {
                Ok(result) => println!("Evaluation result: {}", result),
                Err(error) => eprintln!("Evaluation error: {:?}", error),
            }
        }
        Err(error) => eprintln!("Parsing error: {}", error),
    }
}

/// Print the roots of the given input
pub fn print_roots(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => {
            let mut wrapper = ASTWrapper::new(ast);

            // Calling the `find_all_roots_bisection` method
            match wrapper.roots_string(None, None) {
                Ok(roots) => {
                    if roots.is_empty() {
                        println!("No roots found in the given interval.");
                    } else {
                        println!("Roots found: {:?}", roots);
                    }
                }
                Err(e) => eprintln!("Error finding roots: {}", e),
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}

/// Plot the function and store it as image.
pub fn print_plot(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => {
            let mut wrapper = ASTWrapper::new(ast);
            let _ = wrapper.plot(None, None);
        }
        Err(error) => eprintln!("{}", error),
    }
}
