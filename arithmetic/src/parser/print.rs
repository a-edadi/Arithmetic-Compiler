use super::{ast::ASTNode, Lexer, Parser};

/// Lex, Parse input and return AST
pub fn lex_parse_input(input: &str, use_variables: bool) -> Result<ASTNode, String> {
    let lexer = Lexer::new(input, use_variables);

    let mut parser = Parser::new(lexer).unwrap();
    match parser.parse_expression() {
        Ok(ast) => Ok(ast),
        Err(error) => Err(format!("{}", error)),
    }
}

/// Prints AST from the input
pub fn print_ast(input: &str) {
    match lex_parse_input(input, false) {
        Ok(ast) => println!("Ast Tree:\n{}", ast.to_string_tree("".to_string(), false)),
        Err(error) => eprintln!(" {}", error),
    }
}

/// Prints AST from the input
/// replaces variables with actual values provided by the user
pub fn print_ast_with_values(input: &str) {
    match lex_parse_input(input, true) {
        Ok(ast) => println!("Ast Tree:\n{}", ast.to_string_tree("".to_string(), false)),
        Err(error) => eprintln!("{}", error),
    }
}

/// Prints the Postfix notation of the AST Tree
pub fn print_postfix(input: &str) {
    match lex_parse_input(input, false) {
        Ok(ast) => println!("Postfix Notation: {}", ast.postfix()),
        Err(error) => eprintln!("{}", error),
    }
}

/// Prints the final result of the AST tree evaluation.
pub fn print_evaluation(input: &str) {
    match lex_parse_input(input, true) {
        Ok(ast) => println!("{}", ast.eval_result()),
        Err(error) => eprintln!("{}", error),
    }
}
