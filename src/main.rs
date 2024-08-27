
mod token;
mod token_factory;

use std::fs;

fn main() {
    println!("Hello, world!");
    
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).unwrap_or(&"".to_string()) == "-P" {
        let filename = args.get(2).unwrap_or_else(| | std::process::exit(64));
        let contents = fs::read_to_string(filename).expect("IO error");

        println!("{contents}");
    }
}
