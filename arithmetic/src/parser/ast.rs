use crate::lexer::token::Num;

use super::TokenKind;

#[derive(Debug)]
pub enum ASTNode {
    Number(Num),                                     // A number node
    Mantiss(String),                                 // Mantissa representation as a string
    Identifier(String),                              // A variable or function name
    BinaryOp(Box<ASTNode>, TokenKind, Box<ASTNode>), // A binary operation node using TokenKind
    UnaryOp(TokenKind, Box<ASTNode>),                // A unary operation node
    FunctionCall(String, Box<ASTNode>),              // Function call node (name, argument)
    Constant(TokenKind),                             // Pi and Euler
}

impl ASTNode {
    // Recursive function that returns the tree structure as a string
    pub fn to_string_tree(&self, prefix: String, is_left: bool) -> String {
        match self {
            // Formatting a number node
            ASTNode::Number(n) => {
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
            ASTNode::Mantiss(mantiss_str) => {
                format!(
                    "{}{}{}\n",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    mantiss_str
                )
            }
            // Formatting an identifier node
            ASTNode::Identifier(id) => {
                format!(
                    "{}{}{}\n",
                    prefix,
                    if is_left { "├── " } else { "└── " },
                    id
                )
            }
            // Formatting a constant node: E or Pi
            ASTNode::Constant(c) => {
                format!("{}{}{}\n", prefix, if is_left { "├── " } else { "└── " }, c)
            }
            // Formatting a binary operation node: +, -, *, /
            ASTNode::BinaryOp(left, op, right) => {
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
            ASTNode::UnaryOp(op, expr) => {
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
            ASTNode::FunctionCall(func, arg) => {
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
