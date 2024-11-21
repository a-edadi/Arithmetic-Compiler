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

pub fn print_evaluation(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => {
            let mut ast_wrapper = ASTWrapper::new(ast);

            match ast_wrapper.evaluate() {
                Ok(result) => println!("Evaluation result: {}", result),
                Err(error) => eprintln!("Evaluation error: {:?}", error),
            }
        }
        Err(error) => eprintln!("Parsing error: {}", error),
    }
}

pub fn print_roots(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => {
            let mut wrapper = ASTWrapper::new(ast);

            // Calling the `find_all_roots_bisection` method
            match wrapper.find_roots(None, None) {
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

pub fn print_plot(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => {
            let mut wrapper = ASTWrapper::new(ast);
            let _ = wrapper.plot_function();
        }
        Err(error) => eprintln!("{}", error),
    }
}
