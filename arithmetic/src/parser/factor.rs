use super::{ASTNode, CompilerError, Parser, ParserError, TokenKind};

impl<'a> Parser<'a> {
    pub fn parse_factor(&mut self) -> Result<ASTNode, CompilerError> {
        let span = self.current_token.span.clone();

        match &self.current_token.kind {
            // Handle unary minus (negation)
            TokenKind::Minus => {
                self.advance()?;
                let operand = self.parse_factor()?;
                Ok(ASTNode::UnaryOp(TokenKind::Minus, Box::new(operand), span))
            }

            // Handle unary plus
            TokenKind::Plus => {
                self.advance()?;
                let operand = self.parse_factor()?;
                Ok(operand)
            }

            // Handle Number literal
            TokenKind::Number(n) => {
                let value = n.clone();
                self.advance()?;

                // Check if the next token is also a number without an operator between them
                if let TokenKind::Number(_) = self.current_token.kind {
                    return Err(CompilerError::Parse(ParserError::MissingOperator(
                        self.lexer.line,
                        self.lexer.pos,
                    )));
                }

                Ok(ASTNode::Number(value, span))
            }

            // Handle Mantiss
            TokenKind::Mantissa(mantiss_str) => {
                let mantiss_value = mantiss_str.clone();
                self.advance()?;
                Ok(ASTNode::Mantissa(mantiss_value, span))
            }

            // Handle Euler's number(e)
            TokenKind::Euler => {
                self.advance()?;
                Ok(ASTNode::Constant(TokenKind::Euler, span))
            }

            // Handle Pi
            TokenKind::Pi => {
                self.advance()?;
                Ok(ASTNode::Constant(TokenKind::Pi, span))
            }

            // Handle functions: Sin, Cos, Tan, ...
            TokenKind::Sin
            | TokenKind::Cos
            | TokenKind::Tan
            | TokenKind::Cotan
            | TokenKind::Arcsin
            | TokenKind::Arccos
            | TokenKind::Arctan
            | TokenKind::Arccotan
            | TokenKind::Ln
            | TokenKind::Log
            | TokenKind::Exp
            | TokenKind::Sqrt
            | TokenKind::Sqr => {
                let func_name = self.current_token.kind.clone();

                self.advance()?;

                // after function there should be parens like sin(
                // if there is no left paren Raise error, we are expecting LeftParen
                if self.current_token.kind != TokenKind::LeftParen {
                    return Err(CompilerError::Parse(ParserError::MissingLParen(
                        self.lexer.pos,
                        self.lexer.line,
                    )));
                }

                self.advance()?; // Skip '('

                // Parse Inner expression
                let argument = self.parse_expression()?;

                // after parsing the inner expression we are looking for ) RightParen
                if self.current_token.kind != TokenKind::RightParen {
                    return Err(CompilerError::Parse(ParserError::MissingRParen(
                        self.lexer.line,
                        self.lexer.pos,
                    )));
                }

                self.advance()?; // Skip ')'

                let func_name = match func_name {
                    TokenKind::Sin => "sin",
                    TokenKind::Cos => "cos",
                    TokenKind::Tan => "tan",
                    TokenKind::Cotan => "cotan",
                    TokenKind::Arcsin => "arcsin",
                    TokenKind::Arccos => "arccos",
                    TokenKind::Arctan => "arctan",
                    TokenKind::Arccotan => "arccotan",
                    TokenKind::Ln => "ln",
                    TokenKind::Log => "log",
                    TokenKind::Exp => "exp",
                    TokenKind::Sqrt => "sqrt",
                    TokenKind::Sqr => "sqr",
                    _ => {
                        return Err(CompilerError::Parse(ParserError::UnexpectedToken(
                            self.current_token.kind.clone(),
                            self.lexer.pos,
                            self.lexer.line,
                        )))
                    }
                };

                // Add the node to the Tree
                Ok(ASTNode::FunctionCall(
                    func_name.to_string(),
                    Box::new(argument),
                    span,
                ))
            }

            // Handle identifiers
            TokenKind::Identifier(id) => {
                let identifier = id.clone();
                self.advance()?;
                Ok(ASTNode::Identifier(identifier, span))
            }

            // Parentheses
            TokenKind::LeftParen => {
                self.advance()?;
                let node = self.parse_expression()?;
                if self.current_token.kind != TokenKind::RightParen {
                    return Err(CompilerError::Parse(ParserError::MissingRParen(
                        self.lexer.line,
                        self.lexer.pos,
                    )));
                }
                self.advance()?; // Skip ')'
                Ok(node)
            }

            // handle single right paren, Missing left paren
            TokenKind::RightParen => Err(CompilerError::Parse(ParserError::MissingLParen(
                self.lexer.line,
                self.lexer.pos,
            ))),

            // Unexpected token
            _ => Err(CompilerError::Parse(ParserError::UnexpectedToken(
                self.current_token.kind.clone(),
                self.lexer.pos,
                self.lexer.line,
            ))),
        }
    }
}
