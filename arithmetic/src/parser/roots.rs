use super::{ast::ASTNode, get_and_parse_user_input, CompilerError, Evaluator, RootFinderError};

pub struct RootFinder<'a> {
    ast: &'a ASTNode,
    evaluator: &'a mut Evaluator<'a>,
}

#[allow(unused_assignments)]
impl<'a> RootFinder<'a> {
    pub fn new(ast: &'a ASTNode, evaluator: &'a mut Evaluator<'a>) -> Self {
        Self { ast, evaluator }
    }

    pub fn evaluate_with_x(&mut self, x: f64) -> Result<f64, CompilerError> {
        self.evaluator.evaluate_with_x(self.ast, x)
    }

    pub fn find_root_bisection(
        &mut self,
        a: f64,
        b: f64,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<f64, CompilerError> {
        if a >= b {
            return Err(CompilerError::Root(RootFinderError::InvalidInterval));
        }

        let mut left = a;
        let mut right = b;
        let mut iterations = 0;

        let mut f_left = self.evaluate_with_x(left)?;
        let mut f_right = self.evaluate_with_x(right)?;

        if f_left * f_right > 0.0 {
            return Err(CompilerError::Root(RootFinderError::NoRootInInterval));
        }

        while (right - left).abs() > tolerance && iterations < max_iterations {
            iterations += 1;

            let mid = (left + right) / 2.0;
            let f_mid = self.evaluate_with_x(mid)?;

            if f_mid.abs() <= tolerance {
                return Ok(mid);
            }

            if f_left * f_mid < 0.0 {
                right = mid;
                f_right = f_mid;
            } else {
                left = mid;
                f_left = f_mid;
            }
        }

        if iterations >= max_iterations {
            return Err(CompilerError::Root(RootFinderError::MaxIterationsReached));
        } else {
            Ok((left + right) / 2.0)
        }
    }

    pub fn find_all_roots(
        &mut self,
        tolerance: f64,
        max_iterations: usize,
        step_size: f64,
        a: f64,
        b: f64,
    ) -> Result<Vec<f64>, CompilerError> {
        if a >= b {
            return Err(CompilerError::Root(RootFinderError::InvalidInterval));
        }

        let mut roots = Vec::new();

        let expanded_a = a - (b - a) * 0.1;
        let expanded_b = b + (b - a) * 0.1;

        let mut left = expanded_a;
        while left < expanded_b {
            let right = (left + step_size).min(expanded_b);

            let f_left = match self.evaluate_with_x(left) {
                Ok(val) => val,
                Err(_) => {
                    left = right;
                    continue;
                }
            };

            let f_right = match self.evaluate_with_x(right) {
                Ok(val) => val,
                Err(_) => {
                    left = right;
                    continue;
                }
            };

            // Check for sign change and potential root
            if f_left * f_right <= 0.0 {
                match self.find_root_bisection(left, right, tolerance, max_iterations) {
                    Ok(root) => {
                        if root >= a && root <= b {
                            // let rounded_root = (root * 1000.0).round() / 1000.0;
                            let rounded_root = root.round();

                            if !roots
                                .iter()
                                .any(|&r| (r as f64 - rounded_root).abs() <= tolerance)
                            {
                                roots.push(rounded_root);
                            }
                        }
                    }
                    Err(e) => return Err(e),
                }
            }

            left = right;
        }

        Ok(roots)
    }

    /// More friendly, with defaults
    pub fn find_roots(
        &mut self,
        a: Option<f64>,
        b: Option<f64>,
    ) -> Result<Vec<f64>, CompilerError> {
        let a = a.unwrap_or_else(|| get_and_parse_user_input("a"));
        let b = b.unwrap_or_else(|| get_and_parse_user_input("b"));
        self.find_all_roots(1e-6, 1000, 0.1, a, b)
    }

    pub fn roots_string(
        &mut self,
        a: Option<f64>,
        b: Option<f64>,
    ) -> Result<String, CompilerError> {
        let roots = self.find_roots(a, b)?;

        let roots_str = roots
            .iter()
            .map(|root| format!("{:.3}", root))
            .collect::<Vec<String>>()
            .join(", ");

        Ok(roots_str)
    }
}
