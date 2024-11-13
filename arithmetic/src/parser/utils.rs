use super::{postfix::ast_to_postfix, ASTNode, Lexer, Parser};

pub fn print_ast(input: &str) {
    let lexer = Lexer::new(input);

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

pub fn print_ast_with_values(input: &str) {
    let lexer = Lexer::with_set_values(input, true);

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

pub fn print_postfix(input: &str) {
    let lexer = Lexer::new(input);

    let mut parser = Parser::new(lexer).unwrap();
    let ast = match parser.parse_expression() {
        Ok(ast) => ast,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    println!("Postfix Notation:");
    let postfix = ast_to_postfix(&ast);
    println!("{}", postfix);
}

pub fn print_ast_tree(node: &ASTNode, prefix: String, is_left: bool) {
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
