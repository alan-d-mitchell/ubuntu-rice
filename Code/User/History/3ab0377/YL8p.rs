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
    I32,        // 32-bit int -> default for int
    I64,        // 64-bit int
    F32,        // 32-bit float -> default for float
    F64,        // 64-bit float
    Char,       // Character
    


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