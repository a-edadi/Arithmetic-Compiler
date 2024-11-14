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
    let r = evaluate_ast(&ast);
    println!("Final result is: {:?}", r);
}
