pub mod ast;
pub mod eval;
pub mod postfix;
pub mod utils;

use crate::errors::CompilerError;
use crate::lexer::{
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

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.lexer.reset();
        self.current_token = self.lexer.get_next_token().unwrap();
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
    pub fn parse_term(&mut self) -> Result<ASTNode, CompilerError> {
        // Start with parsing the first factor
        let mut node = self.parse_exponentiation()?;

        // Parse multiplication, division, mod, and div, which are left-associative
        while matches!(
            self.current_token.kind,
            TokenKind::Multiply | TokenKind::Divide | TokenKind::Mod | TokenKind::Div
        ) {
            let op = self.current_token.kind.clone();
            self.advance()?;
            let right_node = self.parse_exponentiation()?;

            // Handle division by zero
            if let (TokenKind::Divide, ASTNode::Number(0.0)) = (&op, &right_node) {
                return Err(CompilerError::DivisionByZero(self.lexer.get_position()));
            }

            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node));
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
            self.advance()?;
            let right_node = self.parse_exponentiation()?; // Recur to handle right-associativity

            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node));
        }

        Ok(node)
    }
    pub fn parse_factor(&mut self) -> Result<ASTNode, CompilerError> {
        match &self.current_token.kind {
            // Handle unary minus (negation)
            TokenKind::Minus => {
                self.advance()?;
                let operand = self.parse_factor()?;
                Ok(ASTNode::UnaryOp(TokenKind::Minus, Box::new(operand)))
            }

            // Handle unary plus
            TokenKind::Plus => {
                self.advance()?;
                let operand = self.parse_factor()?;
                Ok(operand)
            }

            // Number literal
            TokenKind::Number(n) => {
                let value = *n;
                self.advance()?;
                // Check if the next token is also a number without an operator between them
                if let TokenKind::Number(_) = self.current_token.kind {
                    return Err(CompilerError::MissingOperator(Some(
                        "2 consecutive numbers were passed without an operator".to_string(),
                    )));
                }
                Ok(ASTNode::Number(value))
            }

            // Handle constants (E and Pi) by replacing actual values
            TokenKind::E => {
                self.advance()?;
                Ok(ASTNode::Constant(TokenKind::E))
                // Ok(ASTNode::Constant(std::f64::consts::E))
            }
            TokenKind::Pi => {
                self.advance()?;
                Ok(ASTNode::Constant(TokenKind::Pi))
                // Ok(ASTNode::Constant(std::f64::consts::PI))
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
                        self.lexer.get_position(),
                        self.lexer.get_line(),
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
                            self.lexer.get_position(),
                            self.lexer.get_line(),
                        ))
                    }
                };

                Ok(ASTNode::FunctionCall(
                    func_name.to_string(),
                    Box::new(argument),
                ))
            }

            // Handle identifiers
            TokenKind::Identifier(id) => {
                let identifier = id.clone();
                self.advance()?;
                Ok(ASTNode::Identifier(identifier))
            }

            // Parentheses
            TokenKind::LeftParen => {
                self.advance()?;
                let node = self.parse_expression()?;
                if self.current_token.kind != TokenKind::RightParen {
                    return Err(CompilerError::MissingToken(")".to_string()));
                }
                self.advance()?; // Skip ')'
                Ok(node)
            }
            TokenKind::RightParen => Err(CompilerError::MissingToken("(".to_string())),

            // Unexpected token
            _ => Err(CompilerError::UnexpectedToken(
                self.current_token.kind.clone(),
                self.lexer.get_position(),
                self.lexer.get_line(),
            )),
        }
    }
}
