pub mod comments;
pub mod controllers;
pub mod get_values;
pub mod handlers;
pub mod print;
pub mod span;
pub mod token;
pub mod utils;

use crate::errors::CompilerError;
use span::TextSpan;
use std::collections::HashMap;
use token::{Num, Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
    line: usize,
    set_variable_values: bool,
    variables: HashMap<String, Num>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, set_values: bool) -> Self {
        Self {
            input,
            current_pos: 0,
            line: 1,
            variables: HashMap::new(),
            set_variable_values: set_values,
        }
    }

    pub fn get_next_token(&mut self) -> Result<Token, CompilerError> {
        self.skip_whitespace();

        // Ensure that the lexer is in bound and not lexing out of range
        if self.current_pos >= self.input.len() {
            let eof_char: char = '\0';
            return Ok(Token::new(
                TokenKind::Eof,
                TextSpan::new(self.current_pos, self.current_pos, eof_char.to_string()),
            ));
        }

        let c = match self.current_char() {
            Some(ch) => ch,
            None => {
                return Err(CompilerError::InvalidCharacter(
                    '\0',
                    self.line,
                    self.current_pos,
                ))
            }
        };

        // Define span start
        let start = self.current_pos;

        // Check for line comments
        if c == '/' && self.peek_char() == Some('/') {
            self.handle_line_comment();
            return self.get_next_token();
        }

        // Check for block comments
        if c == '{' {
            self.handle_block_comment();
            return self.get_next_token();
        }

        // Matching
        let kind = if Self::is_number_start(&c) {
            let number_kind = self
                .handle_number()
                .map_err(|_| CompilerError::UnexpectedError(self.line, self.current_pos))?;

            // If the next character is part of an identifier -> Raise Error
            if let Some(next_char) = self.current_char() {
                if Self::is_identifier_start(&next_char) {
                    return Err(CompilerError::InvalidIdentifierStart(
                        self.line,
                        self.current_pos,
                    ));
                }
            }

            // return Token kind
            number_kind
        } else if Self::is_identifier_start(&c) {
            let identifier = self.handle_identifier()?;
            let identifier_lower = identifier.to_lowercase();

            match identifier_lower.as_str() {
                "f" => TokenKind::Func,
                "sin" => TokenKind::Sin,
                "cos" => TokenKind::Cos,
                "tan" => TokenKind::Tan,
                "cotan" => TokenKind::Cotan,
                "arcsin" => TokenKind::ArcSin,
                "arccos" => TokenKind::ArcCos,
                "arctan" => TokenKind::ArcTan,
                "arccotan" => TokenKind::ArcCotan,
                "ln" => TokenKind::Ln,
                "log" => TokenKind::Log,
                "exp" => TokenKind::Exp,
                "sqrt" => TokenKind::Sqrt,
                "sqr" => TokenKind::Sqr,
                "div" => TokenKind::Div,
                "mod" => TokenKind::Mod,

                // Constants
                "e" => {
                    if self.set_variable_values {
                        TokenKind::Number(Num::Float(std::f64::consts::E))
                    } else {
                        TokenKind::Euler
                    }
                }
                "pi" => {
                    if self.set_variable_values {
                        TokenKind::Number(Num::Float(std::f64::consts::PI))
                    } else {
                        TokenKind::Pi
                    }
                }
                // Identifiers: All are considered variable in this context
                _ => {
                    if self.set_variable_values {
                        let value = self.get_variable_value(&identifier_lower);
                        TokenKind::Number(value)
                    } else {
                        TokenKind::Identifier(identifier)
                    }
                }
            }
        } else if Self::is_ascii_start(&c) {
            self.handle_punctuation()?
        } else {
            return Err(CompilerError::UnexpectedError(self.line, self.current_pos));
        };

        let end = self.current_pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);

        Ok(Token::new(kind, span))
    }
}
