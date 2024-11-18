use super::{Num, TextSpan, TokenKind, VariableManager};

#[allow(dead_code)]
#[derive(Debug)]
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
    ast: ASTNode,
    vars: VariableManager,
}

impl ASTWrapper {
    pub fn new(tree: ASTNode) -> Self {
        Self {
            ast: tree,
            vars: VariableManager::new(),
        }
    }

    pub fn evaluate(&mut self) {
        match self.ast.evaluate(&mut self.vars) {
            Ok(result) => println!("Evaluation result: {}", result),
            Err(e) => eprintln!("Evaluation error: {}", e),
        }
    }
}
