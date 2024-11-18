use super::Lexer;

/// Lexes an input string and prints the result
pub fn lex_print(input: &str) {
    let mut lexer = Lexer::new(input);

    match lexer.stringify() {
        Ok(tokens_string) => {
            println!("Lexer Output. Token Stream:\n{}", tokens_string);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
