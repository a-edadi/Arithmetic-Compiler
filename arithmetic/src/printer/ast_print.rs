use super::{ASTNode, Lexer, Num, Parser};

// recursively Prints the AST tree
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
        // Printing a mantissa node
        ASTNode::Mantiss(mantiss_str) => {
            println!(
                "{}{}{}",
                prefix,
                if is_left { "├── " } else { "└── " },
                mantiss_str
            );
        }

        // Printing an identifier node
        ASTNode::Identifier(id) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, id);
        }

        // Printing a constant node: E or Pi
        ASTNode::Constant(c) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, c);
        }

        // Printing a binary operation node: +, -, *, /
        ASTNode::BinaryOp(left, op, right) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, op);

            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            print_ast_tree(left, new_prefix.clone(), true); // Print left operand
            print_ast_tree(right, new_prefix, false); // Print right operand
        }

        // Printing a unary operation node: -x
        ASTNode::UnaryOp(op, expr) => {
            println!("{}{}{}", prefix, if is_left { "├── " } else { "└── " }, op);
            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            print_ast_tree(expr, new_prefix, false);
        }

        // Printing a function call node: sin(x), sqrt(x)
        ASTNode::FunctionCall(func, arg) => {
            println!(
                "{}{}{}",
                prefix,
                if is_left { "├── " } else { "└── " },
                func
            );
            let new_prefix = format!("{}{}", prefix, if is_left { "│   " } else { "    " });
            print_ast_tree(arg, new_prefix, false);
        }
    }
}

/// Prints ast from the Input
pub fn print_ast(input: &str) {
    let lexer = Lexer::new(input, false);

    let mut parser = Parser::new(lexer).unwrap();
    let ast = match parser.parse_expression() {
        Ok(ast) => ast,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    println!("Ast Tree:");
    print_ast_tree(&ast, "".to_string(), false);
}

/// Prints the ast from the input and replaces variables with actual values
/// provided by the user
pub fn print_ast_with_values(input: &str) {
    let lexer = Lexer::new(input, true);

    let mut parser = Parser::new(lexer).unwrap();
    let ast = match parser.parse_expression() {
        Ok(ast) => ast,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };
    println!("Ast Tree:");
    print_ast_tree(&ast, "".to_string(), false);
}
