use crate::token::{Token, TokenType};


struct TokenFactory {

}

impl TokenFactory {
    
    fn from_lexeme(lexeme: &str) -> Token {
        let token_type = Self::get_token_type(lexeme);
        Token::new(token_type, lexeme.to_string())
    }

    fn get_token_type(lexeme: &str) -> TokenType {
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
