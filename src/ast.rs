#![allow(dead_code)] // TODO: Should disable eventually
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    AssignVariable(String, Box<Node>, Vec<Node>),
    Binary(BinaryOperation, Box<Node>),
    Boolean(bool),
    CallFunction(String, Vec<Node>),
    DeclareBoolean(String, Box<Node>),
    DeclareFloat(String, Box<Node>),
    DeclareFunction(String, Vec<Node>, Vec<Node>),
    DeclareString(String, Box<Node>),
    Float(f32),
    For(Box<Node>, Box<Node>, Vec<Node>),
    Function(String, Vec<Node>, Vec<Node>),
    If(Box<Node>, Vec<Node>, Vec<Node>),
    Main(Vec<Node>),
    Print(Box<Node>),
    Return(Box<Node>),
    ReadBoolean(Box<Node>),
    ReadFloat(Box<Node>),
    ReadString(Box<Node>),
    String(String),
    Unary(UnaryOperation),
    Variable(String),
    While(Box<Node>, Vec<Node>),
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Float(n) => write!(f, "{}", n),
            Node::String(s) => write!(f, "{}", s),
            Node::Boolean(b) => write!(f, "{}", b),
            _ => unreachable!()
        }
    }
}
