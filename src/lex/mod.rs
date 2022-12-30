mod lexer;
mod token;

pub use lexer::Lexer;
pub use token::TokenType;

#[cfg(test)]
mod tests {
    use super::token::{Token, TokenType};
    use super::Lexer;

    #[test]
    fn basic_lex() {
        let src = r#"3.14 / 5
            == = <= "string" - 6"#;
        let mut lexer = Lexer::new(src.to_string());

        let expected = vec![
            Token::new(TokenType::Number(3.14), 1),
            Token::new(TokenType::Slash, 1),
            Token::new(TokenType::Number(5.0), 1),
            Token::new(TokenType::EqualEqual, 2),
            Token::new(TokenType::Equal, 2),
            Token::new(TokenType::Leq, 2),
            Token::new(TokenType::String("string".to_string()), 2),
            Token::new(TokenType::Minus, 2),
            Token::new(TokenType::Number(6.0), 2),
        ];

        let mut tokens = Vec::new();
        loop {
            let tok = lexer.scan_token().unwrap();
            if tok.ty() == &TokenType::EOF {
                break;
            }
            tokens.push(tok);
        }
        assert_eq!(tokens, expected);
    }

    // TODO: how to test errors
}
