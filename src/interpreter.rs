use std::collections::HashMap;

use crate::ast::Node;

struct State {
    functions: HashMap<String, Node>,
    variables: Vec<HashMap<String, Node>>,
    current: Node,
}

pub fn evaluate(ast: Vec<Node>) -> Result<(), String> {
    let mut main = Node::Noop;
    let state = &mut State {
        functions: HashMap::new(),
        variables: vec![HashMap::new()],
        current: Node::Noop,
    };

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
            state.current = ast;
            Ok(())
        }
        Node::CallFunction(_, _) => todo!(),
        Node::DeclareBoolean(name, boolean) => {
            if let Node::Boolean(value) = *boolean {
                state
                    .variables
                    .last_mut()
                    .unwrap()
                    .insert(name, Node::Boolean(value));
                Ok(())
            } else {
                Err("Not boolean".to_string())
            }
        }
        Node::DeclareFloat(name, float) => {
            if let Node::Float(value) = *float {
                state
                    .variables
                    .last_mut()
                    .unwrap()
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
                    .variables
                    .last_mut()
                    .unwrap()
                    .insert(name, Node::String(value));
                Ok(())
            } else {
                Err("Not string".to_string())
            }
        }
        Node::Float(_) => {
            state.current = ast;
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
            let value = state.current.clone();
            println!("{}", value);
            Ok(())
        }
        Node::Return(_) => todo!(),
        Node::ReadBoolean(_) => todo!(),
        Node::ReadFloat(_) => todo!(),
        Node::ReadString(_) => todo!(),
        Node::String(_) => {
            state.current = ast;
            Ok(())
        }
        Node::Unary(_) => todo!(),
        Node::Variable(name) => {
            // Error if not found
            let value = state.variables.last().unwrap().get(&name).unwrap().clone();
            state.current = value;
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
