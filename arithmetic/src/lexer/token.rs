use crate::lexer::span::TextSpan;
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

    // Identifier
    Func,
    Identifier(String),

    // Operators
    Plus,     // Addition operator: +
    Minus,    // Subtraction operator: -
    Multiply, // Multiplication operator: *
    Divide,   // Division operator: /
    Div,      // Integer division operator: div
    Mod,      // Modulo operator: mod
    Power,    // Exponentiation operator: ^

    // Separators
    LeftParen,  // (
    RightParen, // )

    // Mathematical functions
    Sin,      // Sine function: sin
    Cos,      // Cosine function: cos
    Tan,      // Tangent function: tan
    Cotan,    // Cotangent function: cotan
    ArcSin,   // Inverse sine function: arcsin
    ArcCos,   // Inverse cosine function: arccos
    ArcTan,   // Inverse tangent function: arctan
    ArcCotan, // Inverse cotangent function: arccotan
    Ln,       // Natural logarithm function: ln
    Log,      // Base-10 logarithm function: log
    Exp,      // Exponential function: exp(x) = e^x
    Sqrt,     // Square root function: sqrt(x) = √x
    Sqr,      // Squaring function: sqr(x) = x^2

    // Constants
    Euler, // Euler's number: e
    Pi,    // Pi: π

    // Other
    Eof, // End of file/input marker

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

// Handling the Display for Token, TokenKind and Num

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token: {:?}\n  Span: {:?}\n  Literal: \"{}\" \n",
            self.kind, self.span, self.span.literal
        )
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Num::Integer(i) => write!(f, "Integer({})", i),
            Num::Float(fl) => write!(f, "Float({})", fl),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Number(num) => write!(f, "Number({})", num),
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
