mod hydrac;

use hydrac::parse::lexer::{Lexer, Token, TokenType};
use std::env;
use std::fs;
use std::process;

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file.hydra>", args[0]);
        eprintln!("Example: {} tests/test.hydra", args[0]);

        process::exit(1);
    }

    let filename = &args[1];
    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    println!("=== HYDRA COMPILER ===");
    println!("Compiling file: {}", filename);
}