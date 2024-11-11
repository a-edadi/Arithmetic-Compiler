use crate::lexer::text::TextSpan;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(i64),

    // Operators
    Plus,      // Addition operator: +
    Minus,     // Subtraction operator: -
    Multiply,  // Multiplication operator: *
    Divide,    // Division operator: /
    Div,       // Integer division operator: //
    Remainder, // Modulus operator: %

    Power, // Exponentiation operator: ^

    // Separators
    LeftParen,  // (
    RightParen, // )
    OpenBrace,  // {
    CloseBrace, // }

    // Functions
    Sin,
    Cos,
    Tan,
    Cotan,
    ArcSin,
    ArcCos,
    ArcTan,
    ArcCotan,
    Ln,
    Log,
    Exp,
    Sqrt,
    Sqr,

    // Constants
    E,
    Pi,

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
            TokenKind::Div => write!(f, "Div"),
            TokenKind::Remainder => write!(f, "Remainder"),
            TokenKind::Power => write!(f, "Power"),
            TokenKind::LeftParen => write!(f, "LeftParen"),
            TokenKind::RightParen => write!(f, "RightParen"),
            TokenKind::OpenBrace => write!(f, "OpenBrace"),
            TokenKind::CloseBrace => write!(f, "CloseBrace"),
            TokenKind::Sin => write!(f, "Sin"),
            TokenKind::Cos => write!(f, "Cos"),
            TokenKind::Tan => write!(f, "Tan"),
            TokenKind::Cotan => write!(f, "Cotan"),
            TokenKind::ArcSin => write!(f, "ArcSin"),
            TokenKind::ArcCos => write!(f, "ArcCos"),
            TokenKind::ArcTan => write!(f, "ArcTan"),
            TokenKind::ArcCotan => write!(f, "ArcCotan"),
            TokenKind::Ln => write!(f, "Ln"),
            TokenKind::Log => write!(f, "Log"),
            TokenKind::Exp => write!(f, "Exp"),
            TokenKind::Sqrt => write!(f, "Sqrt"),
            TokenKind::Sqr => write!(f, "Sqr"),
            TokenKind::E => write!(f, "E"),
            TokenKind::Pi => write!(f, "Pi"),
            TokenKind::Func => write!(f, "Func"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}
