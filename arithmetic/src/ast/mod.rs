pub mod eval;
pub mod plot;
pub mod postfix;
pub mod root;
pub mod var;
pub mod wrapper;

use crate::errors::{
    eval::EvaluationError, plot::PlottingError, root::RootFinderError, CompilerError,
};
use crate::lexer::{
    span::TextSpan,
    token::{Num, TokenKind},
};
use crate::utils::{
    prompt_input::get_and_parse_user_input, random_number::generate_random_4_digits,
};

use eval::Evaluator;
use plot::FunctionPlotter;
use root::RootFinder;
use var::VariableManager;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ASTNode {
    Number(Num, TextSpan),
    BinaryOp(Box<ASTNode>, TokenKind, Box<ASTNode>, TextSpan),
    UnaryOp(TokenKind, Box<ASTNode>, TextSpan),
    Mantissa(String, TextSpan),
    Constant(TokenKind, TextSpan),
    Identifier(String, TextSpan),
    FunctionCall(String, Box<ASTNode>, TextSpan),
}

/// Method that returns string representation of the AST in tree format
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

            // Formatting a unary operation node: -x,-1 ,+2
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

            // Formatting a function call node: sin(x), sqrt(x), ...
            ASTNode::FunctionCall(func, arg, _) => {
                let mut result = format!(
                    "{}{}{}\n",
                    prefix,
                    if is_left { "├──" } else { "└── " },
                    func
                );
                let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
                result.push_str(&arg.stringify(new_prefix, false));
                result
            }
        }
    }
}
