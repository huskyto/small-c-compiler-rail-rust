
mod model;
mod parser;

use std::fs;

use parser::Parser;
// use parser::token::Token;
use parser::token::TokenType;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).unwrap_or(&"".to_string()) == "-P" {
        let filename = args.get(2).unwrap_or_else(| | std::process::exit(64));
        let file_contents = fs::read_to_string(filename).expect("IO error");

        // println!("{contents}");
        if !file_contents.is_empty() {
            let tokens_res = Parser::parse(file_contents.to_string());
            let mut has_errors = false;
            let tokens = match tokens_res {
                Ok(ts) => ts,
                Err(ts) => {
                    has_errors = true;
                    ts
                }
            };
            for token in tokens {
                if matches!(token.token_type, TokenType::CarriageReturn | TokenType::LineBreak) {
                    continue;
                }

                let lexeme = {
                    if token.lexeme == "\n" { "\\n" }
                    else if token.lexeme == "\r" { "\\r" }
                    else { &token.lexeme }
                };
                println!("{:#?} {} {}", token.token_type, lexeme, token.literal_print())
            }

            if has_errors {
                std::process::exit(65);
            }
        }
        else {
            println!("EOF null")
        }
    }
}
