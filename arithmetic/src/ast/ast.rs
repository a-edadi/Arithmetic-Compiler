use super::{Num, TextSpan, TokenKind};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(Num, TextSpan),
    BinaryOp(Box<ASTNode>, TokenKind, Box<ASTNode>, TextSpan),
    UnaryOp(TokenKind, Box<ASTNode>, TextSpan),
    Mantissa(String, TextSpan),
    Constant(TokenKind, TextSpan),
    Identifier(String, TextSpan),
    FunctionCall(String, Box<ASTNode>, TextSpan),
}
