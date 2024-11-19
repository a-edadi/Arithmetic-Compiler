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

/// Wrap the tree with AST for variable management
pub fn print_evaluation(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => {
            let mut ast_wrapper = ASTWrapper::new(ast);
            ast_wrapper.evaluate();
        }
        Err(error) => eprintln!("{}", error),
    }
}

pub fn print_postfix_evaluation(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => {
            let mut ast_wrapper = ASTWrapper::new(ast);
            ast_wrapper.evaluate_postfix();
        }
        Err(error) => eprintln!("{}", error),
    }
}
