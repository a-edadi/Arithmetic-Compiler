pub mod factor;

use crate::ast::ast::ASTNode;
use crate::errors::{parser::ParserError, CompilerError};
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::Lexer;

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

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.lexer.reset();
        self.current_token = self.lexer.get_next_token().unwrap();
    }

    // Parser Advance method: Calls the lexer to get the next token
    pub fn advance(&mut self) -> Result<(), CompilerError> {
        self.current_token = self.lexer.get_next_token()?;
        Ok(())
    }

    /// RDP starting point:
    pub fn parse_expression(&mut self) -> Result<ASTNode, CompilerError> {
        let mut node = self.parse_term()?; // Start with parsing a term

        while matches!(self.current_token.kind, TokenKind::Plus | TokenKind::Minus) {
            let op = self.current_token.kind.clone();
            let span = self.current_token.span.clone();
            self.advance()?;
            let right_node = self.parse_term()?;
            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node), span);
        }

        Ok(node)
    }

    pub fn parse_term(&mut self) -> Result<ASTNode, CompilerError> {
        // Start with parsing the right factor
        let mut node = self.parse_exponentiation()?;

        // Parse multiplication, division, mod, and div, which are left-associative
        while matches!(
            self.current_token.kind,
            TokenKind::Multiply | TokenKind::Divide | TokenKind::Mod | TokenKind::Div
        ) {
            let op = self.current_token.kind.clone();
            let span = self.current_token.span.clone();

            self.advance()?;
            let right_node = self.parse_exponentiation()?;

            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node), span);
        }

        Ok(node)
    }

    // Parse an exponentiation expression with right-associativity
    fn parse_exponentiation(&mut self) -> Result<ASTNode, CompilerError> {
        // Start by parsing the left factor
        let mut node = self.parse_factor()?;

        // Handle exponentiation as right-associative
        if self.current_token.kind == TokenKind::Power {
            let op = self.current_token.kind.clone();
            let span = self.current_token.span.clone();

            self.advance()?;
            let right_node = self.parse_exponentiation()?;

            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node), span);
        }

        Ok(node)
    }
}
