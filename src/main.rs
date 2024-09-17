
mod token;
mod token_factory;
mod tokenizer;

use std::fs;

use token::Token;
use token::TokenType;
use token_factory::TokenFactory;
use tokenizer::Tokenizer;

fn parse(code: String) -> Result<Vec<Token>, Vec<Token>> {
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

fn main() {
    println!("Hello, world!");
    
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).unwrap_or(&"".to_string()) == "-P" {
        let filename = args.get(2).unwrap_or_else(| | std::process::exit(64));
        let file_contents = fs::read_to_string(filename).expect("IO error");

        // println!("{contents}");
        if !file_contents.is_empty() {
            let tokens_res = parse(file_contents.to_string());
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
