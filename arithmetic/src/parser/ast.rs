#![allow(dead_code)]
use super::{
    roots::RootFinder, CompilerError, Evaluator, FunctionPlotter, Num, TextSpan, TokenKind,
    VariableManager,
};

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

pub struct ASTWrapper {
    pub ast: ASTNode,
    pub vars: VariableManager,
}

impl ASTWrapper {
    pub fn new(tree: ASTNode) -> Self {
        Self {
            ast: tree,
            vars: VariableManager::new(),
        }
    }

    pub fn evaluate_ast(&mut self) -> Result<f64, CompilerError> {
        let mut evaluator = Evaluator::new(&mut self.vars);

        match evaluator.evaluate(&self.ast) {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub fn find_roots(&mut self, a: Option<f64>, b: Option<f64>) -> Result<String, CompilerError> {
        let mut evaluator = Evaluator::new(&mut self.vars);
        let mut root_finder = RootFinder::new(&self.ast, &mut evaluator);

        root_finder.roots_string(a, b)
    }



    pub fn plot_function(&mut self) -> Result<(), CompilerError> {
        let mut plotter = FunctionPlotter::new(&self.ast, &mut self.vars);
        plotter.plot_function()
    }

    pub fn evaluate_with_x(&mut self, x: f64) -> Result<f64, CompilerError> {
        let mut evaluator = Evaluator::new(&mut self.vars);
        evaluator.evaluate_with_x(&self.ast, x)
    }

    pub fn ast_string(&mut self) -> String {
        self.ast.stringify("".to_string(), false)
    }

    pub fn ast_postfix_string(&mut self) -> String {
        self.ast.postfix()
    }
}
