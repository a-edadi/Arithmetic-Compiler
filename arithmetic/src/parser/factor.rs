use super::{ASTNode, CompilerError, Parser, ParserError, TextSpan, TokenKind};

impl<'a> Parser<'a> {
    pub fn parse_factor(&mut self) -> Result<ASTNode, CompilerError> {
        let span = self.current_token.span.clone();

        match &self.current_token.kind {
            TokenKind::Minus | TokenKind::Plus => self.parse_unary_operator(span),
            TokenKind::Number(_) => self.parse_number(span),
            TokenKind::Mantissa(_) => self.parse_mantissa(span),
            TokenKind::Euler | TokenKind::Pi => self.parse_constant(span),
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
            | TokenKind::Sqr => self.parse_function(span),
            TokenKind::Identifier(_) => self.parse_identifier(span),
            TokenKind::LeftParen => self.parse_parentheses(),
            TokenKind::RightParen => Err(CompilerError::Parse(ParserError::MissingLParen(
                self.lexer.line,
                self.lexer.pos,
            ))),
            _ => Err(CompilerError::Parse(ParserError::UnexpectedToken(
                self.current_token.kind.clone(),
                self.lexer.pos,
                self.lexer.line,
            ))),
        }
    }

    fn parse_unary_operator(&mut self, span: TextSpan) -> Result<ASTNode, CompilerError> {
        let operator = self.current_token.kind.clone();
        self.advance()?;
        let operand = self.parse_factor()?;
        if operator == TokenKind::Minus {
            Ok(ASTNode::UnaryOp(TokenKind::Minus, Box::new(operand), span))
        } else {
            // Unary plus just returns the operand
            Ok(operand)
        }
    }

    fn parse_number(&mut self, span: TextSpan) -> Result<ASTNode, CompilerError> {
        if let TokenKind::Number(value) = &self.current_token.kind {
            let value = value.clone();
            self.advance()?;

            // Ensure no consecutive numbers without an operator
            if let TokenKind::Number(_) = self.current_token.kind {
                return Err(CompilerError::Parse(ParserError::MissingOperator(
                    self.lexer.line,
                    self.lexer.pos,
                )));
            }

            Ok(ASTNode::Number(value, span))
        } else {
            unreachable!()
        }
    }

    fn parse_mantissa(&mut self, span: TextSpan) -> Result<ASTNode, CompilerError> {
        if let TokenKind::Mantissa(value) = &self.current_token.kind {
            let value = value.clone();
            self.advance()?;
            Ok(ASTNode::Mantissa(value, span))
        } else {
            unreachable!()
        }
    }

    fn parse_constant(&mut self, span: TextSpan) -> Result<ASTNode, CompilerError> {
        let constant = self.current_token.kind.clone();
        self.advance()?;
        Ok(ASTNode::Constant(constant, span))
    }

    fn parse_function(&mut self, span: TextSpan) -> Result<ASTNode, CompilerError> {
        let func_name = self.current_token.kind.clone();
        self.advance()?;

        if self.current_token.kind != TokenKind::LeftParen {
            return Err(CompilerError::Parse(ParserError::MissingLParen(
                self.lexer.pos,
                self.lexer.line,
            )));
        }

        self.advance()?; // Skip '('
        let argument = self.parse_expression()?; // Parse function argument

        if self.current_token.kind != TokenKind::RightParen {
            return Err(CompilerError::Parse(ParserError::MissingRParen(
                self.lexer.line,
                self.lexer.pos,
            )));
        }

        self.advance()?; // Skip ')'

        let func_name_str = match func_name {
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
            _ => unreachable!(),
        };

        Ok(ASTNode::FunctionCall(
            func_name_str.to_string(),
            Box::new(argument),
            span,
        ))
    }

    fn parse_identifier(&mut self, span: TextSpan) -> Result<ASTNode, CompilerError> {
        if let TokenKind::Identifier(name) = &self.current_token.kind {
            let identifier = name.clone();
            self.advance()?;
            Ok(ASTNode::Identifier(identifier, span))
        } else {
            unreachable!()
        }
    }

    fn parse_parentheses(&mut self) -> Result<ASTNode, CompilerError> {
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
}
