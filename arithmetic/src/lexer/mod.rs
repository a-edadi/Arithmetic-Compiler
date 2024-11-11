mod errors;
mod text;
mod token;
mod utils;

use errors::LexerError;
use text::TextSpan;
use token::{Token, TokenKind};

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    /// Used to initialize a new lexer
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    /// Returns Current Char
    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    /// Advances to the next position also returns the current char before moving the position
    fn advance(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            return None;
        }
        let c = self.current_char();
        self.current_pos += 1;

        c
    }

    /// resets the lexer position so the input can be lexed again without the need to re initialize
    pub fn reset(&mut self) {
        self.current_pos = 0;
    }

    /// Utilities to categorize kind
    fn is_number_start(c: &char) -> bool {
        c.is_ascii_digit()
    }

    fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic() || c == &'_'
    }

    fn is_ascii_start(c: &char) -> bool {
        c.is_ascii()
    }

    /// Skip White spaces.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn lex_potential_double_char_operator(
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
    fn handle_punctuation(&mut self) -> Result<TokenKind, LexerError> {
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
            '{' => Ok(TokenKind::OpenBrace),
            '}' => Ok(TokenKind::CloseBrace),
            _ => Err(LexerError::InvalidCharacter(c, self.current_pos - 1)),
        }
    }

    /// tokenize the input so we can returns identifiers
    fn handle_identifier(&mut self) -> String {
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

    /// Handle numbers
    fn handle_number(&mut self) -> Result<i64, LexerError> {
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

    pub fn get_next_token(&mut self) -> Result<Token, LexerError> {
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
            None => return Err(LexerError::InvalidCharacter('\0', self.current_pos)),
        };

        let start = self.current_pos;

        let kind = if Self::is_number_start(&c) {
            let number = self
                .handle_number()
                .map_err(|_| LexerError::UnexpectedError(self.current_pos))?;
            TokenKind::Number(number)
        } else if Self::is_identifier_start(&c) {
            let identifier = self.handle_identifier();
            let identifier_lower = identifier.to_lowercase();
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
                _ => TokenKind::Identifier,
            }
        } else if Self::is_ascii_start(&c) {
            self.handle_punctuation()?
        } else {
            return Err(LexerError::UnexpectedError(self.current_pos));
        };

        let end = self.current_pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);

        Ok(Token::new(kind, span))
    }
}
