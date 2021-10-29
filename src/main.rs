use std::io::{self, Read};

mod ast;
mod compiler;
mod interpreter;
mod parser;

fn main() -> Result<(), String> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    if stdin.read_to_string(&mut buffer).is_err() {
        return Err("Could not parse input".to_string());
    };

    let ast = parser::parse(buffer.as_str());
    interpreter::evaluate(ast.unwrap(), io::stdin(), io::stdout())
}
