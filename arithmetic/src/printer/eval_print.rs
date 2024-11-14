use super::{evaluate_ast, Lexer, Parser};

pub fn print_evaluation(input: &str) {
    let lexer = Lexer::new(input, true);

    let mut parser = Parser::new(lexer).unwrap();
    let ast = match parser.parse_expression() {
        Ok(ast) => ast,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    match evaluate_ast(&ast) {
        Ok(r) => {
            println!("Final result is: {:?}", r);
        }
        Err(e) => {
            println!("{}", e)
        }
    }
}
