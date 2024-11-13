use super::TokenKind;

/// AST Node definition
#[derive(Debug)]
pub enum ASTNode {
    Number(f64),                                     // A number node
    Identifier(String),                              // A variable or function name
    BinaryOp(Box<ASTNode>, TokenKind, Box<ASTNode>), // A binary operation node using TokenKind
    UnaryOp(TokenKind, Box<ASTNode>),                // A unary operation node
    FunctionCall(String, Box<ASTNode>),              // Function call node (name, argument)
    Constant(TokenKind),
}
