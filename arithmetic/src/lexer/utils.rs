use super::{CompilerError, Lexer, Token, TokenKind};

impl<'a> Lexer<'a> {
    /// lexes all tokens and returns a vector of theses tokens
    pub fn lex_all(&mut self) -> Result<Vec<Token>, CompilerError> {
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
    pub fn stringify(&mut self) -> Result<String, CompilerError> {
        let tokens_result = self.lex_all();

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
}
