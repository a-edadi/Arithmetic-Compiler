use super::token::{Num, TokenKind};
use super::CompilerError;
use crate::Lexer;

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

    /// Handles Number: floats, Integers and Mantiss
    pub fn handle_number(&mut self) -> Result<TokenKind, CompilerError> {
        let start_pos = self.current_pos;
        let mut number_str = String::new();
        let mut has_dot = false;

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

        // Check for scientific notation: Mantiss(E)
        if let Some('E') = self.current_char() {
            number_str.push('E'); // Add E to the number string
            self.advance(); // Move past 'E'

            // Mantiss sign: E+2 or E-2
            if let Some(sign) = self.current_char() {
                if sign == '+' || sign == '-' {
                    number_str.push(sign);
                    self.advance();
                }
            }

            // Parse the exponent digits
            while let Some(c) = self.current_char() {
                if c.is_ascii_digit() {
                    number_str.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Return based on set_values
        if self.set_variable_values {
            // parse the full number as a float or integer
            if has_dot || number_str.contains('E') {
                match number_str.parse::<f64>() {
                    Ok(float_num) => Ok(TokenKind::Number(Num::Float(float_num))),
                    Err(_) => Err(CompilerError::InvalidNumber(
                        number_str, self.line, start_pos,
                    )),
                }
            } else {
                match number_str.parse::<i64>() {
                    Ok(int_num) => Ok(TokenKind::Number(Num::Integer(int_num))),
                    Err(_) => Err(CompilerError::InvalidNumber(
                        number_str, self.line, start_pos,
                    )),
                }
            }
        } else {
            // If set_values is false, store as a string mantissa
            Ok(TokenKind::Mantiss(number_str))
        }
    }
}
