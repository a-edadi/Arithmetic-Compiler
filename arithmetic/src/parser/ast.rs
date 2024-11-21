use super::{Num, TextSpan, TokenKind, VariableManager};
use std::collections::VecDeque;

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

// Wrapper for the ASTNode to have a built in Variable Manager

pub struct ASTWrapper {
    pub ast: ASTNode,
    pub stack: VecDeque<f64>,
    pub vars: VariableManager,
}

impl ASTWrapper {
    pub fn new(tree: ASTNode) -> Self {
        Self {
            ast: tree,
            stack: VecDeque::new(),
            vars: VariableManager::new(),
        }
    }
}
