use super::{ASTNode, Lexer, Parser};

/// Lex, Parse input and return AST
pub fn lex_parse_input(input: &str) -> Result<ASTNode, String> {
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer).unwrap();
    match parser.parse_expression() {
        Ok(ast) => Ok(ast),
        Err(error) => Err(format!("{}", error)),
    }
}

pub fn lex_print_parse_input(input: &str) -> Result<ASTNode, String> {
    let mut lexer = Lexer::new(input);

    match lexer.stringify() {
        Ok(tokens_string) => {
            println!("Lexer Output. Token Stream:\n{}", tokens_string);
        }
        // this same error is raised in the parsing stage since parser uses lexer
        // return None and let parser return this error to avoid printing the error twice
        Err(_) => {}
    }

    lexer.reset();

    let mut parser = Parser::new(lexer).unwrap();

    match parser.parse_expression() {
        Ok(ast) => Ok(ast),
        Err(error) => Err(format!("{}", error)),
    }
}
