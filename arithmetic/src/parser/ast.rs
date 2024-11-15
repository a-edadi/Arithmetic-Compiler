use super::{Num, TokenKind};
#[allow(dead_code)]
#[derive(Debug)]
pub enum ASTNode {
    Number(Num, usize),         // A number node with span info
    Mantiss(String, usize),     // Mantissa representation as a string
    Constant(TokenKind, usize), // Constant node with span info
    Identifier(String, usize),  // Identifier with span info
    BinaryOp(Box<ASTNode>, TokenKind, Box<ASTNode>, usize), // Binary operation with span info
    UnaryOp(TokenKind, Box<ASTNode>, usize), // Unary operation with span info
    FunctionCall(String, Box<ASTNode>, usize), // Function call with span info
}

/// Recursive function that returns the tree structure as a string
impl ASTNode {
    pub fn to_string_tree(&self, prefix: String, is_left: bool) -> String {
        match self {
            // Formatting a number node
            ASTNode::Number(n, _) => {
                let number_str = match n {
                    Num::Integer(i) => i.to_string(),
                    Num::Float(f) => f.to_string(),
                };
                format!(
                    "{}{}{}\n",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    number_str
                )
            }
            // Formatting a mantissa node
            ASTNode::Mantiss(mantiss_str, _) => {
                format!(
                    "{}{}{}\n",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    mantiss_str
                )
            }
            // Formatting an identifier node
            ASTNode::Identifier(id, _) => {
                format!(
                    "{}{}{}\n",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    id
                )
            }
            // Formatting a constant node: E or Pi
            ASTNode::Constant(c, _) => {
                format!("{}{}{}\n", prefix, if is_left { "├── " } else { "└── " }, c)
            }
            // Formatting a binary operation node: +, -, *, /
            ASTNode::BinaryOp(left, op, right, _) => {
                let mut result = format!(
                    "{}{}{}\n",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    op
                );
                let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
                result.push_str(&left.to_string_tree(new_prefix.clone(), true)); // Left operand
                result.push_str(&right.to_string_tree(new_prefix, false)); // Right operand
                result
            }
            // Formatting a unary operation node: -x
            ASTNode::UnaryOp(op, expr, _) => {
                let mut result = format!(
                    "{}{}{}\n",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    op
                );
                let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
                result.push_str(&expr.to_string_tree(new_prefix, false));
                result
            }
            // Formatting a function call node: sin(x), sqrt(x)
            ASTNode::FunctionCall(func, arg, _) => {
                let mut result = format!(
                    "{}{}{}\n",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    func
                );
                let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
                result.push_str(&arg.to_string_tree(new_prefix, false));
                result
            }
        }
    }
}
