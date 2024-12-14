use super::{get_and_parse_user_input, lex_parse_input, lex_print_parse_input, ASTWrapper};

/// lexes -> print tokens -> parses -> print tree -> print postfix --
/// --> evaluate(get variable values from user and evaluate the ast).
pub fn ultimate_ast_postfix_eval(input: &str) {
    match lex_print_parse_input(input) {
        Ok(ast) => {
            let mut wrapper = ASTWrapper::new(ast);

            let ast_string = wrapper.ast_string();
            println!("{}", ast_string);

            let postfix = wrapper.ast_postfix_string();
            println!("{}", postfix);

            match wrapper.eval_ast() {
                Ok(result) => {
                    println!("Evaluation result: {}", result);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }
        }
        Err(error) => eprintln!("{}", error),
    }
}

/// Returns Roots and plots the function.
pub fn ultimate_root_plot(input: &str) {
    match lex_parse_input(input) {
        Ok(ast) => {
            let mut wrapper = ASTWrapper::new(ast);

            let a = get_and_parse_user_input("a");
            let b = get_and_parse_user_input("b");

            match wrapper.roots_string(Some(a), Some(b)) {
                Ok(s) => println!("Roots: {}", s),
                Err(e) => eprintln!("{}", e),
            }

            match wrapper.plot(Some(a), Some(b)) {
                Ok(_) => println!("Function plot generated successfully."),
                Err(e) => eprintln!("Plotting error: {}", e),
            }
        }
        Err(error) => eprintln!("Error parsing input: {}", error),
    }
}
