use super::token::{Num, TokenKind};
use super::CompilerError;
use crate::Lexer;
use std::io::{self, Write};

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

    pub fn handle_number(&mut self) -> Result<Num, CompilerError> {
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

        // Parse as Float if it has a decimal point, else as Integer
        if has_dot {
            if let Ok(float_num) = number_str.parse::<f64>() {
                Ok(Num::Float(float_num))
            } else {
                Err(CompilerError::InvalidNumber(
                    number_str, self.line, start_pos,
                ))
            }
        } else {
            if let Ok(int_num) = number_str.parse::<i64>() {
                Ok(Num::Integer(int_num))
            } else {
                Err(CompilerError::InvalidNumber(
                    number_str, self.line, start_pos,
                ))
            }
        }
    }
}

/// Handling the comments
impl<'a> Lexer<'a> {
    /// Exclude the rest of the line when // is seen
    pub fn handle_line_comment(&mut self) {
        while let Some(c) = self.current_char() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    // handle block comments
    pub fn handle_block_comment(&mut self) {
        self.advance(); // Skip the initial '{'
        while let Some(c) = self.current_char() {
            if c == '}' {
                self.advance(); // Skip the closing '}'
                break;
            }
            self.advance();
        }
    }

    /// Prompt for variable value and return as Num
    pub fn prompt_for_variable_value(&mut self, var_name: &str) -> Num {
        // Check if the variable has already been assigned a value
        if let Some(value) = self.variables.get(var_name) {
            return value.clone(); // Return existing value
        }

        println!("Enter value for variable '{}':", var_name);
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            // Check if the input has a decimal to determine if it's Float or Integer
            let trimmed_input = input.trim();
            if trimmed_input.contains('.') {
                match trimmed_input.parse::<f64>() {
                    Ok(value) => {
                        let num_value = Num::Float(value);
                        self.variables
                            .insert(var_name.to_string(), num_value.clone());
                        return num_value;
                    }
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                }
            } else {
                match trimmed_input.parse::<i64>() {
                    Ok(value) => {
                        let num_value = Num::Integer(value);
                        self.variables
                            .insert(var_name.to_string(), num_value.clone());
                        return num_value;
                    }
                    Err(_) => println!("Invalid input. Please enter a valid number."),
                }
            }
        }
    }
}
