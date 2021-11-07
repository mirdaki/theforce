use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::ast::{BinaryOperation, Node, UnaryOperation};

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

    fn get_variable(&self, variable_name: &str) -> Result<&Node, String> {
        let variable_node = match self.stack.last() {
            Some(frame) => frame.variables.get(variable_name),
            None => None,
        };

        match variable_node {
            Some(variable) => Ok(variable),
            None => Err("No varaible found".to_string()),
        }
    }

    fn set_variable(&mut self, variable_name: &str, variable_value: &Node) -> Result<(), String> {
        let variable_result = match self.stack.last_mut() {
            Some(frame) => Some(
                frame
                    .variables
                    .insert(variable_name.to_string(), variable_value.clone()),
            ),
            None => None,
        };
        match variable_result {
            Some(_) => Ok(()),
            None => Err("No last frame".to_string()),
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
            state.set_variable(variable_name, &new_current)
        }
        // Taken care of by the assign variable
        Node::Binary(_, _) => unreachable!(),
        Node::Boolean(_) => state.set_current(ast.clone()),
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
        // TODO: Decide if replacing an existing var is an error
        Node::DeclareBoolean(name, boolean) => match **boolean {
            Node::Boolean(value) => state.set_variable(name, &Node::Boolean(value)),
            _ => Err("Not boolean".to_string()),
        },
        Node::DeclareFloat(name, float) => match **float {
            Node::Float(value) => state.set_variable(name, &Node::Float(value)),
            _ => Err("Not float".to_string()),
        },
        // Done in the evaluate function
        Node::DeclareFunction(_, _, _, _) => unreachable!(),
        Node::DeclareString(name, string) => match &**string {
            Node::String(value) => state.set_variable(name, &Node::String(value.clone())),
            _ => Err("Not string".to_string()),
        },
        Node::Float(_) => state.set_current(ast.clone()),
        Node::For(max, flag, statments) => {
            // Validate params
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

            // For evaluation check
            let evaluate_loop_flag = |flag: &Node, max: f32| -> Result<bool, String> {
                match flag {
                    Node::Float(float) => Ok(!float.eq(&max)),
                    _ => Err("Flag not a float".to_string()),
                }
            };

            // Check if should loop
            evaluate_node(&state.get_variable(flag_var_name)?.clone(), state)?;
            let mut continue_loop = evaluate_loop_flag(state.get_current()?, max_value)?;

            // Loop
            while continue_loop {
                for statment in statments {
                    evaluate_node(statment, state)?;
                }

                // Get the variable value
                evaluate_node(&state.get_variable(flag_var_name)?.clone(), state)?;

                let flag_value = if let Node::Float(value) = state.get_current()?.clone() {
                    // Increment variable value
                    Node::Float(value + 1.0)
                } else {
                    return Err("Flag not a float".to_string());
                };

                // Set the variable value
                state.set_variable(flag_var_name, &flag_value)?;

                // Check if should loop
                continue_loop = evaluate_loop_flag(&flag_value, max_value)?;
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
            // Validate it's a value
            match **node {
                Node::Float(_) | Node::Boolean(_) | Node::String(_) | Node::Variable(_) => (),
                _ => return Err("Return not a value".to_string()),
            };

            // Get the value and print
            evaluate_node(node, state)?;
            let value = state.get_current()?.clone();
            write!(state.writer, "{}", value).map_err(|x| x.to_string())
        }
        Node::Return(node) => {
            // Put onto stack
            evaluate_node(node, state)?;

            // Validate it's a value
            match state.get_current()? {
                Node::Float(_) | Node::Boolean(_) | Node::String(_) => Ok(()),
                _ => Err("Return not a value".to_string()),
            }
        }
        Node::ReadBoolean(variable) => read_value(&**variable, Node::Boolean, state),
        Node::ReadFloat(variable) => read_value(&**variable, Node::Float, state),
        Node::ReadString(variable) => read_value(&**variable, Node::String, state),
        Node::String(_) => state.set_current(ast.clone()),
        // Taken care of by the assign variable
        Node::Unary(_) => unreachable!(),
        Node::Variable(name) => state.set_current(state.get_variable(name)?.clone()),
        Node::While(flag, statments) => {
            // Validate params
            let flag_var_name = if let Node::Variable(ref var_name) = **flag {
                var_name
            } else {
                return Err("While flag not vairable".to_string());
            };

            // While evaluation check
            let evaluate_loop_flag = |flag: &Node| -> Result<bool, String> {
                match flag {
                    Node::Boolean(boolean) => Ok(*boolean),
                    Node::Float(float) => Ok(*float != 0.0),
                    _ => Err("Flag not a boolean or float".to_string()),
                }
            };

            // Get the variable value and validate it
            evaluate_node(&state.get_variable(flag_var_name)?.clone(), state)?;
            let mut continue_loop = evaluate_loop_flag(state.get_current()?)?;

            // Start looping
            while continue_loop {
                for statment in statments {
                    evaluate_node(statment, state)?;
                }

                evaluate_node(&state.get_variable(flag_var_name)?.clone(), state)?;
                continue_loop = evaluate_loop_flag(state.get_current()?)?;
            }
            Ok(())
        }
        Node::Noop => Ok(()),
    }
}

fn read_value<V, F, R, W>(
    variable: &Node,
    function: F,
    state: &mut State<R, W>,
) -> Result<(), String>
where
    V: std::str::FromStr,
    R: Read,
    W: Write,
    F: Fn(V) -> Node,
{
    // Validate input is assigned to variable
    let variable_name = if let Node::Variable(variable_name) = variable {
        variable_name.clone()
    } else {
        return Err("Not a variable".to_string());
    };

    // Get input from user
    let mut input = String::new();
    if state.reader.read_to_string(&mut input).is_err() {
        return Err("Unable to read input".to_string());
    }

    // Clean the input and convert it
    let input = if let Ok(input) = input.trim().parse::<V>() {
        input
    } else {
        return Err("Unable to convert input".to_string());
    };

    state.set_variable(variable_name.as_str(), &function(input))
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
                equal_value = state.get_variable(var_name)?.clone()
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
            let var_value = state.get_variable(var_name)?;
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
            let var_value = state.get_variable(var_name)?;
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
            let var_value = state.get_variable(var_name)?;
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
        assert_eq!(output, "Hello there");
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
        assert_eq!(output, "-13.2");

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
        assert_eq!(output, "Nub Nub");

        let input = io::stdin();
        let mut output = Vec::new();
        let ast = vec![Node::Main(vec![
            Node::DeclareBoolean("darkSide".to_string(), Box::new(Node::Boolean(true))),
            Node::Print(Box::new(Node::Variable("darkSide".to_string()))),
        ])];

        let result = evaluate(ast, input, &mut output);
        assert!(result.is_ok());

        let output = String::from_utf8(output).expect("Not UTF-8");
        assert_eq!(output, "true");
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
        assert_eq!(output, "9");
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
        assert_eq!(output, "falsefalsetrue");
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
        assert_eq!(output, "truetruefalse");
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
        assert_eq!(output, "321");
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
        assert_eq!(output, "0123456789");
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
        assert_eq!(output, "Do");
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
        assert_eq!(output, "GoodbyeAlderaanDeathstar noise");

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
        assert_eq!(output, "false");
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
        assert_eq!(output, "3.14");

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
        assert_eq!(output, "Wicket");

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
        assert_eq!(output, "false");
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
