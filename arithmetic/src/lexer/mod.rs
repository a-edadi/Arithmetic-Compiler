pub mod handlers;
pub mod text;
pub mod token;
pub mod utils;

use crate::errors::CompilerError;
use text::TextSpan;
use token::{Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
    line: usize,
    // variables: HashMap<String, f64>,
}

impl<'a> Lexer<'a> {
    /// Used to initialize a new lexer
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
            line: 1,
            // variables: HashMap::new(),
        }
    }

    /// Returns Current Char
    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    /// Advances to the next position also returns the current char before moving the position
    pub fn advance(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        if c == Some('\n') {
            self.line += 1;
        }

        c
    }

    /// Returns the next char without moving the position of the lexer
    pub fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    pub fn get_position(&self) -> usize {
        self.current_pos
    }

    /// resets the lexer position so the input can be lexed again without the need to re initialize
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.current_pos = 0;
        self.line = 1;
    }

    pub fn get_next_token(&mut self) -> Result<Token, CompilerError> {
        self.skip_whitespace();

        if self.current_pos >= self.input.len() {
            // Handle end-of-file case
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
        // used for span start and end
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

        let kind = if Self::is_number_start(&c) {
            let number = self
                .handle_number()
                .map_err(|_| CompilerError::UnexpectedError(self.line, self.current_pos))?;

            // After parsing the number, check if the next character is part of an invalid identifier
            if let Some(next_char) = self.peek_char() {
                if Self::is_identifier_start(&next_char) {
                    return Err(CompilerError::InvalidIdentifierStart(
                        self.line,
                        self.current_pos,
                    ));
                }
            }

            TokenKind::Number(number)
        } else if Self::is_identifier_start(&c) {
            let identifier = self.handle_identifier();
            let identifier_lower = identifier?.to_lowercase();

            match identifier_lower.as_str() {
                "func" => TokenKind::Func,
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
                "e" => TokenKind::E,
                "pi" => TokenKind::Pi,
                "div" => TokenKind::Div,
                "mod" => TokenKind::Remainder,
                _ => TokenKind::Identifier,
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
