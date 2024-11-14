use super::Lexer;

pub fn lex_print_input(input: &str) {
    let mut lexer = Lexer::new(input, false);
    let tokens_result = lexer.lex_all_tokens();

    match tokens_result {
        Ok(tokens) => {
            for token in tokens {
                println!("{}", token);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

#[allow(dead_code)]
pub fn lex_print_input_with_values(input: &str) {
    let mut lexer = Lexer::new(input, true);
    match lexer.lex_to_token_string() {
        Ok(tokens_string) => {
            println!("Tokens:\n{}", tokens_string);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
