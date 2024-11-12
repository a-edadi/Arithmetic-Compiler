pub mod ast;

use crate::errors::CompilerError;

use super::lexer::{
    token::{Token, TokenKind},
    Lexer,
};
use ast::ASTNode;

/// Parser
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Result<Self, CompilerError> {
        let current_token = lexer.get_next_token()?;
        Ok(Parser {
            lexer,
            current_token,
        })
    }

    // Parser Advance method: Calls the lexer to get the next token
    pub fn advance(&mut self) -> Result<(), CompilerError> {
        self.current_token = self.lexer.get_next_token()?; // Move to the next token
        Ok(())
    }

    pub fn parse_expression(&mut self) -> Result<ASTNode, CompilerError> {
        let mut node = self.parse_term()?; // Start with parsing a term

        while matches!(self.current_token.kind, TokenKind::Plus | TokenKind::Minus) {
            let op = self.current_token.kind.clone();
            self.advance()?;
            let right_node = self.parse_term()?;
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node));
        }

        Ok(node)
    }

    // Parse a term (handles *, /, and exponentiation)
    pub fn parse_term(&mut self) -> Result<ASTNode, CompilerError> {
        let mut node = self.parse_factor()?;

        while matches!(
            self.current_token.kind,
            TokenKind::Multiply | TokenKind::Divide | TokenKind::Power
        ) {
            let op = self.current_token.kind.clone();
            self.advance()?;
            let right_node = self.parse_factor()?;
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node));
        }

        Ok(node)
    }

    pub fn parse_factor(&mut self) -> Result<ASTNode, CompilerError> {
        match &self.current_token.kind {
            // Handle unary minus (negation)
            TokenKind::Minus => {
                self.advance()?; // Consume the minus
                let operand = self.parse_factor()?; // Parse the next factor (it can be a number, parentheses, etc.)
                Ok(ASTNode::UnaryOp(TokenKind::Minus, Box::new(operand))) // Return a unary negation node
            }

            // Handle unary plus (optional, but for completeness)
            TokenKind::Plus => {
                self.advance()?; // Consume the plus
                let operand = self.parse_factor()?; // Parse the next factor
                Ok(operand) // Just return the operand, as unary plus does not change the value
            }

            // Number literal
            TokenKind::Number(n) => {
                let value = *n;
                self.advance()?;
                Ok(ASTNode::Number(value))
            }

            // Handle constants (E and Pi)
            TokenKind::E => {
                self.advance()?;
                Ok(ASTNode::Constant(std::f64::consts::E))
            }
            TokenKind::Pi => {
                self.advance()?;
                Ok(ASTNode::Constant(std::f64::consts::PI))
            }

            // Handle functions (Sin, Cos, Tan, etc.)
            TokenKind::Sin
            | TokenKind::Cos
            | TokenKind::Tan
            | TokenKind::Cotan
            | TokenKind::ArcSin
            | TokenKind::ArcCos
            | TokenKind::ArcTan
            | TokenKind::ArcCotan
            | TokenKind::Ln
            | TokenKind::Log
            | TokenKind::Exp
            | TokenKind::Sqrt
            | TokenKind::Sqr => {
                let func_name = self.current_token.kind.clone();
                self.advance()?;

                if self.current_token.kind != TokenKind::LeftParen {
                    return Err(CompilerError::UnexpectedToken(
                        self.current_token.kind.clone(),
                    ));
                }
                self.advance()?; // Skip '('
                let argument = self.parse_expression()?;
                if self.current_token.kind != TokenKind::RightParen {
                    return Err(CompilerError::MissingToken(")".to_string()));
                }
                self.advance()?; // Skip ')'

                let func_name = match func_name {
                    TokenKind::Sin => "sin",
                    TokenKind::Cos => "cos",
                    TokenKind::Tan => "tan",
                    TokenKind::Cotan => "cotan",
                    TokenKind::ArcSin => "arcsin",
                    TokenKind::ArcCos => "arccos",
                    TokenKind::ArcTan => "arctan",
                    TokenKind::ArcCotan => "arccotan",
                    TokenKind::Ln => "ln",
                    TokenKind::Log => "log",
                    TokenKind::Exp => "exp",
                    TokenKind::Sqrt => "sqrt",
                    TokenKind::Sqr => "sqr",
                    _ => {
                        return Err(CompilerError::UnexpectedToken(
                            self.current_token.kind.clone(),
                        ))
                    }
                };

                Ok(ASTNode::FunctionCall(
                    func_name.to_string(),
                    Box::new(argument),
                ))
            }

            // Handle variables or identifiers
            TokenKind::Identifier(id) => {
                let identifier = id.clone();
                self.advance()?;
                Ok(ASTNode::Identifier(identifier))
            }

            // Parentheses for grouping expressions
            TokenKind::LeftParen => {
                self.advance()?;
                let node = self.parse_expression()?;
                if self.current_token.kind != TokenKind::RightParen {
                    return Err(CompilerError::MissingToken(")".to_string()));
                }
                self.advance()?; // Skip ')'
                Ok(node)
            }

            // Unexpected token
            _ => Err(CompilerError::UnexpectedToken(
                self.current_token.kind.clone(),
            )),
        }
    }
}
