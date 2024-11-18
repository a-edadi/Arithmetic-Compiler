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
        self.lex_all().map(|tokens| {
            tokens
                .iter()
                .map(|token| format!("{}\n", token))
                .collect::<String>()
        })
    }
}
