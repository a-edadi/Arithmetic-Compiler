use super::{ASTNode, TokenKind};
use std::f64::consts::{E, PI};

/// Evaluates an AST and returns a single floating-point result
pub fn evaluate_ast(node: &ASTNode) -> Result<f64, String> {
    match node {
        ASTNode::Number(n) => Ok(*n),

        ASTNode::Identifier(id) => Err(format!("variable '{}'", id)),

        ASTNode::Constant(TokenKind::E) => Ok(E),
        ASTNode::Constant(TokenKind::Pi) => Ok(PI),

        ASTNode::BinaryOp(left, op, right) => {
            let left_val = evaluate_ast(left)?; // Recursively evaluate left operand
            let right_val = evaluate_ast(right)?; // Recursively evaluate right operand
            match op {
                TokenKind::Plus => Ok(left_val + right_val),
                TokenKind::Minus => Ok(left_val - right_val),
                TokenKind::Multiply => Ok(left_val * right_val),
                TokenKind::Divide => {
                    if right_val == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(left_val / right_val)
                    }
                }
                TokenKind::Mod => Ok(left_val % right_val),
                TokenKind::Div => Ok((left_val / right_val).floor()), // Integer division
                TokenKind::Power => Ok(left_val.powf(right_val)),     // Exponentiation
                _ => Err(format!("Unsupported binary operator: {:?}", op)),
            }
        }

        ASTNode::UnaryOp(op, expr) => {
            let expr_val = evaluate_ast(expr)?;
            match op {
                TokenKind::Minus => Ok(-expr_val),
                TokenKind::Plus => Ok(expr_val),
                _ => Err(format!("Unsupported unary operator: {:?}", op)),
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
                _ => Err(format!("Unsupported function: {}", func)),
            }
        }

        ASTNode::Constant(_) => unreachable!("Unhandled constant case"),
    }
}
