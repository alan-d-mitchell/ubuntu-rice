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
}