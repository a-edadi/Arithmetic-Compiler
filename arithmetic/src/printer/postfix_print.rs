use super::{ast_to_postfix, Lexer, Parser};

/// gets the input, scans and parses the input build as ast tree then turns that ast tree into postfix notation

pub fn print_postfix(input: &str) {
    let lexer = Lexer::new(input, false);

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
