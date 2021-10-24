use std::collections::HashMap;

use crate::ast::{BinaryOperation, Node, UnaryOperation};

struct Frame {
    variables: HashMap<String, Node>,
    current: Node,
    loop_flag: Vec<Node>,
}

impl Frame {
    fn new() -> Frame {
        Frame {
            variables: HashMap::new(),
            current: Node::Noop,
            loop_flag: vec![Node::Noop],
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
        Node::AssignVariable(variable_name, first_value, operations) => {
            // TODO: Check if value is actually a value?
            // Place value at top of stack
            let _ = evaluate_node(*first_value, state);
            for operation in operations {
                // TOOD: Actual error message
                let _ = match operation {
                    Node::Binary(operation, value) => evaluate_binary(operation, *value, state),
                    Node::Unary(operation) => evaluate_unary(operation, state),
                    _ => Err("Invalid operation".to_string()),
                };
            }
            let new_current = state.stack.last().unwrap().current.clone();
            state
                .stack
                .last_mut()
                .unwrap()
                .variables
                .insert(variable_name, new_current);
            Ok(())
        }
        // Taken care of by the assign variable
        Node::Binary(_, _) => unreachable!(),
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
        Node::For(max, flag, statments)
            if matches!(&*max, Node::Float(_)) && matches!(&*flag, Node::Variable(_)) =>
        {
            // Will be initiliaed, because of the match guard
            let mut max_value = 0.0;
            if let Node::Float(max) = *max {
                max_value = max;
            }

            let mut flag_var_name = "".to_string();
            if let Node::Variable(ref var_name) = *flag {
                flag_var_name = var_name.clone();
            }

            // Get the variable value
            let _ = evaluate_node(
                state
                    .stack
                    .last()
                    .unwrap()
                    .variables
                    .get(&flag_var_name)
                    .unwrap()
                    .clone(),
                state,
            );

            let mut flag_value =
                if let Node::Float(value) = state.stack.last().unwrap().current.clone() {
                    value
                } else {
                    return Err("Flag not a float".to_string());
                };

            // Check if should loop
            let mut continue_loop = !flag_value.eq(&max_value);

            // Loop
            while continue_loop {
                for statment in &statments {
                    evaluate_node(statment.clone(), state)?;
                }

                // Get the variable value
                let _ = evaluate_node(
                    state
                        .stack
                        .last()
                        .unwrap()
                        .variables
                        .get(&flag_var_name)
                        .unwrap()
                        .clone(),
                    state,
                );

                flag_value = if let Node::Float(value) = state.stack.last().unwrap().current.clone()
                {
                    value
                } else {
                    return Err("Flag not a float".to_string());
                };

                // Increment variable value
                flag_value += 1.0;

                // Set the variable value
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(flag_var_name.clone(), Node::Float(flag_value))
                    .unwrap();

                // Check if should loop
                continue_loop = !flag_value.eq(&max_value);
            }
            Ok(())
        }
        Node::If(flag, true_statments, false_statments) => {
            // Processes flag
            evaluate_node(*flag, state)?;

            // Only accept boolean results
            let if_flag = if let Node::Boolean(bool) = state.stack.last().unwrap().current {
                bool
            } else {
                return Err("Not boolean.".to_string());
            };

            // Choose a branch. False branch may not exist, but should be empty from parser
            let statments = if if_flag {
                true_statments
            } else {
                false_statments
            };

            for statment in statments {
                evaluate_node(statment, state)?;
            }

            Ok(())
        }
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
        // Taken care of by the assign variable
        Node::Unary(_) => unreachable!(),
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
        Node::While(flag, statments) => {
            state
                .stack
                .last_mut()
                .unwrap()
                .loop_flag
                .push(*flag.clone());
            // TODO: Propagate error
            let _ = evaluate_node(*flag, state);
            let mut continue_loop = match state.stack.last().unwrap().current {
                Node::Boolean(bool) => bool,
                Node::Float(float) => float != 0.0,
                _ => unreachable!(),
            };

            while continue_loop {
                for statment in &statments {
                    evaluate_node(statment.clone(), state)?;
                }
                let flag = state.stack.last_mut().unwrap().loop_flag.last().unwrap();
                let _ = evaluate_node(flag.clone(), state);
                continue_loop = match state.stack.last().unwrap().current {
                    Node::Boolean(bool) => bool,
                    Node::Float(float) => float != 0.0,
                    _ => unreachable!(),
                };
            }
            state.stack.last_mut().unwrap().loop_flag.pop();
            Ok(())
        }
        Node::Noop => Ok(()),
        _ => Err("Statment node valid".to_string()),
    }
}

fn evaluate_binary(op: BinaryOperation, value: Node, state: &mut State) -> Result<(), String> {
    match op {
        BinaryOperation::Add => math_operations(|x, y| x + y, value, state),
        BinaryOperation::Subtract => math_operations(|x, y| x - y, value, state),
        BinaryOperation::Multiply => math_operations(|x, y| x * y, value, state),
        BinaryOperation::Divide => math_operations(|x, y| x / y, value, state),
        BinaryOperation::Exponent => math_operations(|x, y| x.powf(y), value, state),
        BinaryOperation::Modulus => math_operations(|x, y| x % y, value, state),
        BinaryOperation::Equal => {
            let mut equal_value = value.clone();
            if let Node::Variable(var_name) = &value {
                equal_value = state
                    .stack
                    .last()
                    .unwrap()
                    .variables
                    .get(var_name)
                    .unwrap()
                    .clone();
            };
            match equal_value {
                Node::Boolean(_) => equality_bool_operations(|x, y| x == y, value, state),
                Node::Float(_) => equality_float_operations(|x, y| x.eq(&y), value, state),
                Node::String(_) => todo!(),
                _ => unreachable!(),
            }
        }
        BinaryOperation::GreaterThan => equality_float_operations(|x, y| x > y, value, state),
        BinaryOperation::LessThan => equality_float_operations(|x, y| x < y, value, state),
        BinaryOperation::Or => equality_bool_operations(|x, y| x || y, value, state),
        BinaryOperation::And => equality_bool_operations(|x, y| x && y, value, state),
    }
}

fn math_operations<F>(math_operation: F, value: Node, state: &mut State) -> Result<(), String>
where
    F: Fn(f32, f32) -> f32,
{
    match (state.stack.last().unwrap().current.clone(), value) {
        (Node::Float(float_x), Node::Float(float_y)) => {
            let new_current = Node::Float(math_operation(float_x, float_y));
            state.stack.last_mut().unwrap().current = new_current;
            Ok(())
        }
        (Node::Float(float_x), Node::Variable(var_name)) => {
            let var_value = state
                .stack
                .last()
                .unwrap()
                .variables
                .get(&var_name)
                .unwrap();
            if let Node::Float(float_y) = var_value {
                let new_current = Node::Float(math_operation(float_x, *float_y));
                state.stack.last_mut().unwrap().current = new_current;
                Ok(())
            } else {
                Err("Variable is not float".to_string())
            }
        }
        _ => unreachable!(),
    }
}

fn equality_float_operations<F>(
    equality_operation: F,
    value: Node,
    state: &mut State,
) -> Result<(), String>
where
    F: Fn(f32, f32) -> bool,
{
    match (state.stack.last().unwrap().current.clone(), value) {
        (Node::Float(float_x), Node::Float(float_y)) => {
            let new_current = Node::Boolean(equality_operation(float_x, float_y));
            state.stack.last_mut().unwrap().current = new_current;
            Ok(())
        }
        (Node::Float(float_x), Node::Variable(var_name)) => {
            let var_value = state
                .stack
                .last()
                .unwrap()
                .variables
                .get(&var_name)
                .unwrap();
            if let Node::Float(float_y) = var_value {
                let new_current = Node::Boolean(equality_operation(float_x, *float_y));
                state.stack.last_mut().unwrap().current = new_current;
                Ok(())
            } else {
                Err("Variable is not float".to_string())
            }
        }
        _ => unreachable!(),
    }
}

fn equality_bool_operations<F>(
    bool_operation: F,
    value: Node,
    state: &mut State,
) -> Result<(), String>
where
    F: Fn(bool, bool) -> bool,
{
    match (state.stack.last().unwrap().current.clone(), value) {
        (Node::Boolean(bool_x), Node::Boolean(bool_y)) => {
            let new_current = Node::Boolean(bool_operation(bool_x, bool_y));
            state.stack.last_mut().unwrap().current = new_current;
            Ok(())
        }
        (Node::Boolean(bool_x), Node::Variable(var_name)) => {
            let var_value = state
                .stack
                .last()
                .unwrap()
                .variables
                .get(&var_name)
                .unwrap();
            if let Node::Boolean(bool_y) = var_value {
                let new_current = Node::Boolean(bool_operation(bool_x, *bool_y));
                state.stack.last_mut().unwrap().current = new_current;
                Ok(())
            } else {
                Err("Variable is not boolean".to_string())
            }
        }
        _ => unreachable!(),
    }
}

fn evaluate_unary(op: UnaryOperation, state: &mut State) -> Result<(), String> {
    match op {
        UnaryOperation::Not => {
            match state.stack.last().unwrap().current.clone() {
                Node::Boolean(bool) => {
                    let new_current = Node::Boolean(!bool);
                    state.stack.last_mut().unwrap().current = new_current;
                }
                _ => unreachable!(),
            }
            Ok(())
        }
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
