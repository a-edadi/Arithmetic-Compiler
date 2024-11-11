mod lexer;
use lexer::Lexer;

fn main() {
    let input = "sinh(45) ,";
    let mut lexer = Lexer::new(input);
    lexer.lex_print_tokens();

    lexer.reset();
    let tokens = lexer.lex_all_tokens();
    lexer.print_token_vec(tokens);

    lexer.reset();
    lexer.lex_print_tokens();

    lexer.reset();
    let partial_lexed_token_vec = lexer.lex_till_pos(4);
    lexer.print_token_vec(partial_lexed_token_vec);

    lexer.reset();
    lexer.lex_debug_print_tokens();
}
