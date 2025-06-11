use std::fmt;

// ============================================
// AST DEFINITIONS
// ============================================

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

pub enum Item {
    Function(function),
}

pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: Type,
    pub body: Block,
}

pub struct Parameter {
    pub name: String,
    pub param_type: Type,
}

pub struct Block {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    VarDecl(VarDecl),
    If(IfStmt)
}