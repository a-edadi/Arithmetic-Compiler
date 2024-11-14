use crate::lexer::token::Num;

use super::TokenKind;

#[derive(Debug)]
pub enum ASTNode {
    Number(Num),                                     // A number node
    Identifier(String),                              // A variable or function name
    BinaryOp(Box<ASTNode>, TokenKind, Box<ASTNode>), // A binary operation node using TokenKind
    UnaryOp(TokenKind, Box<ASTNode>),                // A unary operation node
    FunctionCall(String, Box<ASTNode>),              // Function call node (name, argument)
    Constant(TokenKind),                             // Pi and Euler
}
