
pub mod types;

use types::Environment;

use crate::parser::token::Token;

pub struct Model { }

impl Model {
    pub fn to_model(tokens: Vec<Token>) -> Result<Environment, String> {
        Err("Unimplemented".to_string())
    }
}