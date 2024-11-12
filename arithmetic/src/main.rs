mod lexer;
use lexer::Lexer;

fn main() {

    let input = "sinh() sin cos 
    log ln 
    {
    dsfkdsfgsdf
    sd
    fs
    dfsd
    
    }
    arcsin //comment  

    sin
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
