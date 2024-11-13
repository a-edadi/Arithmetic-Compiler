use super::{ASTNode, TokenKind};

pub fn ast_to_postfix(node: &ASTNode) -> String {
    match node {
        // Handle a number node
        ASTNode::Number(n) => format!("{} ", n),

        // Handle an identifier node
        ASTNode::Identifier(id) => format!("{} ", id),

        // Handle a constant node (e.g., E or Pi)
        ASTNode::Constant(c) => format!("{} ", c),

        // Handle a binary operation node (left operand, operator, right operand)
        ASTNode::BinaryOp(left, op, right) => {
            let mut result = String::new();

            // Right-associative handling for exponentiation operator
            if *op == TokenKind::Power {
                result.push_str(&ast_to_postfix(left)); // Left operand first
                result.push_str(&ast_to_postfix(right)); // Right operand
            } else {
                // Left-associative order for other operators
                result.push_str(&ast_to_postfix(left)); // Left operand
                result.push_str(&ast_to_postfix(right)); // Right operand
            }

            result.push_str(&format!("{} ", op)); // Append operator
            result
        }

        // Handle a unary operation node (operator, operand)
        ASTNode::UnaryOp(op, expr) => {
            let mut result = ast_to_postfix(expr); // Operand
            result.push_str(&format!("{} ", op)); // Append operator
            result
        }

        // Handle a function call (e.g., sin(x), sqrt(x), etc.)
        ASTNode::FunctionCall(func, arg) => {
            let mut result = ast_to_postfix(arg); // Argument
            result.push_str(&format!("{} ", func)); // Append function name
            result
        }
    }
}
