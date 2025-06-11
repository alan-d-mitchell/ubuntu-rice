use std::fmt;

// ===========================================================================
// TOKEN DEFINITIONS
// ===========================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),

    // Identifer
    Identifer(String),

    // Keywords
    If,
    ElseIf,                 
    Else,                  
    In,                    
    Is,                     
    Break,
    Return,
    Skip,
    Fn,
    While,
    For,
    ForEach,
    Const,
    Let,
    Null,

    // Types
    I32,        // 32-bit int -> default for int
    I64,        // 64-bit int
    F32,        // 32-bit float -> default for float
    F64,        // 64-bit float
    CharType,   
    StringType,     
    BooleanType,
    VoidType,

    // Operators
    Assign,           // =
    Equal,            // ==
    NotEqual,         // !=
    Less,             // <
    LessEqual,        // <=
    Greater,          // >
    GreaterEqual,     // >=
    Plus,             // +
    Minus,            // -
    Multiply,         // *
    Divide,           // /
    Modulo,           // %
    Power,            // **
    Increment,        // ++
    Decrement,        // --
    PlusAssign,       // +=
    MinusAssign,      // -=
    MultiplyAssign,   // *=
    DivideAssign,     // /=
    ModuloAssign,     // %=
    And,              // &&
    Or,               // ||
    Not,              // !
    Arrow,            // ->

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    SemiColon,
    Comma,
    Colon,              // :
    DoubleColon,        // ::

    // Special
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}