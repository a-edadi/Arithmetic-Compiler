use super::Lexer;

pub fn lex_print_from_input(input: &str) {
    let mut lexer = Lexer::new(input);
    lexer.lex_print_tokens();
}

#[allow(dead_code)]
pub fn lex_print_from_input_with_set_values(input: &str) {
    let mut lexer = Lexer::with_set_values(input, true);
    lexer.lex_print_tokens();
}
