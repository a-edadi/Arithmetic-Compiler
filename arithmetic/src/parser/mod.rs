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

    // Parse an expression and build the corresponding AST
    pub fn parse_expression(&mut self) -> Result<ASTNode, CompilerError> {
        let mut node = self.parse_term()?; // Start with the first term

        // Handle addition and subtraction
        while self.current_token.kind == TokenKind::Plus
            || self.current_token.kind == TokenKind::Minus
        {
            let op = self.current_token.kind.clone(); // Clone the operator kind
            self.advance()?;
            let right_node = self.parse_term()?;
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node)); // Create a binary operation node
        }

        Ok(node)
    }

    // Parse a term (handles * and / operations)
    pub fn parse_term(&mut self) -> Result<ASTNode, CompilerError> {
        let mut node = self.parse_factor()?; // Start with the first factor

        // Handle multiplication and division
        while self.current_token.kind == TokenKind::Multiply
            || self.current_token.kind == TokenKind::Divide
        {
            let op = self.current_token.kind.clone(); // Clone the operator kind
            self.advance()?;
            let right_node = self.parse_factor()?;

            // Handle division by zero
            if let (TokenKind::Divide, ASTNode::Number(0.0)) = (&op, &right_node) {
                return Err(CompilerError::DivisionByZero(self.lexer.get_position()));
            }

            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node)); // Create a binary operation node
        }

        Ok(node)
    }

    // Parse a factor (a number or a parenthesized expression)
    pub fn parse_factor(&mut self) -> Result<ASTNode, CompilerError> {
        match &self.current_token.kind {
            // Handle a number
            TokenKind::Number(n) => {
                let value = *n; // Use the number directly (n is already i64 or f64)
                self.advance()?;

                // Check if the next token is also a number without an operator between them
                if let TokenKind::Number(_) = self.current_token.kind {
                    return Err(CompilerError::MissingOperator(Some(
                        "2 consecutive numbers were passed without an operator".to_string(),
                    )));
                }

                Ok(ASTNode::Number(value as f64))
            }
            // Handle parenthesized expression
            TokenKind::LeftParen => {
                self.advance()?; // Skip '('
                let node = self.parse_expression()?; // Parse inner expression
                if self.current_token.kind != TokenKind::RightParen {
                    return Err(CompilerError::MissingToken(")".to_string())); // Check for closing ')'
                }
                self.advance()?; // Skip ')'
                Ok(node)
            }
            // Handle unmatched closing parenthesis
            TokenKind::RightParen => Err(CompilerError::MissingToken("(".to_string())),
            // Handle unexpected token
            _ => Err(CompilerError::UnexpectedToken(
                self.current_token.kind.clone(),
            )),
        }
    }
}
