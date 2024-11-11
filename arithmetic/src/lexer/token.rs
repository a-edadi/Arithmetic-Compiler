use crate::lexer::text::TextSpan;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(i64),

    // Operators
    Plus,      // Addition operator: +
    Minus,     // Subtraction operator: -
    Multiply,  // Multiplication operator: *
    Divide,    // Division operator: /
    Remainder, // Modulus operator: %
    Power,     // Exponentiation operator: ^

    // Separators
    LeftParen,  // (
    RightParen, // )
    OpenBrace,  // {
    CloseBrace, // }
    Comma,      // ,

    // Identifier
    Func,
    Identifier,

    // Other
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token: {:?}\n  Span: {:?}\n  Literal: \"{}\" \n",
            self.kind, self.span, self.span.literal
        )
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Number(num) => write!(f, "Number({})", num),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Multiply => write!(f, "Multiply"),
            TokenKind::Divide => write!(f, "Divide"),
            TokenKind::Remainder => write!(f, "Remainder"),
            TokenKind::Power => write!(f, "Power"),
            TokenKind::LeftParen => write!(f, "LeftParen"),
            TokenKind::RightParen => write!(f, "RightParen"),
            TokenKind::OpenBrace => write!(f, "OpenBrace"),
            TokenKind::CloseBrace => write!(f, "CloseBrace"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Func => write!(f, "Func"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}
