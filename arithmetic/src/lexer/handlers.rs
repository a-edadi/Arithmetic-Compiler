use super::token::TokenKind;
use super::CompilerError;
use crate::Lexer;

impl<'a> Lexer<'a> {
    pub fn is_number_start(c: &char) -> bool {
        c.is_ascii_digit()
    }

    pub fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic() || c == &'_'
    }

    fn is_identifier_continuation(c: &char) -> bool {
        c.is_alphanumeric() || *c == '_'
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

    /// Handle the punctuations operators and separators
    pub fn handle_punctuation(&mut self) -> Result<TokenKind, CompilerError> {
        let c = self.advance().unwrap();
        match c {
            '+' => Ok(TokenKind::Plus),
            '-' => Ok(TokenKind::Minus),
            '*' => Ok(TokenKind::Multiply),
            '/' => Ok(TokenKind::Divide),
            '(' => Ok(TokenKind::LeftParen),
            ')' => Ok(TokenKind::RightParen),
            '^' => Ok(TokenKind::Power),
            _ => Err(CompilerError::InvalidCharacter(
                c,
                self.line,
                self.current_pos - 1,
            )),
        }
    }

    pub fn handle_identifier(&mut self) -> Result<String, CompilerError> {
        let mut identifier = String::new();

        // Ensure the first character is valid for the start of an identifier
        if let Some(c) = self.current_char() {
            if c.is_digit(10) {
                // If the identifier starts with a number, raise an error
                return Err(CompilerError::InvalidIdentifierStart(
                    self.line,
                    self.current_pos,
                ));
            } else if Self::is_identifier_start(&c) {
                identifier.push(c);
                self.advance();
            }
        }

        // Continue adding while valid identifier characters (alphanumeric or underscore)
        while let Some(c) = self.current_char() {
            if Self::is_identifier_continuation(&c) {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        Ok(identifier)
    }

    /// Handle numbers by collecting the numbers inside a string and parsing them
    pub fn handle_number(&mut self) -> Result<f64, CompilerError> {
        let start_pos = self.current_pos;
        let mut number_str = String::new();
        let mut has_dot = false;

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                number_str.push(c);
                self.advance();
            } else if c == '.' && !has_dot {
                // Allow only one decimal point
                number_str.push(c);
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        // Try to parse the number as f64
        if let Ok(number) = number_str.parse::<f64>() {
            Ok(number)
        } else {
            Err(CompilerError::InvalidNumber(
                number_str, self.line, start_pos,
            ))
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
