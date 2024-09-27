
pub mod types;

use types::Environment;

use crate::{keywords::Keywords, parser::token::{Token, TokenType}};

pub struct Model { }

impl Model {
    pub fn to_model(tokens: Vec<Token>) -> Result<Environment, String> {
        // Err("Unimplemented".to_string())
        let mut head = 0;
        let mut state = State::Outside;
        let mut environment = Environment::new();
        let dummy_token = Token::new(TokenType::Undef, "".to_string());

        while head < tokens.len() {
            let current_token = tokens.get(head).unwrap();
            let next_token = tokens.get(head + 1).unwrap_or(&dummy_token);
                println!("Tokens.\n  Curr:\n{:?}.\n  Next:\n{:?}", current_token, next_token);
                // INCLUDES //
            if current_token.token_type == TokenType::Sharp {
                if Self::compare_identifier(next_token, Keywords::INCLUDE) {
                    return Err("Include format error".to_string());
                }
                head += 2;
                let mut following_token = tokens.get(head).unwrap_or(&dummy_token);
                    
                if following_token.token_type == TokenType::String {
                    environment.imports.push(following_token.literal.clone().unwrap());
                }
                else if following_token.token_type == TokenType::Less {
                    let mut include_path = String::new();
                    head += 1;
                    following_token = tokens.get(head).unwrap_or(&dummy_token);
                    while matches!(following_token.token_type, TokenType::Identifier | TokenType::Dot) {
                        include_path.push_str(&following_token.lexeme);
                        head += 1;
                        following_token = tokens.get(head).unwrap_or(&dummy_token);
                    }
                    if following_token.token_type != TokenType::Greater {
                        return Err("Include format error. Closing >".to_string());
                    }
                    environment.imports.push(include_path);
                }
                else {
                    return Err("Include format error".to_string());
                }
            }

            head += 1;
        }

        Ok(environment)
        // Err("".to_string())
    }

    fn compare_identifier(token: &Token, lexeme: &str) -> bool {
        token.token_type == TokenType::Identifier
                && token.lexeme == lexeme
    }
}

enum State {
    Outside,
    InClass,
    InFunction
}