use super::TokenKind;

/// AST Node definition
#[derive(Debug)]
pub enum ASTNode {
    Number(f64),                                     // A number node
    Identifier(String),                              // A variable or function name
    BinaryOp(Box<ASTNode>, TokenKind, Box<ASTNode>), // A binary operation node using TokenKind
    UnaryOp(TokenKind, Box<ASTNode>),                // A unary operation node
    FunctionCall(String, Box<ASTNode>),              // Function call node (name, argument)
    Constant(f64),                                   // Constant node (E or Pi)
}

pub fn print_ast(node: &ASTNode, prefix: String, is_left: bool) {
    match node {
        // Printing a number node
        ASTNode::Number(n) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, n);
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
            print_ast(left, new_prefix.clone(), true); // Print left operand
            print_ast(right, new_prefix, false); // Print right operand
        }
        // Printing a unary operation node (like sin(x), -x, etc.)
        ASTNode::UnaryOp(op, expr) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, op);
            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            print_ast(expr, new_prefix, false); // Print the operand for unary operations
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
            print_ast(arg, new_prefix, false); // Print the argument for the function
        }
    }
}
