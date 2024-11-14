use crate::lexer::token::Num;

use super::ASTNode;

pub fn print_ast_tree(node: &ASTNode, prefix: String, is_left: bool) {
    match node {
        // Printing a number node
        ASTNode::Number(n) => {
            let number_str = match n {
                Num::Integer(i) => i.to_string(),
                Num::Float(f) => f.to_string(),
            };
            println!(
                "{}{}{}",
                prefix,
                if is_left { "├── " } else { "└── " },
                number_str
            );
        }
        // Printing an identifier node
        ASTNode::Identifier(id) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, id);
        }
        // Printing a constant node (E or Pi)
        ASTNode::Constant(c) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, c);
        }
        // Printing a binary operation node (like +, -, *, /, etc.)
        ASTNode::BinaryOp(left, op, right) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, op);
            // Adjust the prefix for child nodes
            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            print_ast_tree(left, new_prefix.clone(), true); // Print left operand
            print_ast_tree(right, new_prefix, false); // Print right operand
        }
        // Printing a unary operation node (like sin(x), -x, etc.)
        ASTNode::UnaryOp(op, expr) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, op);
            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            print_ast_tree(expr, new_prefix, false); // Print the operand for unary operations
        }
        // Printing a function call node (e.g., sin(x), sqrt(x), etc.)
        ASTNode::FunctionCall(func, arg) => {
            println!(
                "{}{}{}",
                prefix,
                if is_left { "├── " } else { "└── " },
                func
            );
            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            print_ast_tree(arg, new_prefix, false); // Print the argument for the function
        }
    }
}
