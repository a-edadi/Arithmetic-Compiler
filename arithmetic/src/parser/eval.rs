use super::{ASTNode, ASTWrapper, CompilerError, EvaluationError, Num, TextSpan, TokenKind};
use std::f64::consts::{E, PI};

impl ASTWrapper {
    pub fn evaluate(&mut self) -> Result<f64, CompilerError> {
        // Clone or temporarily take ownership of the AST to avoid conflicts
        let ast = self.ast.clone();

        // Process the cloned AST
        self.process_node(&ast)?;

        // Get the final result
        self.get_result()
    }

    pub fn process_node(&mut self, node: &ASTNode) -> Result<(), CompilerError> {
        match node {
            ASTNode::Number(n, _) => self.process_number(n),
            ASTNode::Constant(token, span) => self.process_constant(token, span),
            ASTNode::Identifier(id, span) => self.process_identifier(id, span),
            ASTNode::Mantissa(value, span) => self.process_mantissa(value, span),
            ASTNode::BinaryOp(left, op, right, span) => {
                self.process_binary_op(left, op, right, span)
            }
            ASTNode::UnaryOp(op, expr, span) => self.process_unary_op(op, expr, span),
            ASTNode::FunctionCall(func, arg, span) => self.process_function_call(func, arg, span),
        }
    }

    fn process_number(&mut self, n: &Num) -> Result<(), CompilerError> {
        match n {
            Num::Integer(i) => self.stack.push_back(*i as f64),
            Num::Float(f) => self.stack.push_back(*f),
        }
        Ok(())
    }

    fn process_constant(
        &mut self,
        token: &TokenKind,
        span: &TextSpan,
    ) -> Result<(), CompilerError> {
        match token {
            TokenKind::Pi => {
                self.stack.push_back(PI);
                Ok(())
            }
            TokenKind::Euler => {
                self.stack.push_back(E);
                Ok(())
            }
            _ => Err(CompilerError::Eval(EvaluationError::InvalidConstant(
                span.line, span.start,
            ))),
        }
    }

    fn process_identifier(&mut self, id: &str, _: &TextSpan) -> Result<(), CompilerError> {
        let value = self.vars.get_variable_value(id);

        match value {
            Num::Integer(i) => self.stack.push_back(i as f64),
            Num::Float(f) => self.stack.push_back(f),
        }
        Ok(())
    }

    fn process_mantissa(&mut self, value: &str, span: &TextSpan) -> Result<(), CompilerError> {
        let parsed_value = value.parse::<f64>().map_err(|_| {
            CompilerError::Eval(EvaluationError::InvalidMantissa(span.line, span.start))
        })?;
        self.stack.push_back(parsed_value);
        Ok(())
    }

    fn process_binary_op(
        &mut self,
        left: &ASTNode,
        op: &TokenKind,
        right: &ASTNode,
        span: &TextSpan,
    ) -> Result<(), CompilerError> {
        self.process_node(left)?;
        let left_val = self
            .stack
            .pop_back()
            .ok_or_else(|| CompilerError::GenericError(span.line, span.start))?;

        self.process_node(right)?;
        let right_val = self
            .stack
            .pop_back()
            .ok_or_else(|| CompilerError::GenericError(span.line, span.start))?;

        let result = match op {
            TokenKind::Plus => left_val + right_val,
            TokenKind::Minus => left_val - right_val,
            TokenKind::Multiply => left_val * right_val,
            TokenKind::Divide => self.handle_division(left_val, right_val, span)?,
            TokenKind::Mod => self.handle_modulo(left_val, right_val, span)?,
            TokenKind::Div => self.handle_integer_division(left_val, right_val, span)?,
            TokenKind::Power => left_val.powf(right_val),
            _ => {
                return Err(CompilerError::Eval(
                    EvaluationError::UnsupportedBinaryOperator(
                        op.to_string(),
                        span.line,
                        span.start,
                    ),
                ))
            }
        };

        self.stack.push_back(result);
        Ok(())
    }

    fn handle_division(
        &self,
        left_val: f64,
        right_val: f64,
        span: &TextSpan,
    ) -> Result<f64, CompilerError> {
        if right_val == 0.0 {
            Err(CompilerError::Eval(EvaluationError::DivisionByZero(
                span.line, span.start,
            )))
        } else {
            Ok(left_val / right_val)
        }
    }

    fn handle_modulo(
        &self,
        left_val: f64,
        right_val: f64,
        span: &TextSpan,
    ) -> Result<f64, CompilerError> {
        if left_val.fract() != 0.0 || right_val.fract() != 0.0 {
            Err(CompilerError::Eval(
                EvaluationError::IntegerOperatorWithFloatOperands(span.line, span.start),
            ))
        } else {
            Ok(left_val % right_val)
        }
    }

    fn handle_integer_division(
        &self,
        left_val: f64,
        right_val: f64,
        span: &TextSpan,
    ) -> Result<f64, CompilerError> {
        if left_val.fract() != 0.0 || right_val.fract() != 0.0 {
            Err(CompilerError::Eval(
                EvaluationError::IntegerOperatorWithFloatOperands(span.line, span.start),
            ))
        } else if right_val == 0.0 {
            Err(CompilerError::Eval(EvaluationError::DivisionByZero(
                span.line, span.start,
            )))
        } else {
            Ok((left_val / right_val).floor())
        }
    }

    fn process_unary_op(
        &mut self,
        op: &TokenKind,
        expr: &ASTNode,
        span: &TextSpan,
    ) -> Result<(), CompilerError> {
        self.process_node(expr)?;

        let val = self
            .stack
            .pop_back()
            .ok_or_else(|| CompilerError::GenericError(span.line, span.start))?;

        let result = match op {
            TokenKind::Minus => -val,
            TokenKind::Plus => val,
            _ => {
                return Err(CompilerError::Eval(
                    EvaluationError::UnsupportedUnaryOperator(
                        op.to_string(),
                        span.line,
                        span.start,
                    ),
                ))
            }
        };

        self.stack.push_back(result);
        Ok(())
    }

    fn process_function_call(
        &mut self,
        func: &str,
        arg: &ASTNode,
        span: &TextSpan,
    ) -> Result<(), CompilerError> {
        self.process_node(arg)?;

        let arg_val = self
            .stack
            .pop_back()
            .ok_or_else(|| CompilerError::GenericError(span.line, span.start))?;

        let result = match func {
            "sin" => arg_val.to_radians().sin(),
            "cos" => arg_val.to_radians().cos(),
            "tan" => arg_val.to_radians().tan(),
            "arctan" => arg_val.atan().to_degrees(),
            "arccotan" => (PI / 2.0 - arg_val.atan()).to_degrees(),
            "ln" => arg_val.ln(),
            "log" => arg_val.log10(),
            "exp" => arg_val.exp(),
            "sqrt" => arg_val.sqrt(),
            "sqr" => arg_val * arg_val,
            _ => {
                return Err(CompilerError::Eval(EvaluationError::UnsupportedFunction(
                    func.to_string(),
                    span.line,
                )))
            }
        };

        self.stack.push_back(result);
        Ok(())
    }

    pub fn get_result(&mut self) -> Result<f64, CompilerError> {
        self.stack
            .pop_back()
            .ok_or(CompilerError::GenericError(0, 0))
    }
}
