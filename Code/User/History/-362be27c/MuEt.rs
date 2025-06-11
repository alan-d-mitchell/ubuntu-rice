use crate::hydrac::parse::lexer::token::{Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    chars: Vec<char>,
    current: usize,
    start: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars().collect(),
            current: 0,
            start: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, String> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            self.start = self.current;
            let start_line = self.line;
            let start_column = self.column;

            if let Some(token_type) = self.scan_token()? {
                let lexeme = &self.input[self.start_offset()..self.current_offset()];

                tokens.push(Token {
                    token_type,
                    lexeme,
                    line: start_line,
                    column: start_column,
                });
            }
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "",
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<TokenType>, String> {
        let c = self.advance();

        let result = match c {
            '"' => return self.scan_string(),
            '\'' => return self.scan_char()
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ';' => Some(TokenType::Semicolon),
            ':' => if self.match_char(':') {
                Some(TokenType::DoubleColon)
            } else {
                Some(TokenType::Colon)
            },
            '&' => if self.match_char('&') {
                Some(TokenType::And)
            } else {
                Some(TokenType::Reference)
            },
            '|' => if self.match_char('|') {
                Some(TokenType::Or)
            } else {
                Some(TokenType::HeapPointerBar)
            },
            '?' => Some(TokenType::Optional),
            ',' => Some(TokenType::Comma),
            '+' => if self.match_char('=') {
                Some(TokenType::PlusAssign)
            } else if self.match_char('+') {
                Some(TokenType::Increment)
            } else {
                Some(TokenType::Plus)
            },
            '-' => if self.match_char('=') {
                Some(TokenType::MinusAssign)
            } else if self.match_char('-') {
                Some(TokenType::Decrement)
            } else {
                Some(TokenType::Minus)
            },
            '/' => if self.match_char('/') {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }

                None
            } else if self.match_char('*') {
                // multi line comment, consume until '*/' or EOF
                while !self.is_at_end() {
                    if self.peek() == '*' && self.peek_next() == '/' {
                        // consume '*' and '/'
                        self.advance();
                        self.advance();

                        break;
                    }
                    else {
                        if self.peek() == '\n' {
                            self.line += 1;
                            self.column = 0;
                        }

                        self.advance();
                    }
                }
                
                None
            } else {
                Some(TokenType::Divide)
            },
            '=' => if self.match_char('=') {
                Some(TokenType::Equal)
            } else {
                Some(TokenType::Assign)
            },
            '!' => if self.match_char('=') {
                Some(TokenType::NotEqual)
            } else {
                Some(TokenType::Not)
            },
            '<' => if self.match_char('=') {
                Some(TokenType::LessEqual)
            } else {
                Some(TokenType::LeftAngle)
            },
            '>' => if self.match_char('=') {
                Some(TokenType::GreaterEqual)
            } else {
                Some(TokenType::RightAngle)
            },
            '\n' => {
                self.line += 1;
                self.column = 0;

                Some(TokenType::Newline)
            }
            _ => {
                if c.is_ascii_digit() {
                    return self.scan_number(c);
                } 
                else if c.is_alphabetic() || c == '_' {
                    return self.scan_identifier(c);
                } 
                else {
                    return Err(format!("Unexpected character '{}' at line {}, column {}", c, self.line, self.column));
                }
            }
        };

        Ok(result)
    }

    fn scan_number(&mut self, first_digit: char) -> Result<Option<TokenType>, String> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume '.'

            while self.peek().is_ascii_digit() {
                self.advance();
            }

            let value: f64 = self.input[self.start_offset()..self.current_offset()].parse().map_err(|_| "Invalid float literal")?;

            Ok(Some(TokenType::FloatLiteral(value)))
        } 
        else {
            let value: i64 = self.input[self.start_offset()..self.current_offset()].parse().map_err(|_| "Invalid integer literal")?;

            Ok(Some(TokenType::IntLiteral(value)))
        }
    }

    fn scan_identifier(&mut self, _first: char) -> Result<Option<TokenType>, String> {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text = &self.input[self.start_offset()..self.current_offset()];

        Ok(Some(self.get_keyword_or_identifier(text)))
    }

    fn get_keyword_or_identifier(&self, text: &str) -> TokenType {
        match text {
            "if" => TokenType::If,
            "else if" => TokenType::ElseIf,
            "else" => TokenType::Else,
            "in" => TokenType::In,
            "is" => TokenType::Is,
            "break" => TokenType::Break,
            "return" => TokenType::Return,
            "skip" => TokenType::Skip,
            "fn" => TokenType::Function,
            "while" => TokenType::While,
            "for" => TokenType::For,
            "forEach" => TokenType::ForEach,
            "Const" => TokenType::Const,
            "let" => TokenType::Let,
            "true" => TokenType::BoolLiteral(true),
            "false" => TokenType::BoolLiteral(false),
            "null" => TokenType::Null,
            _ => TokenType::Identifier(text.to_string()),
        }
    }

    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1;
        self.column += 1;

        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.chars[self.current] != expected {
            false
        } 
        else {
            self.current += 1;
            self.column += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } 
        else {
            self.chars[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.current + 1]
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                _ => break,
            }
        }
    }

    fn scan_string(&mut self) -> Result<Option<TokenType>, String> {
        let mut value = String::new();

        while !self.is_at_end() {
            let c = self.advance();

            match c {
                '"' => {
                    return Ok(Some(TokenType::StringLiteral(value)));
                }
                '\\' => {
                    let escaped = match self.advance() {
                        'n' => '\n', // new line
                        'r' => '\r', // move cursor to beginnning of line
                        't' => '\t', // tab
                        '"' => '"',
                        '\\' => '\\',

                        other => return Err(format!("Invalid escape sequence: '\\{}'", other)),
                    };

                    value.push(escaped);
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;

                    value.push('\n');
                }
                _ => {
                    value.push(c);
                }
            }
        }

        Err(format!(
            "Unterminated string at line {}, column {}",
            self.line, self.column
        ))
    }

    fn scan_char(&mut self) -> Result<Option<TokenType>, String> {
        if self.is_at_end() {
            return Err(format!("Unterminated char at line {}, column {}", self.line, self.column));
        }

        let c = match self.advance() {
            '\\' => match self.advance() {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',

                other => return Err(format!("Invalid escape sequence: '\\{}", other))
            },

            other => other,
        };

        if self.peek() != '\'' {
            return Err(format!(
                "Expected closing single quote at line {}, column {}",
                self.line, self.column
            ));
        }

        self.advance();

        Ok(Some(TokenType::CharLiteral(c)))
    }

    fn start_offset(&self) -> usize {
        self.chars[..self.start].iter().map(|c| c.len_utf8()).sum()
    }

    fn current_offset(&self) -> usize {
        self.chars[..self.current].iter().map(|c| c.len_utf8()).sum()
    }
}
