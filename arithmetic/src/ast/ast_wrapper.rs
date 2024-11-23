#![allow(dead_code)]
use super::{
    root_finder::RootFinder, ASTNode, CompilerError, Evaluator, FunctionPlotter, VariableManager,
};
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

    /// clear the Variable Manager values
    pub fn clear_variables(&mut self) {
        self.vars.clear();
    }

    /// returns AST tree string representation.
    pub fn ast_string(&mut self) -> String {
        self.ast.stringify("".to_string(), false)
    }

    /// returns ast tree postfix representation.
    pub fn ast_postfix_string(&mut self) -> String {
        self.ast.postfix()
    }

    /// evaluate with a certain value for x
    pub fn evaluate_with_x(&mut self, x: f64) -> Result<f64, CompilerError> {
        let mut evaluator = Evaluator::new(&mut self.vars);
        evaluator.evaluate_with_x(&self.ast, x)
    }

    /// evaluate the ast tree using a stack
    pub fn eval_ast(&mut self) -> Result<f64, CompilerError> {
        let mut evaluator = Evaluator::new(&mut self.vars);

        match evaluator.evaluate(&self.ast) {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    /// Returns a string representation of the roots found within an interval
    pub fn roots_string(
        &mut self,
        a: Option<f64>,
        b: Option<f64>,
    ) -> Result<String, CompilerError> {
        let mut evaluator = Evaluator::new(&mut self.vars);
        let mut root_finder = RootFinder::new(&self.ast, &mut evaluator);

        let roots = root_finder.find_roots(a, b)?;

        let roots_str = roots
            .iter()
            .map(|root| format!("{:.3}", root))
            .collect::<Vec<String>>()
            .join(", ");
        Ok(roots_str)
    }

    /// plot the function within the given interval.
    pub fn plot(&mut self, a: Option<f64>, b: Option<f64>) -> Result<(), CompilerError> {
        let mut plotter = FunctionPlotter::new(&mut self.vars);
        plotter.plot_function(&self.ast, a, b)
    }
}
