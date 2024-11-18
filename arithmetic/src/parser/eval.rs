use super::var_manager::VariableManager;
use super::{ASTNode, CompilerError, Num, TokenKind};

use std::f64::consts::E;
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

                    // Check for Division By Zero: 1/0 -> Err
                    TokenKind::Divide => {
                        if right_val == 0.0 {
                            Err(CompilerError::DivisionByZero(span.line, span.start))
                        } else {
                            Ok(left_val / right_val)
                        }
                    }

                    // Handle Mod: 2 mod 3.2 or 3.2 mod 2 -> Err
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

                    // Handle Div: 2 div 3.2 or 3.2 div 2 or 2 div 0 -> Err
                    TokenKind::Div => {
                        if left_val.fract() != 0.0 || right_val.fract() != 0.0 {
                            Err(CompilerError::IntegerOperatorWithFloatOperands(
                                span.line,
                                span.column,
                            ))
                        } else if right_val == 0.0 {
                            Err(CompilerError::DivisionByZero(span.line, span.start))
                        } else {
                            Ok((left_val / right_val).floor())
                        }
                    }

                    // Handle Power ^
                    TokenKind::Power => Ok(left_val.powf(right_val)),
                    _ => Err(CompilerError::UnsupportedBinaryOperator(
                        op.to_string(),
                        span.line,
                        span.start,
                    )),
                }
            }

            // Unary Operators: +2, -2, ...
            ASTNode::UnaryOp(op, expr, span) => {
                let expr_val = expr.evaluate(var_manager)?;
                match op {
                    TokenKind::Minus => Ok(-expr_val),
                    TokenKind::Plus => Ok(expr_val),
                    _ => Err(CompilerError::UnsupportedUnaryOperator(
                        op.to_string(),
                        span.line,
                        span.start,
                    )),
                }
            }

            // Handle Function calls
            ASTNode::FunctionCall(func, arg, span) => {
                let arg_val = arg.evaluate(var_manager)?;
                match func.as_str() {
                    // Trig Functions
                    "sin" => Ok(arg_val.to_radians().sin()),
                    "cos" => Ok(arg_val.to_radians().cos()),
                    "tan" => Ok(arg_val.to_radians().tan()),
                    "arctan" => Ok(arg_val.atan().to_degrees()),
                    "arccotan" => Ok((PI / 2.0 - arg_val.atan()).to_degrees()),

                    // Other
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

            // Replace with the standard Pi and euler's number provided by rust
            ASTNode::Constant(token, span) => match token {
                TokenKind::Pi => Ok(PI),
                TokenKind::Euler => Ok(E),
                _ => Err(CompilerError::InValidConstant(span.line, span.start)),
            },

            // Parse the Mantiss string as a Number
            ASTNode::Mantissa(value, span) => match value.parse::<f64>() {
                Ok(parsed_value) => Ok(parsed_value),
                Err(_) => Err(CompilerError::InvalidMantissa(span.line, span.start)),
            },
        }
    }
}
