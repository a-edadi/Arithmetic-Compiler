use super::token::{Num, TokenKind};
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
    pub fn handle_number(&mut self) -> Result<TokenKind, CompilerError> {
        let start_pos = self.current_pos;
        let mut number_str = String::new();
        let mut has_dot = false;

        // Parse the mantissa part (base part of the number)
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

        // Check for scientific notation (e.g., "E+2")
        if let Some('E') = self.current_char() {
            number_str.push('E'); // Add E to the number string
            self.advance(); // Move past 'E'

            // Optional sign for the exponent part
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

        // Return based on `set_values`
        if self.set_variable_values {
            // Attempt to parse the full number as a float or integer
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
            // If `set_values` is false, store as a string mantissa
            Ok(TokenKind::Mantiss(number_str))
        }
    }

    // pub fn handle_number(&mut self) -> Result<Num, CompilerError> {
    //     let start_pos = self.current_pos;
    //     let mut number_str = String::new();
    //     let mut has_dot = false;

    //     while let Some(c) = self.current_char() {
    //         if c.is_ascii_digit() {
    //             number_str.push(c);
    //             self.advance();
    //         } else if c == '.' && !has_dot {
    //             // Allow only one decimal point
    //             number_str.push(c);
    //             has_dot = true;
    //             self.advance();
    //         } else {
    //             break;
    //         }
    //     }

    //     // Parse as Float if it has a decimal point, else as Integer
    //     if has_dot {
    //         if let Ok(float_num) = number_str.parse::<f64>() {
    //             Ok(Num::Float(float_num))
    //         } else {
    //             Err(CompilerError::InvalidNumber(
    //                 number_str, self.line, start_pos,
    //             ))
    //         }
    //     } else {
    //         if let Ok(int_num) = number_str.parse::<i64>() {
    //             Ok(Num::Integer(int_num))
    //         } else {
    //             Err(CompilerError::InvalidNumber(
    //                 number_str, self.line, start_pos,
    //             ))
    //         }
    //     }
    // }
}
