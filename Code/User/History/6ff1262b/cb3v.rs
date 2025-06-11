mod hydrac;

use hydrac::parse::lexer::{Lexer, Token, TokenType};

fn main() {
    // Test 1: Basic tokens
    test_basic_tokens();
    
    // Test 2: Operators and assignments
    test_operators();
    
    // Test 3: Keywords and identifiers
    test_keywords_and_identifiers();
    
    // Test 4: Literals
    test_literals();
    
    // Test 5: Comments
    test_comments();
    
    // Test 6: Complex expression
    test_complex_expression();
    
    // Test 7: Error cases
    test_error_cases();
}

fn test_basic_tokens() {
    println!("=== Testing Basic Tokens ===");
    let input = "( ) { } [ ] ; , : ::";
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

fn test_operators() {
    println!("=== Testing Operators ===");
    let input = "+ - * / % ** ++ -- += -= *= /= %= == != < <= > >= && || ! ->";
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

fn test_keywords_and_identifiers() {
    println!("=== Testing Keywords and Identifiers ===");
    let input = "let const fn if else while for forEach break return skip null true false my_var _private";
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

fn test_literals() {
    println!("=== Testing Literals ===");
    let input = r#"42 3.14 "hello world" 'a' true false "escaped\n\t\"string""#;
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

fn test_comments() {
    println!("=== Testing Comments ===");
    let input = r#"
    let x = 5; // single line comment
    /* multi-line
       comment */
    let y = 10;
    "#;
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

fn test_complex_expression() {
    println!("=== Testing Complex Expression ===");
    let input = r#"
    fn fibonacci(n: i32) -> i32 {
        if n <= 1 {
            return n;
        }
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
    "#;
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

fn test_error_cases() {
    println!("=== Testing Error Cases ===");
    
    // Test unterminated string
    println!("Testing unterminated string:");
    let input1 = r#"let msg = "unterminated"#;
    let mut lexer1 = Lexer::new(input1);
    match lexer1.tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Test unexpected character
    println!("\nTesting unexpected character:");
    let input2 = "let x = 5 @ 10;";
    let mut lexer2 = Lexer::new(input2);
    match lexer2.tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Test single & or |
    println!("\nTesting single & character:");
    let input3 = "let x = a & b;";
    let mut lexer3 = Lexer::new(input3);
    match lexer3.tokenize() {
        Ok(tokens) => {
            for token in &tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
    println!();
}

// Helper function to test specific token sequences
fn assert_tokens(input: &str, expected: Vec<TokenType>) {
    let mut lexer = Lexer::new(input);
    match lexer.tokenize() {
        Ok(tokens) => {
            let token_types: Vec<_> = tokens.iter()
                .filter(|t| !matches!(t.token_type, TokenType::Newline | TokenType::Eof))
                .map(|t| &t.token_type)
                .collect();
            
            if token_types.len() == expected.len() {
                let matches = token_types.iter().zip(expected.iter()).all(|(actual, expected)| {
                    std::mem::discriminant(*actual) == std::mem::discriminant(expected)
                });
                
                if matches {
                    println!("✓ Test passed for: {}", input);
                } else {
                    println!("✗ Test failed for: {}", input);
                    println!("  Expected: {:?}", expected);
                    println!("  Actual: {:?}", token_types);
                }
            } else {
                println!("✗ Test failed for: {} (length mismatch)", input);
                println!("  Expected {} tokens, got {}", expected.len(), token_types.len());
            }
        }
        Err(e) => {
            println!("✗ Test failed with error for: {}", input);
            println!("  Error: {}", e);
        }
    }
}