use std::fmt;

// ===========================================================================
// TOKEN DEFINITIONS
// ===========================================================================

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

    // Types
    

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


}