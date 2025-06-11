mod hydrac;

use hydrac::parse::lexer::{Lexer, Token, TokenType};
use std::env;
use std::fs;
use std::process;


fn print_help(program_name: &str) {
    println!(
        "\
Usage: {} <input.hydra> [-o output.hydra]

Flags:
  -o <file>       Specify output filename (default: <input>_output.hydra)
  --help          Show this help message

Environment variables:
  token_verbose   If set, print all tokens during tokenization

Examples:
  {} source.hydra
  {} source.hydra -o result.hydra
  token_verbose=1 {} source.hydra
",
        program_name, program_name, program_name, program_name
    );
}

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() == 1 || args.iter().any(|a| a == "--help") {
        print_help(&args[0]);

        return;
    }

    let filename = &args[1];
    let mut output_file = None;

    let mut i = 2;
    while i < args.len() {
        if args[i] == "-o" {
            if i + 1 >= args.len() {
                eprintln!("Error: -o flag requires an output filename");
                
                process::exit(1);
            }

            output_file = Some(args[i + 1].clone());
            i += 2;
        }
        else {
            eprintln!("Unknown argument: {}", args[i]);
            eprintln!("Usage: {} <input.hydra> [-o output(.hydra)]", args[0]);

            process::exit(1);
        }
    }

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    println!("=== HYDRA COMPILER ===");
    println!("Compiling file: {}", filename);

    println!("=== TOKENIZING ===");

    let mut lexer = Lexer::new(&contents);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            
            process::exit(1);
        }
    };

    println!("Tokenization successful");
    println!("Tokens found: {}", tokens.len());

    if env::var("token_verbose").is_ok() {
        println!("\n All tokens:");

        for (i, token) in tokens.iter().enumerate() {
            println!("  {:3}: {:?}", i, token);
        }
    }

    let output_file = output_file.unwrap_or_else(|| {
        let input_stem = Path::new(filename)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();

        format!("{}_output", input_stem)
    });

    let output_contents = tokens
        .iter()
        .map(|t| format!("{:?}", t))
        .collect::<Vec<_>>()
        .join("\n");
    
    match fs::write(&output_file, output_contents) {
        Ok(_) => println!("Output written to: {}", output_file),
        Err(err) => {
            eprintln!("Error writing output file '{}': {}", output_file, err);

            process::exit(1);
        }
    }
}