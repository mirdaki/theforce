#![allow(dead_code)] // Should disable eventually

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    // For,
    AssignVariable(String, Vec<Node>),
    Binary(BinaryOperation, Box<Node>, Box<Node>),
    Boolean(bool),
    CallFunction(String, Vec<Node>),
    DeclareBoolean(String, Box<Node>),
    DeclareFloat(String, Box<Node>),
    DeclareFunction(String, Vec<Node>, Vec<Node>),
    DeclareString(String, Box<Node>),
    Float(f32),
    If(Box<Node>, Vec<Node>, Vec<Node>),
    Print(String),
    Return(Box<Node>),
    String(String),
    Unary(UnaryOperation, Box<Node>),
    Variable(String),
    While(Box<Node>, Vec<Node>),
    Noop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperation {
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Exponent,
    Modulus,
    Equal,
    GreaterThan,
    LessThan,
    Or,
    And,
}
