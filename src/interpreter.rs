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

#[derive(Clone)]
struct Function {
    parameters: Vec<Node>,
    body: Vec<Node>,
    void: bool,
}

struct State {
    functions: HashMap<String, Function>,
    stack: Vec<Frame>,
}

impl State {
    fn new() -> State {
        State {
            functions: HashMap::new(),
            stack: vec![Frame::new()],
        }
    }

    fn get_current(&self) -> Result<&Node, String> {
        match &self.stack.last() {
            Some(frame) => Ok(&frame.current),
            _ => Err("Current not found".to_string()),
        }
    }

    fn set_current(&mut self, new_current: Node) {
        self.stack.last_mut().unwrap().current = new_current;
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
            Node::DeclareFunction(function_name, parameters, body, void) => {
                let function = Function {
                    parameters: parameters.to_vec(),
                    body: body.to_vec(),
                    void: *void,
                };
                state.functions.insert(function_name.to_string(), function);
            }
            _ => unreachable!(), // TODO: Get actual error message
        }
    }

    evaluate_node(&main, state)
}

fn evaluate_node(ast: &Node, state: &mut State) -> Result<(), String> {
    match ast {
        Node::AssignVariable(variable_name, first_value, operations) => {
            // TODO: Check if value is actually a value?
            // Place value at top of stack
            evaluate_node(first_value, state)?;
            for operation in operations {
                // TOOD: Actual error message
                let _ = match operation {
                    Node::Binary(operation, value) => evaluate_binary(operation, value, state),
                    Node::Unary(operation) => evaluate_unary(operation, state),
                    _ => Err("Invalid operation".to_string()),
                };
            }
            let new_current = state.get_current()?.clone();
            state
                .stack
                .last_mut()
                .unwrap()
                .variables
                .insert(variable_name.to_string(), new_current);
            Ok(())
        }
        // Taken care of by the assign variable
        Node::Binary(_, _) => unreachable!(),
        Node::Boolean(_) => {
            state.set_current(ast.clone());
            Ok(())
        }
        Node::CallFunction(name, arguments) => {
            // Validate the function exists
            let function = if let Some(function) = state.functions.get(name) {
                function.clone()
            } else {
                return Err("Function not defined".to_string());
            };

            // // Validate the inputs match
            if arguments.len() != function.parameters.len() {
                return Err("Paramaters do not match arguments".to_string());
            }

            // Create a new frame in the stack
            let mut new_frame = Frame::new();
            for (argument, parameter) in arguments.iter().zip(function.parameters.iter()) {
                // Processes argument
                evaluate_node(argument, state)?;

                // Validate that arguments and parameters are the right types
                let parameter_name = if let Node::Variable(name) = parameter {
                    name
                } else {
                    return Err("Parameter is not a string".to_string());
                };

                if let Node::Float(_) | Node::Boolean(_) | Node::String(_) = state.get_current()? {
                    new_frame
                        .variables
                        .insert(parameter_name.clone(), state.get_current()?.clone());
                } else {
                    return Err("Argument not a value".to_string());
                }
            }
            state.stack.push(new_frame);

            // Evaluate the body
            for statment in &function.body {
                evaluate_node(statment, state)?;
            }

            // Pop the stack frame. If non-void, set the return value to the new current
            let possible_return = state.get_current()?.clone();
            state.stack.pop();
            if !function.void {
                state.set_current(possible_return);
            }

            Ok(())
        }
        Node::DeclareBoolean(name, boolean) => {
            if let Node::Boolean(value) = **boolean {
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name.to_string(), Node::Boolean(value));
                // TODO: Decide if replacing an existing var is an error
                Ok(())
            } else {
                Err("Not boolean".to_string())
            }
        }
        Node::DeclareFloat(name, float) => {
            if let Node::Float(value) = **float {
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name.to_string(), Node::Float(value));
                Ok(())
            } else {
                Err("Not float".to_string())
            }
        }
        // Done in the evaluate function
        Node::DeclareFunction(_, _, _, _) => unreachable!(),
        Node::DeclareString(name, string) => {
            if let Node::String(value) = &**string {
                state
                    .stack
                    .last_mut()
                    .unwrap()
                    .variables
                    .insert(name.to_string(), Node::String(value.to_string()));
                Ok(())
            } else {
                Err("Not string".to_string())
            }
        }
        Node::Float(_) => {
            state.set_current(ast.clone());
            Ok(())
        }
        Node::For(max, flag, statments) => {
            // Will be initiliaed, because of the match guard
            let max_value = if let Node::Float(max) = **max {
                max
            } else {
                return Err("For max not float".to_string());
            };

            let flag_var_name = if let Node::Variable(ref var_name) = **flag {
                var_name
            } else {
                return Err("For flag not vairable".to_string());
            };

            // Get the variable value
            evaluate_node(
                &state
                    .stack
                    .last()
                    .unwrap()
                    .variables
                    .get(flag_var_name)
                    .unwrap()
                    .clone(),
                state,
            )?;

            let mut flag_value = if let Node::Float(value) = state.get_current()?.clone() {
                value
            } else {
                return Err("Flag not a float".to_string());
            };

            // Check if should loop
            let mut continue_loop = !flag_value.eq(&max_value);

            // Loop
            while continue_loop {
                for statment in statments {
                    evaluate_node(statment, state)?;
                }

                // Get the variable value
                evaluate_node(
                    &state
                        .stack
                        .last()
                        .unwrap()
                        .variables
                        .get(flag_var_name)
                        .unwrap()
                        .clone(),
                    state,
                )?;

                flag_value = if let Node::Float(value) = state.get_current()?.clone() {
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
            evaluate_node(flag, state)?;

            // Only accept boolean results
            let if_flag = if let Node::Boolean(bool) = state.get_current()? {
                bool
            } else {
                return Err("Not boolean.".to_string());
            };

            // Choose a branch. False branch may not exist, but should be empty from parser
            let statments = if *if_flag {
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
            evaluate_node(node, state)?;
            let value = state.get_current()?.clone();
            println!("{}", value);
            Ok(())
        }
        Node::Return(node) => {
            evaluate_node(node, state)?;

            // Validate it's a value
            if let Node::Float(_) | Node::Boolean(_) | Node::String(_) = state.get_current()? {
                Ok(())
            } else {
                Err("Argument not a value".to_string())
            }
        }
        Node::ReadBoolean(_) => todo!(),
        Node::ReadFloat(_) => todo!(),
        Node::ReadString(_) => todo!(),
        Node::String(_) => {
            state.set_current(ast.clone());
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
                .get(name)
                .unwrap()
                .clone();
            state.set_current(value);
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
            evaluate_node(flag, state)?;
            let mut continue_loop = match state.get_current()? {
                Node::Boolean(bool) => *bool,
                Node::Float(float) => *float != 0.0,
                _ => unreachable!(),
            };

            while continue_loop {
                for statment in statments {
                    evaluate_node(statment, state)?;
                }
                let flag = state
                    .stack
                    .last()
                    .unwrap()
                    .loop_flag
                    .last()
                    .unwrap()
                    .clone();
                evaluate_node(&flag, state)?;
                continue_loop = match *state.get_current()? {
                    Node::Boolean(bool) => bool,
                    Node::Float(float) => float != 0.0,
                    _ => unreachable!(),
                };
            }
            state.stack.last_mut().unwrap().loop_flag.pop();
            Ok(())
        }
        Node::Noop => Ok(()),
    }
}

fn evaluate_binary(op: &BinaryOperation, value: &Node, state: &mut State) -> Result<(), String> {
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

fn math_operations<F>(math_operation: F, value: &Node, state: &mut State) -> Result<(), String>
where
    F: Fn(f32, f32) -> f32,
{
    match (&state.get_current()?, value) {
        (Node::Float(float_x), Node::Float(float_y)) => {
            let new_current = Node::Float(math_operation(*float_x, *float_y));
            state.set_current(new_current);
            Ok(())
        }
        (Node::Float(float_x), Node::Variable(var_name)) => {
            let var_value = state.stack.last().unwrap().variables.get(var_name).unwrap();
            if let Node::Float(float_y) = var_value {
                let new_current = Node::Float(math_operation(*float_x, *float_y));
                state.set_current(new_current);
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
    value: &Node,
    state: &mut State,
) -> Result<(), String>
where
    F: Fn(f32, f32) -> bool,
{
    match (&state.get_current()?, value) {
        (Node::Float(float_x), Node::Float(float_y)) => {
            let new_current = Node::Boolean(equality_operation(*float_x, *float_y));
            state.set_current(new_current);
            Ok(())
        }
        (Node::Float(float_x), Node::Variable(var_name)) => {
            let var_value = state.stack.last().unwrap().variables.get(var_name).unwrap();
            if let Node::Float(float_y) = var_value {
                let new_current = Node::Boolean(equality_operation(*float_x, *float_y));
                state.set_current(new_current);
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
    value: &Node,
    state: &mut State,
) -> Result<(), String>
where
    F: Fn(bool, bool) -> bool,
{
    match (&state.get_current()?, value) {
        (Node::Boolean(bool_x), Node::Boolean(bool_y)) => {
            let new_current = Node::Boolean(bool_operation(*bool_x, *bool_y));
            state.set_current(new_current);
            Ok(())
        }
        (Node::Boolean(bool_x), Node::Variable(var_name)) => {
            let var_value = state.stack.last().unwrap().variables.get(var_name).unwrap();
            if let Node::Boolean(bool_y) = var_value {
                let new_current = Node::Boolean(bool_operation(*bool_x, *bool_y));
                state.set_current(new_current);
                Ok(())
            } else {
                Err("Variable is not boolean".to_string())
            }
        }
        _ => unreachable!(),
    }
}

fn evaluate_unary(op: &UnaryOperation, state: &mut State) -> Result<(), String> {
    match op {
        UnaryOperation::Not => {
            match state.get_current()? {
                Node::Boolean(bool) => {
                    let new_current = Node::Boolean(!bool);
                    state.set_current(new_current);
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
