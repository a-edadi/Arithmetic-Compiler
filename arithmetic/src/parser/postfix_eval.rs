use super::var_manager::VariableManager;
use super::{ASTNode, CompilerError, Num, TokenKind};
use std::collections::VecDeque;
use std::f64::consts::{E, PI};

impl ASTNode {
    pub fn evaluate_postfix(
        &self,
        var_manager: &mut VariableManager,
    ) -> Result<f64, CompilerError> {
        let mut stack: VecDeque<f64> = VecDeque::new();

        fn process_node(
            node: &ASTNode,
            stack: &mut VecDeque<f64>,
            var_manager: &mut VariableManager,
        ) -> Result<(), CompilerError> {
            match node {
                ASTNode::Number(n, _) => match n {
                    Num::Integer(i) => stack.push_back(*i as f64),
                    Num::Float(f) => stack.push_back(*f),
                },

                ASTNode::Constant(token, span) => match token {
                    TokenKind::Pi => stack.push_back(PI),
                    TokenKind::Euler => stack.push_back(E),
                    _ => return Err(CompilerError::InValidConstant(span.line, span.start)),
                },

                ASTNode::Identifier(id, _) => match var_manager.get_variable_value(id) {
                    Num::Integer(i) => stack.push_back(i as f64),
                    Num::Float(f) => stack.push_back(f),
                },

                ASTNode::Mantissa(value, span) => match value.parse::<f64>() {
                    Ok(parsed_value) => stack.push_back(parsed_value),
                    Err(_) => return Err(CompilerError::InvalidMantissa(span.line, span.start)),
                },

                ASTNode::BinaryOp(left, op, right, span) => {
                    process_node(left, stack, var_manager)?;
                    process_node(right, stack, var_manager)?;

                    if stack.len() < 2 {
                        return Err(CompilerError::UnexpectedError(span.line, span.start));
                    }

                    let right_val = stack.pop_back().unwrap();
                    let left_val = stack.pop_back().unwrap();

                    let result = match op {
                        TokenKind::Plus => left_val + right_val,
                        TokenKind::Minus => left_val - right_val,
                        TokenKind::Multiply => left_val * right_val,
                        TokenKind::Divide => {
                            if right_val == 0.0 {
                                return Err(CompilerError::DivisionByZero(span.line, span.start));
                            }
                            left_val / right_val
                        }
                        TokenKind::Mod => {
                            if left_val.fract() != 0.0 || right_val.fract() != 0.0 {
                                return Err(CompilerError::IntegerOperatorWithFloatOperands(
                                    span.line, span.start,
                                ));
                            }
                            left_val % right_val
                        }
                        TokenKind::Div => {
                            if left_val.fract() != 0.0 || right_val.fract() != 0.0 {
                                return Err(CompilerError::IntegerOperatorWithFloatOperands(
                                    span.line, span.start,
                                ));
                            }
                            if right_val == 0.0 {
                                return Err(CompilerError::DivisionByZero(span.line, span.start));
                            }
                            (left_val / right_val).floor()
                        }
                        TokenKind::Power => left_val.powf(right_val),
                        _ => {
                            return Err(CompilerError::UnsupportedBinaryOperator(
                                op.to_string(),
                                span.line,
                                span.start,
                            ))
                        }
                    };

                    stack.push_back(result);
                }

                ASTNode::UnaryOp(op, expr, span) => {
                    process_node(expr, stack, var_manager)?;

                    if stack.is_empty() {
                        return Err(CompilerError::UnexpectedError(span.line, span.start));
                    }

                    let val = stack.pop_back().unwrap();
                    let result = match op {
                        TokenKind::Minus => -val,
                        TokenKind::Plus => val,
                        _ => {
                            return Err(CompilerError::UnsupportedUnaryOperator(
                                op.to_string(),
                                span.line,
                                span.start,
                            ))
                        }
                    };

                    stack.push_back(result);
                }

                ASTNode::FunctionCall(func, arg, span) => {
                    process_node(arg, stack, var_manager)?;

                    if stack.is_empty() {
                        return Err(CompilerError::UnexpectedError(span.line, span.start));
                    }

                    let arg_val = stack.pop_back().unwrap();
                    let result = match func.as_str() {
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
                            return Err(CompilerError::UnsupportedFunction(
                                func.to_string(),
                                span.line,
                            ))
                        }
                    };

                    stack.push_back(result);
                }
            }
            Ok(())
        }

        process_node(self, &mut stack, var_manager)?;

        stack.pop_back().ok_or(CompilerError::UnexpectedError(0, 0))
    }
}
