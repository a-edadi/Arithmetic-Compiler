use super::errors::LexerError;
use super::token::TokenKind;
use crate::Lexer;

impl<'a> Lexer<'a> {
    pub fn is_number_start(c: &char) -> bool {
        c.is_ascii_digit()
    }

    pub fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic() || c == &'_'
    }

    pub fn is_ascii_start(c: &char) -> bool {
        c.is_ascii()
    }

    /// Skip White spaces.
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn lex_potential_double_char_operator(
        &mut self,
        expected: char,
        one_char_kind: TokenKind,
        double_char_kind: TokenKind,
    ) -> TokenKind {
        if let Some(next) = self.current_char() {
            if next == expected {
                self.advance();
                double_char_kind
            } else {
                one_char_kind
            }
        } else {
            one_char_kind
        }
    }

    /// Handle the punctuations operators and separators
    pub fn handle_punctuation(&mut self) -> Result<TokenKind, LexerError> {
        let c = self.advance().unwrap();
        match c {
            '+' => Ok(TokenKind::Plus),
            '-' => Ok(TokenKind::Minus),
            '*' => Ok(TokenKind::Multiply),
            '%' => Ok(TokenKind::Remainder),
            '/' => {
                Ok(self.lex_potential_double_char_operator('/', TokenKind::Divide, TokenKind::Div))
            }
            '(' => Ok(TokenKind::LeftParen),
            ')' => Ok(TokenKind::RightParen),
            '^' => Ok(TokenKind::Power),
            _ => Err(LexerError::InvalidCharacter(c, self.current_pos - 1)),
        }
    }

    /// tokenize the input so we can return identifiers
    pub fn handle_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if Self::is_identifier_start(&c) {
                self.advance().unwrap();
                identifier.push(c);
            } else {
                break;
            }
        }
        identifier
    }

    /// Handle numbers by collecting the numbers inside a string and parsing them
    pub fn handle_number(&mut self) -> Result<i64, LexerError> {
        let start_pos = self.current_pos;
        let mut number_str = String::new();

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                number_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if let Ok(number) = number_str.parse::<i64>() {
            Ok(number)
        } else {
            Err(LexerError::InvalidNumber(number_str, start_pos))
        }
    }
}

/// Handling the comments
impl<'a> Lexer<'a> {
    /// Exclude the rest of the line when // is seen
    pub fn handle_line_comment(&mut self) {
        while let Some(c) = self.peek_char() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    // handle block comments
    pub fn handle_block_comment(&mut self) {
        self.advance(); // Skip the initial '{'
        while let Some(c) = self.peek_char() {
            if c == '}' {
                self.advance(); // Skip the closing '}'
                break;
            }
            self.advance();
        }
    }
}
