use super::{get_and_parse_user_input, ASTWrapper, Num};

#[allow(unused_assignments)]
impl ASTWrapper {
    pub fn evaluate_with_x(&mut self, x: f64) -> Result<f64, String> {
        // Set the variable "x"
        self.vars.set_variable_value("x".to_string(), Num::Float(x));
        let ast = self.ast.clone();

        // Evaluate the AST
        self.process_node(&ast)
            .map_err(|e| format!("Evaluation error: {:?}", e))?;

        // Get the result
        self.get_result()
            .map_err(|e| format!("Error retrieving result: {:?}", e))
    }

    pub fn find_root_bisection(
        &mut self,
        a: f64,
        b: f64,
        tolerance: f64,
        max_iterations: usize,
    ) -> Result<f64, String> {
        if a >= b {
            return Err("Invalid interval: `a` must be less than `b`.".to_string());
        }

        let mut left = a;
        let mut right = b;
        let mut iterations = 0;

        let mut f_left = self.evaluate_with_x(left)?;
        let mut f_right = self.evaluate_with_x(right)?;

        if f_left * f_right > 0.0 {
            return Err(
                "No root guaranteed in the interval (f(a) and f(b) must have opposite signs)."
                    .to_string(),
            );
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
            Err("Maximum iterations reached without finding a root.".to_string())
        } else {
            Ok((left + right) / 2.0)
        }
    }

    pub fn find_all_roots_bisection(
        &mut self,
        tolerance: f64,
        max_iterations: usize,
        step_size: f64,
        a: f64,
        b: f64,
    ) -> Result<Vec<f64>, String> {
        if a >= b {
            return Err("Invalid interval: `a` must be less than `b`.".to_string());
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
                            let rounded_root = (root * 1000.0).round() / 1000.0;
                            if !roots
                                .iter()
                                .any(|&r| (r as f64 - rounded_root).abs() <= tolerance)
                            {
                                roots.push(rounded_root);
                            }
                        }
                    }
                    Err(e) => eprintln!(
                        "Error finding root in interval ({}, {}): {}",
                        left, right, e
                    ),
                }
            }

            left = right;
        }

        Ok(roots)
    }

    pub fn find_roots(&mut self, a: Option<f64>, b: Option<f64>) -> Result<Vec<f64>, String> {
        let a = a.unwrap_or_else(|| get_and_parse_user_input("a"));
        let b = b.unwrap_or_else(|| get_and_parse_user_input("b"));
        self.find_all_roots_bisection(1e-6, 1000, 0.1, a, b)
    }
}
