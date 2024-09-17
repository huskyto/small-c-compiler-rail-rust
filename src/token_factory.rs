use crate::token::{Token, TokenType};


struct TokenFactory {

}

impl TokenFactory {
    
    fn from_lexeme(lexeme: &str) -> Result<Token, String> {
        let token_type = Self::get_token_type(lexeme);
        if token_type == TokenType::Undef {
            Err(format!("Error: Unexpected character: {lexeme}"))
        }
        else if token_type == TokenType::String {
            if lexeme.ends_with("\"") {
                Ok(Token::new_lit(token_type, lexeme.to_string(), lexeme[1..lexeme.len() - 1].to_string()))
            }
            else {
                Err("Error: Unterminated string.".to_string())
            }
        }
        else {
            Ok(Token::new(token_type, lexeme.to_string()))
        }
    }

    fn get_token_type(lexeme: &str) -> TokenType {
        if lexeme.starts_with("//") {
            TokenType::Empty
        }
        else if lexeme.starts_with("\"") {
            TokenType::String
        }
        // else if lexeme.chars().next().unwrap().is_ascii_digit() {
        //     TokenType::Number
        // }
        // else if lexeme.chars().next().unwrap().is_ascii_alphabetic() || lexeme.starts_with('_') {
        //     TokenType::Identifier
        // }
        else {
            match lexeme {
                "(" => TokenType::LeftParen,
                ")" => TokenType::RightParen,
                "[" => TokenType::LeftBracket,
                "]" => TokenType::RightBracket,
                "{" => TokenType::LeftBrace,
                "}" => TokenType::RightBrace,

                "+" => TokenType::Plus,
                "-" => TokenType::Minus,
                "*" => TokenType::Star,

                "=" => TokenType::Equals,
                "==" => TokenType::EqualsEquals,
                "!=" => TokenType::NotEquals,
                "!" => TokenType::Bang,
                "<" => TokenType::Less,
                ">" => TokenType::Greater,
                "<=" => TokenType::LessEqual,
                ">=" => TokenType::GreaterEqual,

                // "" => TokenType::String,
                // "" => TokenType::Integer,
                // "" => TokenType::Float,

                _ => TokenType::Undef
            }
        }
    }
}
