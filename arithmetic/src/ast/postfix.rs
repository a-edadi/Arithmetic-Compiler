use super::{ASTNode, Num, TokenKind};

/// Converts the AST into a postfix notation string.
impl ASTNode {
    pub fn postfix(&self) -> String {
        match self {
            // Handle a number node
            ASTNode::Number(n, _) => {
                let number_str = match n {
                    Num::Integer(i) => i.to_string(),
                    Num::Float(f) => f.to_string(),
                };
                format!("{} ", number_str)
            }

            // Handle a mantissa node
            ASTNode::Mantissa(mantiss_str, _) => {
                format!("{} ", mantiss_str)
            }

            // Handle an identifier node
            ASTNode::Identifier(id, _) => format!("{} ", id),

            // Handle a constant node: Euler's Number(e) or Pi
            ASTNode::Constant(c, _) => format!("{} ", c),

            // Handle a binary operation node: left operand, operator, right operand
            ASTNode::BinaryOp(left, op, right, _) => {
                let mut result = String::new();

                // Right-associative handling for exponentiation operator(^)
                if *op == TokenKind::Power {
                    result.push_str(&left.postfix());
                    result.push_str(&right.postfix());
                } else {
                    // Left-associative order for other operators
                    result.push_str(&left.postfix());
                    result.push_str(&right.postfix());
                }

                result.push_str(&format!("{} ", op));
                result
            }

            // Handle a unary operation node: operator, operand
            ASTNode::UnaryOp(op, expr, _) => {
                let mut result = expr.postfix(); // Operand
                result.push_str(&format!("{} ", op));
                result
            }

            // Handle a function call: sin(x), sqrt(x)
            ASTNode::FunctionCall(func, arg, _) => {
                let mut result = arg.postfix(); // Argument
                result.push_str(&format!("{} ", func)); // Append function name
                result
            }
        }
    }
}
