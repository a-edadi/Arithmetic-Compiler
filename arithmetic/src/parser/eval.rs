use super::{ASTNode, TokenKind};
use crate::errors::CompilerError;
use crate::lexer::token::Num;
use std::f64::consts::PI;

/// Evaluates an AST and returns a single floating-point result
pub fn evaluate_ast(node: &ASTNode) -> Result<f64, CompilerError> {
    match node {
        ASTNode::Number(n) => match n {
            Num::Integer(i) => Ok(*i as f64),
            Num::Float(f) => Ok(*f),
        },

        ASTNode::BinaryOp(left, op, right) => {
            let left_val = evaluate_ast(left)?; // Recursively evaluate left operand
            let right_val = evaluate_ast(right)?; // Recursively evaluate right operand
            match op {
                TokenKind::Plus => Ok(left_val + right_val),
                TokenKind::Minus => Ok(left_val - right_val),
                TokenKind::Multiply => Ok(left_val * right_val),
                TokenKind::Divide => {
                    if right_val == 0.0 {
                        Err(CompilerError::EvalDivisionByZero)
                    } else {
                        Ok(left_val / right_val)
                    }
                }
                TokenKind::Mod => {
                    // Ensure both left and right values are integers for modulus
                    if left_val.fract() != 0.0 || right_val.fract() != 0.0 {
                        Err(CompilerError::IntegerOperatorWithFloatOperands)
                    } else {
                        Ok(left_val % right_val)
                    }
                }
                TokenKind::Div => {
                    // Ensure both left and right values are integers for integer division
                    if left_val.fract() != 0.0 || right_val.fract() != 0.0 {
                        Err(CompilerError::IntegerOperatorWithFloatOperands)
                    } else if right_val == 0.0 {
                        Err(CompilerError::EvalDivisionByZero)
                    } else {
                        Ok((left_val / right_val).floor())
                    }
                }
                TokenKind::Power => Ok(left_val.powf(right_val)), // Exponentiation
                _ => Err(CompilerError::UnsupportedBinaryOperator(op.to_string())),
            }
        }

        ASTNode::UnaryOp(op, expr) => {
            let expr_val = evaluate_ast(expr)?;
            match op {
                TokenKind::Minus => Ok(-expr_val),
                TokenKind::Plus => Ok(expr_val),
                _ => Err(CompilerError::UnsupportedUnaryOperator(op.to_string())),
            }
        }

        ASTNode::FunctionCall(func, arg) => {
            let arg_val = evaluate_ast(arg)?; // Evaluate argument
            match func.as_str() {
                "sin" => Ok(arg_val.to_radians().sin()),
                "cos" => Ok(arg_val.to_radians().cos()),
                "tan" => Ok(arg_val.to_radians().tan()),
                "arctan" => {
                    let r = arg_val.atan();
                    Ok(r.to_degrees())
                }
                "arccotan" => {
                    let r = PI / 2.0 - arg_val.atan();
                    Ok(r.to_degrees())
                }
                "ln" => Ok(arg_val.ln()),
                "log" => Ok(arg_val.log10()),
                "exp" => Ok(arg_val.exp()),
                "sqrt" => Ok(arg_val.sqrt()),
                "sqr" => Ok(arg_val * arg_val),
                _ => Err(CompilerError::UnsupportedFunction(func.to_string())),
            }
        }

        // unreachable
        ASTNode::Constant(_) => Err(CompilerError::TryEvalUnreachable("Constants".to_string())),
        ASTNode::Mantiss(_) => Err(CompilerError::TryEvalUnreachable("Mantiss".to_string())),
        ASTNode::Identifier(id) => Err(CompilerError::TryEvalUnreachable(id.to_string())),
        // ASTNode::Constant(TokenKind::Euler) => Err(format!("Value not found")),
        // ASTNode::Constant(TokenKind::Pi) => Err(format!("Value not found")),
    }
}
