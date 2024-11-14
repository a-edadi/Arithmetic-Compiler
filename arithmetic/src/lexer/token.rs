use crate::lexer::text::TextSpan;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]

pub enum Num {
    Integer(i64),
    Float(f64),
}
#[derive(Debug, Clone, PartialEq)]

pub enum TokenKind {
    // Literals
    Number(Num),

    // Identifier
    Func,
    Identifier(String),

    // Operators
    Plus,     // Addition operator: +
    Minus,    // Subtraction operator: -
    Multiply, // Multiplication operator: *
    Divide,   // Division operator: /
    Div,      // Integer division operator: div
    Mod,      // Modulus operator: mod
    Power,    // Exponentiation operator: ^

    // Separators
    LeftParen,  // (
    RightParen, // )

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
    Euler,
    Pi,

    // Other
    Eof,
    // Scientific Notations
    // Mantis,
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
            TokenKind::Number(num) => write!(f, "Number({:?})", num),
            TokenKind::Plus => write!(f, "+"),
            // TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "-"),
            // TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Multiply => write!(f, "*"),
            // TokenKind::Multiply => write!(f, "Multiply"),
            TokenKind::Divide => write!(f, "/"),
            // TokenKind::Divide => write!(f, "Divide"),
            TokenKind::Div => write!(f, "div"),
            // TokenKind::Div => write!(f, "Div"),
            TokenKind::Mod => write!(f, "mod"),
            // TokenKind::Mod => write!(f, "Modulus"),
            TokenKind::Power => write!(f, "^"),
            // TokenKind::Power => write!(f, "Power"),
            TokenKind::LeftParen => write!(f, "("),
            // TokenKind::LeftParen => write!(f, "LeftParen"),
            TokenKind::RightParen => write!(f, ")"),
            // TokenKind::RightParen => write!(f, "RightParen"),
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
            // TokenKind::Mantis => write!(f, "E"),
            TokenKind::Euler => write!(f, "e"),
            TokenKind::Pi => write!(f, "π"),
            TokenKind::Func => write!(f, "Func"),
            TokenKind::Identifier(string) => write!(f, "Identifier({})", string),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}
