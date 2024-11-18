use super::TextSpan;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Num {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(Num),
    Mantissa(String),

    // Identifier
    Func,
    Identifier(String),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Div,
    Mod,
    Power,

    // Separators and precedence
    LeftParen,
    RightParen,

    // Mathematical functions
    Sin,
    Cos,
    Tan,
    Cotan,
    Arcsin, // Standardized capitalization
    Arccos,
    Arctan,
    Arccotan,
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

// Handling the Display for Token, TokenKind and Num
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            // "Token: {:?}\n  Span: {:?}\n  Literal: \"{}\" \n", // Prettier
            // "Token: {:?} Span: {:?}  Literal: \"{}\"", // Just like debug print, Minimal space.
            "Token: {:?}  Literal: \"{}\"",
            self.kind, self.span.literal
        )
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Integer(i) => write!(f, "Integer({})", i),
            Num::Float(fl) => write!(f, "Float({:.5})", fl),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Number(num) => write!(f, "Number({})", num),
            TokenKind::Identifier(string) => write!(f, "Identifier({})", string),
            TokenKind::Func => write!(f, "Func"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Multiply => write!(f, "*"),
            TokenKind::Divide => write!(f, "/"),
            TokenKind::Div => write!(f, "div"),
            TokenKind::Mod => write!(f, "mod"),
            TokenKind::Power => write!(f, "^"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::Sin => write!(f, "Sin"),
            TokenKind::Cos => write!(f, "Cos"),
            TokenKind::Tan => write!(f, "Tan"),
            TokenKind::Cotan => write!(f, "Cotan"),
            TokenKind::Arcsin => write!(f, "ArcSin"),
            TokenKind::Arccos => write!(f, "ArcCos"),
            TokenKind::Arctan => write!(f, "ArcTan"),
            TokenKind::Arccotan => write!(f, "ArcCotan"),
            TokenKind::Ln => write!(f, "Ln"),
            TokenKind::Log => write!(f, "Log"),
            TokenKind::Exp => write!(f, "Exp"),
            TokenKind::Sqrt => write!(f, "Sqrt"),
            TokenKind::Sqr => write!(f, "Sqr"),
            TokenKind::Mantissa(num_str) => write!(f, "{}", num_str),
            TokenKind::Euler => write!(f, "e"),
            TokenKind::Pi => write!(f, "π"),
            TokenKind::Eof => write!(f, "End of Input"),
        }
    }
}
