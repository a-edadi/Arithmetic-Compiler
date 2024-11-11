use crate::lexer::errors::LexerError;

use super::token::{Token, TokenKind};

use crate::Lexer;

impl<'a> Lexer<'a> {
    /// lexer lexes all tokens and returns a vector of theses tokens
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

    /// lexer continues to work till the give position and returns a Vector
    pub fn lex_till_pos(&mut self, end_position: usize) -> Result<Vec<Token>, LexerError> {
        let mut tokens_vec = Vec::new();

        loop {
            match self.get_next_token() {
                Ok(token) => {
                    if token.kind == TokenKind::Eof {
                        break;
                    }
                    if self.current_pos > end_position {
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
