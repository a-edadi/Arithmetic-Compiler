mod lexer;
use lexer::Lexer;

fn main() {

    let input = "sinh() sin cos  
    log ln 
    sin 
    xY_2__z_
    ";
    let mut lexer = Lexer::new(input);
    lexer.lex_print_tokens();

    lexer.reset();
    let tokens = lexer.lex_all_tokens();
    lexer.print_token_vec(tokens);

    lexer.reset();
    lexer.lex_print_tokens();

    lexer.reset();
    lexer.lex_debug_print_tokens();
}
