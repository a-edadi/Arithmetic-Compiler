use super::{get_and_parse_user_input, ASTWrapper, Num};

#[allow(unused_assignments)]
impl ASTWrapper {
    /// Find a single root
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
    ) -> Result<Vec<f64>, String> {
        let a = get_and_parse_user_input("a");
        let b = get_and_parse_user_input("b");

        if a >= b {
            return Err("Invalid interval: `a` must be less than `b`.".to_string());
        }

        let mut roots = Vec::new();

        let mut left = a;
        while left < b {
            let right = (left + step_size).min(b);

            let f_left = self.evaluate_with_x(left)?;
            let f_right = self.evaluate_with_x(right)?;

            if f_left * f_right < 0.0 {
                match self.find_root_bisection(left, right, tolerance, max_iterations) {
                    Ok(root) => {
                        let rounded_root = root.round();
                        if !roots
                            .iter()
                            .any(|&r: &f64| (r - rounded_root).abs() <= tolerance)
                        {
                            roots.push(rounded_root);
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

        if roots.is_empty() {
            Err("No roots found in the given interval.".to_string())
        } else {
            Ok(roots)
        }
    }

    pub fn evaluate_with_x(&mut self, x: f64) -> Result<f64, String> {
        self.vars.set_variable_value("x".to_string(), Num::Float(x));
        self.ast
            .evaluate(&mut self.vars)
            .map_err(|e| format!("Evaluation error: {:?}", e))
    }
}
