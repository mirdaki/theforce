use std::{collections::HashMap, fmt::Result};

use crate::ast::Node;

struct State {
    functions: HashMap<String, Node>,
}

pub fn evaluate(ast: Vec<Node>) -> Result {
    for ast in ast {
        evaluate_node(ast)?
    }
    Ok(())
}

fn evaluate_node(ast: Node) -> Result {
    match ast {
        Node::Main(statments) => {
            for statment in statments {
                evaluate_node(statment);
            }
        }
        Node::Print(x) => {
            println!("{}", x);
        }
        _ => todo!(),
    }
    Ok(())
}
