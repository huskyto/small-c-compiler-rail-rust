
pub mod token;
mod tokenizer;
mod token_factory;

use token::{Token, TokenType};
use token_factory::TokenFactory;
use tokenizer::Tokenizer;

pub struct Parser { }

impl Parser {
    pub fn parse(code: String) -> Result<Vec<Token>, Vec<Token>> {
        let mut res:Vec<Token> = vec![];
        let mut has_errors = false;
        let mut line_number = 1;
    
        let mut tokenizer = Tokenizer::new(&code);
        while let Some(lexeme) = tokenizer.next_token() {
            match TokenFactory::from_lexeme(&lexeme) {
                Ok(token) => {
                    if token.token_type == TokenType::Empty {
                        continue;
                    };
                    if token.token_type == TokenType::LineBreak {
                        line_number += 1;
                    };
                    res.push(token);
                },
                Err(error) => {
                    eprintln!("[Line {line_number}] {error}");
                    has_errors = true;
                }
            }
        }
    
        res.push(Token { token_type: TokenType::EoF, lexeme: "".to_string(), literal: None});
    
        if has_errors {
            Err(res)
        }
        else {
            Ok(res)
        }
    }
}