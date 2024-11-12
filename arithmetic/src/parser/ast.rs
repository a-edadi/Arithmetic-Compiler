use super::TokenKind;

/// AST Node definition
#[derive(Debug)]
pub enum ASTNode {
    Number(f64),                                     // A number node
    BinaryOp(Box<ASTNode>, TokenKind, Box<ASTNode>), // A binary operation node using TokenKind
}

pub fn print_ast(node: &ASTNode, prefix: String, is_left: bool) {
    match node {
        // Printing a number node
        ASTNode::Number(n) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, n);
        }
        // Printing a binary operation node
        ASTNode::BinaryOp(left, op, right) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, op);
            // Adjust the prefix for child nodes
            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            print_ast(left, new_prefix.clone(), true);
            print_ast(right, new_prefix, false);
        }
    }
}
