
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    Plus,
    Minus,
    Star,

    Equals,
    EqualsEquals,
    NotEquals,
    Bang,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    String,
    Integer,
    Float,

    Empty,
    Undef
}

impl Token {

    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Self { token_type, lexeme, literal: None }
    }

    pub fn new_lit(token_type: TokenType, lexeme: String, literal: String) -> Self {
        Self { token_type, lexeme, literal: Some(literal) }
    }
}