use super::var::VariableManager;
use super::{ASTNode, CompilerError, Num, TokenKind};

use std::f64::consts::PI;
impl ASTNode {
    pub fn evaluate(&self, var_manager: &mut VariableManager) -> Result<f64, CompilerError> {
        match self {
            ASTNode::Identifier(id, _) => match var_manager.get_variable_value(id) {
                Num::Integer(i) => Ok(i as f64),
                Num::Float(f) => Ok(f),
            },

            ASTNode::Number(n, _) => match n {
                Num::Integer(i) => Ok(*i as f64),
                Num::Float(f) => Ok(*f),
            },

            ASTNode::BinaryOp(left, op, right, span) => {
                let left_val = left.evaluate(var_manager)?; // Recursively evaluate left operand
                let right_val = right.evaluate(var_manager)?; // Recursively evaluate right operand
                match op {
                    TokenKind::Plus => Ok(left_val + right_val),
                    TokenKind::Minus => Ok(left_val - right_val),
                    TokenKind::Multiply => Ok(left_val * right_val),
                    TokenKind::Divide => {
                        if right_val == 0.0 {
                            Err(CompilerError::EvalDivisionByZero(span.line))
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    TokenKind::Mod => {
                        if left_val.fract() != 0.0 || right_val.fract() != 0.0 {
                            Err(CompilerError::IntegerOperatorWithFloatOperands(
                                span.line,
                                span.column,
                            ))
                        } else {
                            Ok(left_val % right_val)
                        }
                    }
                    TokenKind::Div => {
                        if left_val.fract() != 0.0 || right_val.fract() != 0.0 {
                            Err(CompilerError::IntegerOperatorWithFloatOperands(
                                span.line,
                                span.column,
                            ))
                        } else if right_val == 0.0 {
                            Err(CompilerError::EvalDivisionByZero(span.line))
                        } else {
                            Ok((left_val / right_val).floor())
                        }
                    }
                    TokenKind::Power => Ok(left_val.powf(right_val)),
                    _ => Err(CompilerError::UnsupportedBinaryOperator(
                        op.to_string(),
                        span.line,
                    )),
                }
            }

            ASTNode::UnaryOp(op, expr, span) => {
                let expr_val = expr.evaluate(var_manager)?;
                match op {
                    TokenKind::Minus => Ok(-expr_val),
                    TokenKind::Plus => Ok(expr_val),
                    _ => Err(CompilerError::UnsupportedUnaryOperator(
                        op.to_string(),
                        span.line,
                    )),
                }
            }

            ASTNode::FunctionCall(func, arg, span) => {
                let arg_val = arg.evaluate(var_manager)?;
                match func.as_str() {
                    "sin" => Ok(arg_val.to_radians().sin()),
                    "cos" => Ok(arg_val.to_radians().cos()),
                    "tan" => Ok(arg_val.to_radians().tan()),
                    "arctan" => Ok(arg_val.atan().to_degrees()),
                    "arccotan" => Ok((PI / 2.0 - arg_val.atan()).to_degrees()),
                    "ln" => Ok(arg_val.ln()),
                    "log" => Ok(arg_val.log10()),
                    "exp" => Ok(arg_val.exp()),
                    "sqrt" => Ok(arg_val.sqrt()),
                    "sqr" => Ok(arg_val * arg_val),
                    _ => Err(CompilerError::UnsupportedFunction(
                        func.to_string(),
                        span.line,
                    )),
                }
            }

            ASTNode::Constant(token, _) => match token {
                TokenKind::Pi => Ok(PI),
                TokenKind::Euler => Ok(std::f64::consts::E),
                _ => Err(CompilerError::Unexpected()),
                // invalid constant
            },

            ASTNode::Mantiss(value, _) => match value.parse::<f64>() {
                // invalid mantissa
                Ok(parsed_value) => Ok(parsed_value),
                Err(_) => Err(CompilerError::Unexpected()),
            },
        }
    }

    pub fn eval_result(&self, var_manager: &mut VariableManager) -> String {
        match self.evaluate(var_manager) {
            Ok(result) => format!("Evaluation result: {}", result),
            Err(e) => format!("{}", e),
        }
    }
}
