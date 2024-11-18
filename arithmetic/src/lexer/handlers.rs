use super::{CompilerError, Lexer, Num, TokenKind};

impl<'a> Lexer<'a> {
    /// 0...9
    pub fn is_number_start(c: &char) -> bool {
        c.is_ascii_digit()
    }

    /// a...z
    pub fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic() || c == &'_'
    }

    /// indicates that x2 is a valid variable
    fn is_identifier_continuation(c: &char) -> bool {
        c.is_alphanumeric() || *c == '_'
    }

    /// + - / * ( )
    pub fn is_ascii_start(c: &char) -> bool {
        c.is_ascii()
    }

    /// Skip White spaces: spaces, newline, \t
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
    /// The ones that are not alphabetic but are ascii
    pub fn handle_punctuation(&mut self) -> Result<TokenKind, CompilerError> {
        let c = self.current_char().unwrap();
        self.advance();
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

    /// Handles Identifiers: sin, cos, func, Pi ,Div ,Mod, variables, ...
    pub fn handle_identifier(&mut self) -> Result<String, CompilerError> {
        let mut identifier = String::new();

        // Ensure the first character is valid for the start of an identifier
        if let Some(c) = self.current_char() {
            if c.is_digit(10) {
                return Err(CompilerError::InvalidIdentifierStart(
                    self.line,
                    self.current_pos,
                ));
            } else if Self::is_identifier_start(&c) {
                identifier.push(c);
                self.advance();
            }
        }

        // Indicates that a variable can contain numbers, just not in the beginning
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

    pub fn handle_number(&mut self) -> Result<TokenKind, CompilerError> {
        let start_pos = self.current_pos;
        let mut number_str = String::new();
        let mut has_dot = false;

        // Parse the main number part (digits with optional dot)
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                number_str.push(c);
                self.advance();
            } else if c == '.' && !has_dot {
                number_str.push(c);
                has_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        // Check for scientific notation (e.g., 1.3E+2)
        let mut is_scientific = false;
        if let Some('E') = self.current_char() {
            number_str.push('E'); // Include 'E' in the number string
            self.advance();
            is_scientific = true;

            // Optional sign after 'E'
            if let Some(sign) = self.current_char() {
                if sign == '+' || sign == '-' {
                    number_str.push(sign);
                    self.advance();
                }
            }

            // Parse exponent digits
            let mut has_exponent_digits = false;
            while let Some(c) = self.current_char() {
                if c.is_ascii_digit() {
                    number_str.push(c);
                    has_exponent_digits = true;
                    self.advance();
                } else {
                    break;
                }
            }

            // Ensure the scientific notation is valid
            if !has_exponent_digits {
                // TODO: InvalidMantissa in the compiler error
                return Err(CompilerError::InvalidNumber(
                    number_str, self.line, start_pos,
                ));
            }
        }

        // Handle the parsed number
        if is_scientific {
            // Return scientific notation as a Mantiss token
            Ok(TokenKind::Mantissa(number_str))
        } else if has_dot {
            // Handle floating-point numbers
            match number_str.parse::<f64>() {
                Ok(float_num) => Ok(TokenKind::Number(Num::Float(float_num))),
                Err(_) => Err(CompilerError::InvalidNumber(
                    number_str, self.line, start_pos,
                )),
            }
        } else {
            // Handle integers
            match number_str.parse::<i64>() {
                Ok(int_num) => Ok(TokenKind::Number(Num::Integer(int_num))),
                Err(_) => Err(CompilerError::InvalidNumber(
                    number_str, self.line, start_pos,
                )),
            }
        }
    }
}
