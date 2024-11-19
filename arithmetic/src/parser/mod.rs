pub mod ast;
pub mod ast_string;
pub mod eval;
pub mod postfix;
pub mod postfix_eval;
pub mod print;
pub mod var_manager;

use crate::errors::CompilerError;
use crate::lexer::{
    span::TextSpan,
    token::{Num, Token, TokenKind},
    Lexer,
};
use ast::{ASTNode, ASTWrapper};
use var_manager::VariableManager;

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
            let right_node = self.parse_exponentiation()?; // Recursion to handle right-associativity

            node = ASTNode::BinaryOp(Box::new(node), op, Box::new(right_node), span);
        }

        Ok(node)
    }

    pub fn parse_factor(&mut self) -> Result<ASTNode, CompilerError> {
        let span = self.current_token.span.clone();

        match &self.current_token.kind {
            // Handle unary minus (negation)
            TokenKind::Minus => {
                self.advance()?;
                let operand = self.parse_factor()?;
                Ok(ASTNode::UnaryOp(TokenKind::Minus, Box::new(operand), span))
            }

            // Handle unary plus
            TokenKind::Plus => {
                self.advance()?;
                let operand = self.parse_factor()?;
                Ok(operand)
            }

            // Handle Number literal
            TokenKind::Number(n) => {
                let value = n.clone();
                self.advance()?;

                // Check if the next token is also a number without an operator between them
                if let TokenKind::Number(_) = self.current_token.kind {
                    return Err(CompilerError::MissingOperator(
                        self.lexer.line,
                        self.lexer.pos,
                    ));
                }

                Ok(ASTNode::Number(value, span))
            }

            // Handle Mantiss
            TokenKind::Mantissa(mantiss_str) => {
                let mantiss_value = mantiss_str.clone();
                self.advance()?;
                Ok(ASTNode::Mantissa(mantiss_value, span))
            }

            // Handle Euler's number(e)
            TokenKind::Euler => {
                self.advance()?;
                Ok(ASTNode::Constant(TokenKind::Euler, span))
            }

            // Handle Pi
            TokenKind::Pi => {
                self.advance()?;
                Ok(ASTNode::Constant(TokenKind::Pi, span))
            }

            // Handle functions: Sin, Cos, Tan, ...
            TokenKind::Sin
            | TokenKind::Cos
            | TokenKind::Tan
            | TokenKind::Cotan
            | TokenKind::Arcsin
            | TokenKind::Arccos
            | TokenKind::Arctan
            | TokenKind::Arccotan
            | TokenKind::Ln
            | TokenKind::Log
            | TokenKind::Exp
            | TokenKind::Sqrt
            | TokenKind::Sqr => {
                let func_name = self.current_token.kind.clone();

                self.advance()?;

                // after function there should be parens like sin(
                // if there is no left paren Raise error, we are expecting LeftParen
                if self.current_token.kind != TokenKind::LeftParen {
                    return Err(CompilerError::MissingLParen(
                        self.lexer.pos,
                        self.lexer.line,
                    ));
                }

                self.advance()?; // Skip '('

                // Parse Inner expression
                let argument = self.parse_expression()?;

                // after parsing the inner expression we are looking for ) RightParen
                if self.current_token.kind != TokenKind::RightParen {
                    return Err(CompilerError::MissingRParen(
                        self.lexer.line,
                        self.lexer.pos,
                    ));
                }

                self.advance()?; // Skip ')'

                let func_name = match func_name {
                    TokenKind::Sin => "sin",
                    TokenKind::Cos => "cos",
                    TokenKind::Tan => "tan",
                    TokenKind::Cotan => "cotan",
                    TokenKind::Arcsin => "arcsin",
                    TokenKind::Arccos => "arccos",
                    TokenKind::Arctan => "arctan",
                    TokenKind::Arccotan => "arccotan",
                    TokenKind::Ln => "ln",
                    TokenKind::Log => "log",
                    TokenKind::Exp => "exp",
                    TokenKind::Sqrt => "sqrt",
                    TokenKind::Sqr => "sqr",
                    _ => {
                        return Err(CompilerError::UnexpectedToken(
                            self.current_token.kind.clone(),
                            self.lexer.pos,
                            self.lexer.line,
                        ))
                    }
                };

                // Add the node to the Tree
                Ok(ASTNode::FunctionCall(
                    func_name.to_string(),
                    Box::new(argument),
                    span,
                ))
            }

            // Handle identifiers
            TokenKind::Identifier(id) => {
                let identifier = id.clone();
                self.advance()?;
                Ok(ASTNode::Identifier(identifier, span))
            }

            // Parentheses
            TokenKind::LeftParen => {
                self.advance()?;
                let node = self.parse_expression()?;
                if self.current_token.kind != TokenKind::RightParen {
                    return Err(CompilerError::MissingRParen(
                        self.lexer.line,
                        self.lexer.pos,
                    ));
                }
                self.advance()?; // Skip ')'
                Ok(node)
            }

            // handle single right paren, Missing left paren
            TokenKind::RightParen => Err(CompilerError::MissingLParen(
                self.lexer.line,
                self.lexer.pos,
            )),

            // Unexpected token
            _ => Err(CompilerError::UnexpectedToken(
                self.current_token.kind.clone(),
                self.lexer.pos,
                self.lexer.line,
            )),
        }
    }
}
