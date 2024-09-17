
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>
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
    Undef,

    LineBreak,
    CarriageReturn,
    EoF
}

impl Token {

    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Self { token_type, lexeme, literal: None }
    }

    pub fn new_lit(token_type: TokenType, lexeme: String, literal: String) -> Self {
        Self { token_type, lexeme, literal: Some(literal) }
    }

    pub fn literal_print(&self) -> String {
        match self.literal.as_ref() {
            Some(lit) => lit,
            None => "null"
        }.to_string()
    }
}