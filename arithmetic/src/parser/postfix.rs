use super::{ASTNode, TokenKind};

pub fn ast_to_postfix(node: &ASTNode) -> String {
    match node {
        // Handle a number node
        ASTNode::Number(n) => return n.to_string() + " ",
        
        // Handle an identifier node
        ASTNode::Identifier(id) => return id.clone() + " ",

        // Handle a constant node (e.g., E or Pi)
        ASTNode::Constant(c) => return c.to_string() + " ",

        // Handle a binary operation node (left operand, operator, right operand)
        ASTNode::BinaryOp(left, op, right) => {
            let mut result = String::new();

            // If operator is exponentiation (right-associative)
            if *op == TokenKind::Power {
                // Process the right operand first, then the left
                result.push_str(&ast_to_postfix(right)); // Right operand first
                result.push_str(&ast_to_postfix(left)); // Left operand second
            } else {
                // For other operators, process left operand first, then right
                result.push_str(&ast_to_postfix(left)); // Left operand
                result.push_str(&ast_to_postfix(right)); // Right operand
            }

            result.push_str(&format!("{} ", op)); // Add the operator at the end
            result
        }

        // Handle a unary operation node (operator, operand)
        ASTNode::UnaryOp(op, expr) => {
            let mut result = String::new();
            result.push_str(&ast_to_postfix(expr)); // Operand
            result.push_str(&format!("{} ", op)); // Operator
            result
        }
        
        // Handle a function call (e.g., sin(x), sqrt(x), etc.)
        ASTNode::FunctionCall(func, arg) => {
            let mut result = String::new();
            result.push_str(&ast_to_postfix(arg)); // Argument
            result.push_str(&format!("{} ", func)); // Function name
            result
        }
    }
}
