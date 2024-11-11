/// A few simple tests to test the overall lexer logic
#[cfg(test)]
mod tests {
    use crate::lexer::{Lexer, LexerError, TokenKind};

    #[test]
    fn test_empty_input() {
        let input = "";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex_all_tokens().unwrap();

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_single_number() {
        let input = "123";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex_all_tokens().unwrap();

        // Check that we have one number token and an Eof token
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Number(123));
    }

    #[test]
    fn test_basic_expression() {
        let input = "3 + 4";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex_all_tokens().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Number(3));
        assert_eq!(tokens[1].kind, TokenKind::Plus);
        assert_eq!(tokens[2].kind, TokenKind::Number(4));
    }

    #[test]
    fn test_punctuation() {
        let input = "+ - * % / ( ) ^ , { }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex_all_tokens().unwrap();

        let expected_kinds = vec![
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Multiply,
            TokenKind::Remainder,
            TokenKind::Divide,
            TokenKind::LeftParen,
            TokenKind::RightParen,
            TokenKind::Power,
            TokenKind::Comma,
            TokenKind::OpenBrace,
            TokenKind::CloseBrace,
        ];

        assert_eq!(tokens.len(), expected_kinds.len());

        for (expected, token) in expected_kinds.iter().zip(tokens.iter()) {
            assert_eq!(token.kind, *expected);
        }
    }

    #[test]
    fn test_function_identifier() {
        let input = "sinh(45) sin tan tanh cot coth";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex_all_tokens().unwrap();

        assert_eq!(tokens.len(), 9);
        assert_eq!(tokens[0].kind, TokenKind::Identifier); // "sinh"
        assert_eq!(tokens[1].kind, TokenKind::LeftParen); // "("
        assert_eq!(tokens[2].kind, TokenKind::Number(45)); // "45"
        assert_eq!(tokens[3].kind, TokenKind::RightParen); // ")"
        assert_eq!(tokens[4].kind, TokenKind::Identifier); // "sin"
        assert_eq!(tokens[5].kind, TokenKind::Identifier); // "tan"
        assert_eq!(tokens[6].kind, TokenKind::Identifier); // "tanh"
        assert_eq!(tokens[7].kind, TokenKind::Identifier); // "cot"
        assert_eq!(tokens[8].kind, TokenKind::Identifier); // "coth"
    }

    #[test]
    fn test_unexpected_character() {
        let input = "3 + @";
        let mut lexer = Lexer::new(input);
        let result = lexer.lex_all_tokens();

        // We expect an error due to the '@' character
        assert!(result.is_err());
        match result {
            Err(LexerError::InvalidCharacter(c, pos)) => {
                assert_eq!(c, '@');
                assert_eq!(pos, 4);
            }
            _ => panic!("Expected InvalidCharacter error"),
        }
    }
}
