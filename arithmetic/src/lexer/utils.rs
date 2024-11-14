use super::CompilerError;

use super::token::{Token, TokenKind};

use crate::Lexer;

/// lexes all tokens and returns a vector of theses tokens
impl<'a> Lexer<'a> {
    pub fn lex_all_tokens(&mut self) -> Result<Vec<Token>, CompilerError> {
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

    /// Lexes all tokens and returns a formatted string representation of the input
    pub fn lex_to_token_string(&mut self) -> Result<String, CompilerError> {
        let tokens_result = self.lex_all_tokens();

        match tokens_result {
            Ok(tokens) => {
                let mut result = String::new();
                for token in tokens {
                    result.push_str(&format!("{}\n", token));
                }
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    #[allow(dead_code)]
    /// Built in Terminal Printing Utility
    pub fn lex_print_tokens(&mut self) {
        match self.lex_to_token_string() {
            Ok(tokens_string) => {
                println!("Tokens:\n{}", tokens_string);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
