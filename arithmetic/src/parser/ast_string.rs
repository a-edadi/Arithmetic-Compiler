use super::{ASTNode, Num};

impl ASTNode {
    pub fn stringify(&self, prefix: String, is_left: bool) -> String {
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
            ASTNode::Mantissa(mantiss_str, _) => {
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
                result.push_str(&left.stringify(new_prefix.clone(), true)); // Left operand
                result.push_str(&right.stringify(new_prefix, false)); // Right operand
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
                result.push_str(&expr.stringify(new_prefix, false));
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
                result.push_str(&arg.stringify(new_prefix, false));
                result
            }
        }
    }
}
