use super::Lexer;

pub fn lex_print(input: &str) {
    let mut lexer = Lexer::new(input, false);
    match lexer.lex_string() {
        Ok(tokens_string) => {
            println!("Tokens:\n{}", tokens_string);
        }
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
