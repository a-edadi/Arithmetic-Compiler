use super::errors::LexerError;
use super::token::Token;
use super::Lexer;

impl<'a> Lexer<'a> {
    /// Pretty prints and lexes all tokens using only a ref to self
    pub fn lex_print_tokens(&mut self) {
        let tokens_result = self.lex_all_tokens();

        match tokens_result {
            Ok(tokens) => {
                for token in tokens {
                    println!("{}", token);
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
    /// Simple printing using the debug
    pub fn lex_debug_print_tokens(&mut self) {
        let tokens_result = self.lex_all_tokens();

        match tokens_result {
            Ok(tokens) => {
                for token in tokens {
                    println!("{:?}", token);
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }

    /// Prints a given lexed vector input
    pub fn print_token_vec(&mut self, vec: Result<Vec<Token>, LexerError>) {
        match vec {
            Ok(tokens) => {
                for token in tokens {
                    println!("{}", token);
                }
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}
