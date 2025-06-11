use hydrac::parse::{Lexer, Token, TokenType};

fn main() {
    test_basic();
}

fn test_basic() {
    println!("=== Test Basic Tokens ===");
    
    let input = "() { } [ ] ;  ,  : ::";
    let mut lexer = Lexer::new(input);

    match lexer.tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("{:?}", token);
            }
        }

        Err(e) => println!("Error: {}", e),
    }

    println!();
}