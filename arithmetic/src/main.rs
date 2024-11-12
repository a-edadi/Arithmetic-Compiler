mod lexer;
use lexer::Lexer;

fn main() {
    let input = "xY__2__2 -3//dfsdfds
    ///
    34
    sd
    32

    ";
    let mut lexer = Lexer::new(input);
    lexer.lex_print_tokens();
    lexer.reset();
}
