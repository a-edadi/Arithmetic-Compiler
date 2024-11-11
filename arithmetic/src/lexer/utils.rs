use crate::lexer::errors::LexerError;

use super::token::{Token, TokenKind};

use crate::Lexer;

/// lexes all tokens and returns a vector of theses tokens
impl<'a> Lexer<'a> {
    pub fn lex_all_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens_vec = Vec::new();

        loop {
            match self.get_next_token() {
                Ok(token) => {
                    if token.kind == TokenKind::Eof {
                        break;
                    }
                    tokens_vec.push(token);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(tokens_vec)
    }
}

/// Terminal Printing Utilities
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

    /// each token occupies a single line
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
