pub mod controllers;
pub mod handlers;
pub mod span;
pub mod token;
pub mod utils;

use crate::errors::{lexer::LexerError, CompilerError};
use span::TextSpan;
use token::{Num, Token, TokenKind};

/// Lexer: lexes the input and returns token stream
pub struct Lexer<'a> {
    input: &'a str,
    pub pos: usize,
    pub line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
            column: 0,
        }
    }

    /// The core lexer logic
    pub fn get_next_token(&mut self) -> Result<Token, CompilerError> {
        self.skip_whitespace();

        // Ensure that the lexer is in bound.
        if self.pos >= self.input.len() {
            let eof_char: char = '\0';
            return Ok(Token::new(
                TokenKind::Eof,
                TextSpan::new(
                    self.pos,
                    self.pos,
                    eof_char.to_string(),
                    self.line,
                    self.column,
                ),
            ));
        }

        // fetch the current char
        let c = match self.current_char() {
            Some(ch) => ch,
            None => {
                return Err(CompilerError::Lex(LexerError::InvalidCharacter(
                    '\0', self.line, self.pos,
                )))
            }
        };

        // Define Span values
        let start = self.pos;
        let line = self.line;
        let column = self.column;

        // Check for line comments
        if c == '/' && self.peek() == Some('/') {
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
                .map_err(|_| CompilerError::GenericError(self.line, self.pos))?;

            // If the next character is part of an identifier -> Raise Error
            if let Some(next_char) = self.current_char() {
                if Self::is_identifier_start(&next_char) {
                    return Err(CompilerError::Lex(LexerError::InvalidIdentifier(
                        self.line, self.pos,
                    )));
                }
            }

            // return Token kind
            number_kind
        } else if Self::is_identifier_start(&c) {
            let identifier = self.handle_identifier()?;
            let identifier_lower = identifier.to_lowercase();

            match identifier_lower.as_str() {
                "sin" => TokenKind::Sin,
                "cos" => TokenKind::Cos,
                "tan" => TokenKind::Tan,
                "cotan" => TokenKind::Cotan,
                "arcsin" => TokenKind::Arcsin,
                "arccos" => TokenKind::Arccos,
                "arctan" => TokenKind::Arctan,
                "arccotan" => TokenKind::Arccotan,
                "ln" => TokenKind::Ln,
                "log" => TokenKind::Log,
                "exp" => TokenKind::Exp,
                "sqrt" => TokenKind::Sqrt,
                "sqr" => TokenKind::Sqr,
                "div" => TokenKind::Div,
                "mod" => TokenKind::Mod,
                "e" => TokenKind::Euler,
                "pi" => TokenKind::Pi,

                _ => {
                    if identifier_lower
                        .chars()
                        .next()
                        .map_or(false, |c| c.is_ascii_digit())
                    {
                        return Err(CompilerError::Lex(LexerError::InvalidIdentifier(
                            self.line, self.pos,
                        )));
                    } else {
                        TokenKind::Identifier(identifier)
                    }
                }
            }
        } else if Self::is_ascii_start(&c) {
            self.handle_punctuation()?
        } else {
            return Err(CompilerError::GenericError(self.line, self.pos));
        };

        let end = self.pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal, line, column);

        Ok(Token::new(kind, span))
    }
}
