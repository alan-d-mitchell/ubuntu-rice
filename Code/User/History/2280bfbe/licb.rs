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