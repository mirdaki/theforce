use std::{
    collections::HashMap,
    io::{Read, Write},
};

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

struct State<R, W> {
    functions: HashMap<String, Function>,
    stack: Vec<Frame>,
    reader: R,
    writer: W,
}

impl<R, W> State<R, W>
where
    R: Read,
    W: Write,
{
    fn new(reader: R, writer: W) -> State<R, W> {
        State {
            functions: HashMap::new(),
            stack: vec![Frame::new()],
            reader,
            writer,
        }
    }

    fn get_current(&self) -> Result<&Node, String> {
        match self.stack.last() {
            Some(frame) => Ok(&frame.current),
            _ => Err("Current not found".to_string()),
        }
    }

    fn set_current(&mut self, new_current: Node) -> Result<(), String> {
        match self.stack.last_mut() {
            Some(frame) => {
                frame.current = new_current;
                Ok(())
            }
            _ => Err("No last frame".to_string()),
        }
    }
}

pub fn evaluate<R, W>(ast: Vec<Node>, reader: R, writer: W) -> Result<(), String>
where
    R: Read,
    W: Write,
{
    let mut main = Node::Noop;
    let state = &mut State::new(reader, writer);

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

fn evaluate_node<R, W>(ast: &Node, state: &mut State<R, W>) -> Result<(), String>
where
    R: Read,
    W: Write,
{
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
            state.set_current(ast.clone())?;
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
                state.set_current(possible_return)?;
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
            state.set_current(ast.clone())?;
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
            writeln!(state.writer, "{}", value).map_err(|x| x.to_string())
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
        Node::ReadBoolean(variable) => {
            // Validate input is variable
            let variable_name = if let Node::Variable(variable_name) = &**variable {
                variable_name.clone()
            } else {
                return Err("Not a variable".to_string());
            };

            let mut input = Vec::<u8>::new();
            if state.reader.read_to_end(&mut input).is_err() {
                return Err("Unable to read string".to_string());
            }

            let input = if let Ok(input) = std::str::from_utf8(&input) {
                input.to_string()
            } else {
                return Err("Unable to convert string".to_string());
            };

            let input = if let Ok(input) = input.parse::<bool>() {
                input
            } else {
                return Err("Unable to convert bool".to_string());
            };

            state
                .stack
                .last_mut()
                .unwrap()
                .variables
                .insert(variable_name, Node::Boolean(input));
            Ok(())
        }
        Node::ReadFloat(variable) => {
            // Validate input is variable
            let variable_name = if let Node::Variable(variable_name) = &**variable {
                variable_name.clone()
            } else {
                return Err("Not a variable".to_string());
            };

            let mut input = Vec::<u8>::new();
            if state.reader.read_to_end(&mut input).is_err() {
                return Err("Unable to read string".to_string());
            }

            let input = if let Ok(input) = std::str::from_utf8(&input) {
                input.to_string()
            } else {
                return Err("Unable to convert string".to_string());
            };

            let input = if let Ok(input) = input.parse::<f32>() {
                input
            } else {
                return Err("Unable to convert float".to_string());
            };

            state
                .stack
                .last_mut()
                .unwrap()
                .variables
                .insert(variable_name, Node::Float(input));
            Ok(())
        }
        Node::ReadString(variable) => {
            // Validate input is variable
            let variable_name = if let Node::Variable(variable_name) = &**variable {
                variable_name.clone()
            } else {
                return Err("Not a variable".to_string());
            };

            let mut input = Vec::<u8>::new();
            if state.reader.read_to_end(&mut input).is_err() {
                return Err("Unable to read string".to_string());
            }

            let input = if let Ok(input) = std::str::from_utf8(&input) {
                input.to_string()
            } else {
                return Err("Unable to convert string".to_string());
            };

            state
                .stack
                .last_mut()
                .unwrap()
                .variables
                .insert(variable_name, Node::String(input));
            Ok(())
        }
        Node::String(_) => {
            state.set_current(ast.clone())?;
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
            state.set_current(value)?;
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

fn evaluate_binary<R, W>(
    op: &BinaryOperation,
    value: &Node,
    state: &mut State<R, W>,
) -> Result<(), String>
where
    R: Read,
    W: Write,
{
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

fn math_operations<R, W, F>(
    math_operation: F,
    value: &Node,
    state: &mut State<R, W>,
) -> Result<(), String>
where
    R: Read,
    W: Write,
    F: Fn(f32, f32) -> f32,
{
    match (&state.get_current()?, value) {
        (Node::Float(float_x), Node::Float(float_y)) => {
            let new_current = Node::Float(math_operation(*float_x, *float_y));
            state.set_current(new_current)?;
            Ok(())
        }
        (Node::Float(float_x), Node::Variable(var_name)) => {
            let var_value = state.stack.last().unwrap().variables.get(var_name).unwrap();
            if let Node::Float(float_y) = var_value {
                let new_current = Node::Float(math_operation(*float_x, *float_y));
                state.set_current(new_current)?;
                Ok(())
            } else {
                Err("Variable is not float".to_string())
            }
        }
        _ => unreachable!(),
    }
}

fn equality_float_operations<R, W, F>(
    equality_operation: F,
    value: &Node,
    state: &mut State<R, W>,
) -> Result<(), String>
where
    R: Read,
    W: Write,
    F: Fn(f32, f32) -> bool,
{
    match (&state.get_current()?, value) {
        (Node::Float(float_x), Node::Float(float_y)) => {
            let new_current = Node::Boolean(equality_operation(*float_x, *float_y));
            state.set_current(new_current)?;
            Ok(())
        }
        (Node::Float(float_x), Node::Variable(var_name)) => {
            let var_value = state.stack.last().unwrap().variables.get(var_name).unwrap();
            if let Node::Float(float_y) = var_value {
                let new_current = Node::Boolean(equality_operation(*float_x, *float_y));
                state.set_current(new_current)?;
                Ok(())
            } else {
                Err("Variable is not float".to_string())
            }
        }
        _ => unreachable!(),
    }
}

fn equality_bool_operations<R, W, F>(
    bool_operation: F,
    value: &Node,
    state: &mut State<R, W>,
) -> Result<(), String>
where
    R: Read,
    W: Write,
    F: Fn(bool, bool) -> bool,
{
    match (&state.get_current()?, value) {
        (Node::Boolean(bool_x), Node::Boolean(bool_y)) => {
            let new_current = Node::Boolean(bool_operation(*bool_x, *bool_y));
            state.set_current(new_current)?;
            Ok(())
        }
        (Node::Boolean(bool_x), Node::Variable(var_name)) => {
            let var_value = state.stack.last().unwrap().variables.get(var_name).unwrap();
            if let Node::Boolean(bool_y) = var_value {
                let new_current = Node::Boolean(bool_operation(*bool_x, *bool_y));
                state.set_current(new_current)?;
                Ok(())
            } else {
                Err("Variable is not boolean".to_string())
            }
        }
        _ => unreachable!(),
    }
}

fn evaluate_unary<R, W>(op: &UnaryOperation, state: &mut State<R, W>) -> Result<(), String>
where
    R: Read,
    W: Write,
{
    match op {
        UnaryOperation::Not => {
            match state.get_current()? {
                Node::Boolean(bool) => {
                    let new_current = Node::Boolean(!bool);
                    state.set_current(new_current)?;
                }
                _ => unreachable!(),
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn hello_there() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![Node::Print(Box::new(Node::String(
            "Hello there".to_string(),
        )))])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "Hello there\n");
    }

    #[test]
    fn variable() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareFloat("jawa".to_string(), Box::new(Node::Float(-13.2))),
            Node::Print(Box::new(Node::Variable("jawa".to_string()))),
        ])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "-13.2\n");

        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareString(
                "ewok".to_string(),
                Box::new(Node::String("Nub Nub".to_string())),
            ),
            Node::Print(Box::new(Node::Variable("ewok".to_string()))),
        ])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "Nub Nub\n");

        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareBoolean("darkSide".to_string(), Box::new(Node::Boolean(true))),
            Node::Print(Box::new(Node::Variable("darkSide".to_string()))),
        ])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "true\n");
    }

    #[test]
    fn math() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareFloat("porg".to_string(), Box::new(Node::Float(4.0))),
            Node::AssignVariable(
                "porg".to_string(),
                Box::new(Node::Variable("porg".to_string())),
                vec![
                    Node::Binary(BinaryOperation::Add, Box::new(Node::Float(2.0))),
                    Node::Binary(BinaryOperation::Subtract, Box::new(Node::Float(1.0))),
                    Node::Binary(BinaryOperation::Multiply, Box::new(Node::Float(3.0))),
                    Node::Binary(BinaryOperation::Divide, Box::new(Node::Float(5.0))),
                    Node::Binary(BinaryOperation::Exponent, Box::new(Node::Float(2.0))),
                    Node::Binary(BinaryOperation::Modulus, Box::new(Node::Float(10.0))),
                ],
            ),
            Node::Print(Box::new(Node::Variable("porg".to_string()))),
        ])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "9\n");
    }

    #[test]
    fn equality() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareFloat("anakin".to_string(), Box::new(Node::Float(27700.0))),
            Node::DeclareFloat("luke".to_string(), Box::new(Node::Float(14500.0))),
            Node::DeclareFloat("leia".to_string(), Box::new(Node::Float(14500.0))),
            Node::DeclareBoolean("midichlorian".to_string(), Box::new(Node::Boolean(false))),
            Node::AssignVariable(
                "midichlorian".to_string(),
                Box::new(Node::Variable("luke".to_string())),
                vec![Node::Binary(
                    BinaryOperation::GreaterThan,
                    Box::new(Node::Variable("anakin".to_string())),
                )],
            ),
            Node::Print(Box::new(Node::Variable("midichlorian".to_string()))),
            Node::AssignVariable(
                "midichlorian".to_string(),
                Box::new(Node::Variable("anakin".to_string())),
                vec![Node::Binary(
                    BinaryOperation::LessThan,
                    Box::new(Node::Variable("leia".to_string())),
                )],
            ),
            Node::Print(Box::new(Node::Variable("midichlorian".to_string()))),
            Node::AssignVariable(
                "midichlorian".to_string(),
                Box::new(Node::Variable("leia".to_string())),
                vec![Node::Binary(
                    BinaryOperation::Equal,
                    Box::new(Node::Variable("luke".to_string())),
                )],
            ),
            Node::Print(Box::new(Node::Variable("midichlorian".to_string()))),
        ])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "false\nfalse\ntrue\n");
    }

    #[test]
    fn logic() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareBoolean("lightside".to_string(), Box::new(Node::Boolean(true))),
            Node::DeclareBoolean("darkside".to_string(), Box::new(Node::Boolean(false))),
            Node::DeclareBoolean("revan".to_string(), Box::new(Node::Boolean(false))),
            Node::AssignVariable(
                "revan".to_string(),
                Box::new(Node::Variable("lightside".to_string())),
                vec![Node::Binary(
                    BinaryOperation::Or,
                    Box::new(Node::Variable("darkside".to_string())),
                )],
            ),
            Node::Print(Box::new(Node::Variable("revan".to_string()))),
            Node::AssignVariable(
                "revan".to_string(),
                Box::new(Node::Variable("revan".to_string())),
                vec![Node::Binary(
                    BinaryOperation::And,
                    Box::new(Node::Variable("lightside".to_string())),
                )],
            ),
            Node::Print(Box::new(Node::Variable("revan".to_string()))),
            Node::AssignVariable(
                "revan".to_string(),
                Box::new(Node::Variable("revan".to_string())),
                vec![Node::Unary(UnaryOperation::Not)],
            ),
            Node::Print(Box::new(Node::Variable("revan".to_string()))),
        ])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "true\ntrue\nfalse\n");
    }

    #[test]
    fn while_loop() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareFloat("deathStars".to_string(), Box::new(Node::Float(3.0))),
            Node::While(
                Box::new(Node::Variable("deathStars".to_string())),
                vec![
                    Node::Print(Box::new(Node::Variable("deathStars".to_string()))),
                    Node::AssignVariable(
                        "deathStars".to_string(),
                        Box::new(Node::Variable("deathStars".to_string())),
                        vec![Node::Binary(
                            BinaryOperation::Subtract,
                            Box::new(Node::Float(1.0)),
                        )],
                    ),
                ],
            ),
        ])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "3\n2\n1\n");
    }

    #[test]
    fn for_loop() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareFloat("deadYounglings".to_string(), Box::new(Node::Float(0.0))),
            Node::For(
                Box::new(Node::Float(10.0)),
                Box::new(Node::Variable("deadYounglings".to_string())),
                vec![Node::Print(Box::new(Node::Variable(
                    "deadYounglings".to_string(),
                )))],
            ),
        ])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "0\n1\n2\n3\n4\n5\n6\n7\n8\n9\n");
    }

    #[test]
    fn if_else() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![Node::If(
            Box::new(Node::Boolean(true)),
            vec![Node::Print(Box::new(Node::String("Do".to_string())))],
            vec![Node::Print(Box::new(Node::String("Don't".to_string())))],
        )])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "Do\n");
    }

    #[test]
    fn functions() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![
            Node::DeclareFunction(
                "NameTheSystem".to_string(),
                vec![Node::Variable("planet".to_string())],
                vec![
                    Node::Print(Box::new(Node::String("Goodbye".to_string()))),
                    Node::Print(Box::new(Node::Variable("planet".to_string()))),
                    Node::Print(Box::new(Node::String("Deathstar noise".to_string()))),
                ],
                true,
            ),
            Node::Main(vec![Node::CallFunction(
                "NameTheSystem".to_string(),
                vec![Node::String("Alderaan".to_string())],
            )]),
        ];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "Goodbye\nAlderaan\nDeathstar noise\n");

        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![
            Node::DeclareFunction(
                "TheOdds".to_string(),
                vec![Node::Variable("odds".to_string())],
                vec![
                    Node::DeclareBoolean("survive".to_string(), Box::new(Node::Boolean(false))),
                    Node::AssignVariable(
                        "survive".to_string(),
                        Box::new(Node::Variable("odds".to_string())),
                        vec![
                            Node::Binary(BinaryOperation::Modulus, Box::new(Node::Float(3720.0))),
                            Node::Binary(BinaryOperation::Equal, Box::new(Node::Float(0.0))),
                        ],
                    ),
                    Node::Return(Box::new(Node::Variable("survive".to_string()))),
                ],
                false,
            ),
            Node::Main(vec![
                Node::DeclareBoolean("survive".to_string(), Box::new(Node::Boolean(false))),
                Node::AssignVariable(
                    "survive".to_string(),
                    Box::new(Node::CallFunction(
                        "TheOdds".to_string(),
                        vec![Node::Float(52.0)],
                    )),
                    vec![],
                ),
                Node::Print(Box::new(Node::Variable("survive".to_string()))),
            ]),
        ];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "false\n");
    }

    #[test]
    fn input() {
        let input = "3.14";
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareFloat("jawa".to_string(), Box::new(Node::Float(0.0))),
            Node::ReadFloat(Box::new(Node::Variable("jawa".to_string()))),
            Node::Print(Box::new(Node::Variable("jawa".to_string()))),
        ])];

        let result = evaluate(ast, input.as_bytes(), &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "3.14\n");

        let input = "Wicket";
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareString("ewok".to_string(), Box::new(Node::String("".to_string()))),
            Node::ReadString(Box::new(Node::Variable("ewok".to_string()))),
            Node::Print(Box::new(Node::Variable("ewok".to_string()))),
        ])];

        let result = evaluate(ast, input.as_bytes(), &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "Wicket\n");

        let input = "false";
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareBoolean("darkSide".to_string(), Box::new(Node::Boolean(true))),
            Node::ReadBoolean(Box::new(Node::Variable("darkSide".to_string()))),
            Node::Print(Box::new(Node::Variable("darkSide".to_string()))),
        ])];

        let result = evaluate(ast, input.as_bytes(), &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "false\n");
    }

    #[test]
    fn other() {
        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(Vec::new())];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "");

        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "");
    }
}
