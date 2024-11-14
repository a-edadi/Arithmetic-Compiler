use super::{ASTNode, Num, TokenKind};

/// Converts the AST into a postfix notation string.
impl ASTNode {
    pub fn postfix(&self) -> String {
        match self {
            // Handle a number node
            ASTNode::Number(n) => {
                let number_str = match n {
                    Num::Integer(i) => i.to_string(),
                    Num::Float(f) => f.to_string(),
                };
                format!("{} ", number_str)
            }

            // Handle a mantissa node
            ASTNode::Mantiss(mantiss_str) => {
                format!("{} ", mantiss_str)
            }

            // Handle an identifier node
            ASTNode::Identifier(id) => format!("{} ", id),

            // Handle a constant node: Euler's Number(e) or Pi
            ASTNode::Constant(c) => format!("{} ", c),

            // Handle a binary operation node: left operand, operator, right operand
            ASTNode::BinaryOp(left, op, right) => {
                let mut result = String::new();

                // Right-associative handling for exponentiation operator(^)
                if *op == TokenKind::Power {
                    result.push_str(&left.postfix()); // Left operand first
                    result.push_str(&right.postfix()); // Right operand
                } else {
                    // Left-associative order for other operators
                    result.push_str(&left.postfix()); // Left operand
                    result.push_str(&right.postfix()); // Right operand
                }

                result.push_str(&format!("{} ", op)); // Append operator
                result
            }

            // Handle a unary operation node: operator, operand
            ASTNode::UnaryOp(op, expr) => {
                let mut result = expr.postfix(); // Operand
                result.push_str(&format!("{} ", op)); // Append operator
                result
            }

            // Handle a function call: sin(x), sqrt(x)
            ASTNode::FunctionCall(func, arg) => {
                let mut result = arg.postfix(); // Argument
                result.push_str(&format!("{} ", func)); // Append function name
                result
            }
        }
    }
}
