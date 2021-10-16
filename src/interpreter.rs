use std::collections::HashMap;

use crate::ast::Node;

struct Frame {
    variables: HashMap<String, Node>,
    current: Node,
}

impl Frame {
    fn new() -> Frame {
        Frame {
            variables: HashMap::new(),
            current: Node::Noop,
        }
    }
}

struct State {
    functions: HashMap<String, Node>,
    stack: Vec<Frame>,
}

impl State {
    fn new() -> State {
        State {
            functions: HashMap::new(),
            stack: vec![Frame::new()],
        }
    }
}

pub fn evaluate(ast: Vec<Node>) -> Result<(), String> {
    let mut main = Node::Noop;
    let state = &mut State::new();

    for node in ast {
        match &node {
            Node::Main(_) => {
                main = node;
            }
            Node::DeclareFunction(name, _, _) => {
                state.functions.insert(name.to_string(), node);
            }
            _ => unreachable!(), // TODO: Get actual error message
        }
    }

    evaluate_node(main, state)
}

fn evaluate_node(ast: Node, state: &mut State) -> Result<(), String> {
    match ast {
        Node::AssignVariable(_, _, _) => todo!(),
        Node::Binary(_, _) => todo!(),
        Node::Boolean(_) => {
            state.stack.last_mut().unwrap().current = ast;
            Ok(())
        }
        Node::CallFunction(_, _) => todo!(),
        Node::DeclareBoolean(name, boolean) => {
            if let Node::Boolean(value) = *boolean {
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name, Node::Boolean(value));
                // TODO: Decide if replacing an existing var is an error
                Ok(())
            } else {
                Err("Not boolean".to_string())
            }
        }
        Node::DeclareFloat(name, float) => {
            if let Node::Float(value) = *float {
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name, Node::Float(value));
                Ok(())
            } else {
                Err("Not float".to_string())
            }
        }
        Node::DeclareFunction(_, _, _) => todo!(),
        Node::DeclareString(name, string) => {
            if let Node::String(value) = *string {
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name, Node::String(value));
                Ok(())
            } else {
                Err("Not string".to_string())
            }
        }
        Node::Float(_) => {
            state.stack.last_mut().unwrap().current = ast;
            Ok(())
        }
        Node::For(_, _, _) => todo!(),
        Node::If(_, _, _) => todo!(),
        Node::Main(statments) => {
            for statment in statments {
                evaluate_node(statment, state)?;
            }
            Ok(())
        }
        Node::Print(node) => {
            evaluate_node(*node, state)?;
            let value = state.stack.last().unwrap().current.clone();
            println!("{}", value);
            Ok(())
        }
        Node::Return(_) => todo!(),
        Node::ReadBoolean(_) => todo!(),
        Node::ReadFloat(_) => todo!(),
        Node::ReadString(_) => todo!(),
        Node::String(_) => {
            state.stack.last_mut().unwrap().current = ast;
            Ok(())
        }
        Node::Unary(_) => todo!(),
        Node::Variable(name) => {
            // Error if not found
            let value = state
                .stack
                .last_mut()
                .unwrap()
                .variables
                .get(&name)
                .unwrap()
                .clone();
            state.stack.last_mut().unwrap().current = value;
            Ok(())
        }
        Node::While(_, _) => todo!(),
        Node::Noop => Ok(()),
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn hello_there() {
//         let ast = vec![Node::Main(vec!(Node::Print(Box::new(Node::String(
//             "Hello there".to_string())))))];

//         assert_eq!(
//             evaluate(ast)

//         );
//     }

//     #[test]
//     fn hello_test() {
//         let mut stdout = Vec::new();

//         // pass fake stdout when calling when testing
//         hello(&mut stdout);

//         assert_eq!(stdout, b"Hello world\n");
//     }
// }
