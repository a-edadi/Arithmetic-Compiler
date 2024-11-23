use super::{ast::ASTNode, get_and_parse_user_input, CompilerError, Evaluator, RootFinderError};

pub struct RootFinder<'a> {
    ast: &'a ASTNode,                 // Expression to find roots for
    evaluator: &'a mut Evaluator<'a>, // Evaluator for f(x) = 0
}

#[allow(unused_assignments)]
impl<'a> RootFinder<'a> {
    pub fn new(ast: &'a ASTNode, evaluator: &'a mut Evaluator<'a>) -> Self {
        Self { ast, evaluator }
    }

    /// Evaluates the expression at a given x value
    pub fn evaluate_at(&mut self, x: f64) -> Result<f64, CompilerError> {
        self.evaluator.evaluate_with_x(self.ast, x)
    }

    /// Checks if there's a sign change between two points
    fn has_sign_change(&mut self, x1: f64, x2: f64) -> Result<bool, CompilerError> {
        let f1 = self.evaluate_at(x1)?;
        let f2 = self.evaluate_at(x2)?;
        Ok(f1 * f2 <= 0.0)
    }

    /// Validates if the interval [a, b] is valid
    fn validate_interval(a: f64, b: f64) -> Result<(), CompilerError> {
        if a >= b {
            Err(CompilerError::Root(RootFinderError::InvalidInterval))
        } else {
            Ok(())
        }
    }

    /// Expands interval by 10% on both sides
    fn expand_interval(a: f64, b: f64) -> (f64, f64) {
        let padding = (b - a) * 0.1;
        (a - padding, b + padding)
    }

    /// Rounds a value to 3 decimal places
    fn round_to_precision(value: f64) -> f64 {
        (value * 1000.0).round() / 1000.0
    }

    /// Checks if a value is within tolerance of any value in the list
    fn is_duplicate(value: f64, values: &[f64], tolerance: f64) -> bool {
        values.iter().any(|&x| (x - value).abs() <= tolerance)
    }

    /// Find a single root using bisection method
    pub fn find_root_bisection(
        &mut self,
        a: f64,
        b: f64,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<f64, CompilerError> {
        Self::validate_interval(a, b)?;

        let mut left = a;
        let mut right = b;
        let mut iterations = 0;

        if !self.has_sign_change(left, right)? {
            return Err(CompilerError::Root(RootFinderError::NoRootInInterval));
        }

        while (right - left).abs() > tolerance && iterations < max_iterations {
            iterations += 1;
            let mid = (left + right) / 2.0;
            let f_mid = self.evaluate_at(mid)?;

            if f_mid.abs() <= tolerance {
                return Ok(mid);
            }

            if self.has_sign_change(left, mid)? {
                right = mid;
            } else {
                left = mid;
            }
        }

        if iterations >= max_iterations {
            Err(CompilerError::Root(RootFinderError::MaxIterationsReached))
        } else {
            Ok((left + right) / 2.0)
        }
    }

    /// Find all roots in an interval
    pub fn find_all_roots(
        &mut self,
        tolerance: f64,
        max_iterations: usize,
        step_size: f64,
        a: f64,
        b: f64,
    ) -> Result<Vec<f64>, CompilerError> {
        Self::validate_interval(a, b)?;

        let mut roots = Vec::new();
        let (expanded_a, expanded_b) = Self::expand_interval(a, b);
        let mut left = expanded_a;

        while left < expanded_b {
            let right = (left + step_size).min(expanded_b);

            // Skip evaluation errors and continue with next interval
            if let Ok(true) = self.has_sign_change(left, right) {
                if let Ok(root) = self.find_root_bisection(left, right, tolerance, max_iterations) {
                    if root >= a && root <= b {
                        let rounded_root = Self::round_to_precision(root);
                        if !Self::is_duplicate(rounded_root, &roots, tolerance) {
                            roots.push(rounded_root);
                        }
                    }
                }
            }

            left = right;
        }

        Ok(roots)
    }

    /// User-friendly method with default parameters
    pub fn find_roots(
        &mut self,
        a: Option<f64>,
        b: Option<f64>,
    ) -> Result<Vec<f64>, CompilerError> {
        let a = a.unwrap_or_else(|| get_and_parse_user_input("a"));
        let b = b.unwrap_or_else(|| get_and_parse_user_input("b"));

        // Default parameters
        const TOLERANCE: f64 = 1e-6;
        const MAX_ITERATIONS: usize = 1000;
        const STEP_SIZE: f64 = 0.1;

        self.find_all_roots(TOLERANCE, MAX_ITERATIONS, STEP_SIZE, a, b)
    }
}
