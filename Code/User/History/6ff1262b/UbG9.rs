mod hydrac;

use hydrac::parse::lexer::{Lexer, Token, TokenType};

use std::env;
use std::fs;
use std::process;
use std::path::Path;


fn print_help(program_name: &str) {
    println!(
        "\
Usage: {} <input.hydra> [-o output]

Flags:
  -o <file>       Specify output filename (default: <input>_output)
  --tokens        Show tokens of <input.hydra> in a file
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

    if args.len() < 2 {
        eprintln!("Error: Missing input file");
        eprintln!("Run '{} --help' for help.", args[0]);

        process::exit(1);
    }

    let filename = &args[1];

    let filename_path = Path::new(filename);
    match filename_path.extension().and_then(|extension| extension.to_str()) {
        Some("hydra") => {}
        _ => {
             eprintln!("Error: '{}' is not a hydra file", filename);
             eprintln!("Run '{} --help' for help.", args[0]);

             process::exit(1);
        }
    }

    let mut output_file = None;

    let mut i = 2;
    while i < args.len() {
        if args[i] == "-o" {
            if i + 1 >= args.len() {
                eprintln!("Error: -o flag requires an output filename");
                eprintln!("Run '{} --help' for help.", args[0]);
                
                process::exit(1);
            }

            output_file = Some(args[i + 1].clone());
            i += 2;
        }
        else {
            eprintln!("Unknown argument: {}", args[i]);
            eprintln!("Run '{} --help' for help.", args[0]);

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

    let mut output = String::new();

    output.push_str("=== HYDRA COMPILER ===\n");
    output.push_str(&format!("Compiling file: {}\n\n", filename));

    output.push_str("=== TOKENIZING ===\n");

    let mut lexer = Lexer::new(&contents);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Lexer error: {}", err);

            process::exit(1);
        }
    };

    output.push_str("Tokenization successful\n");
    output.push_str(&format!("Tokens found: {}\n", tokens.len()));

    if env::var("token_verbose").is_ok() {
        output.push_str("\nAll tokens:\n");

        for (i, token) in tokens.iter().enumerate() {
            output.push_str(&format!("  {:3}: {:?}\n", i, token));
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

    let token_lines = tokens.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>().join("\n");
    output.push_str("\nTokens (raw):\n");
    output.push_str(&token_lines);
    
    match fs::write(&output_file, &output) {
        Ok(_) => println!("Output written to: {}", output_file),
        Err(err) => {
            eprintln!("Error writing output file '{}': {}", output_file, err);

            process::exit(1);
        }
    }
}