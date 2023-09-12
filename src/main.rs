use std::io;

mod ast;
mod cli;
mod interpreter;
mod parser;

#[cfg(feature = "llvm")]
mod compiler;

fn main() -> Result<(), String> {
    let args = cli::parse_arguments();
    let source = cli::read_source(args)?;

    let ast = parser::parse(source.as_str());

    if args.contains_id("parse_error") {
        match ast {
            Ok(t) => interpreter::evaluate(t, io::stdin().lock(), io::stdout()),
            Err(e) => Err(format!("Error encountered while parsing: {}", e).to_string()),
        }
    }
    else {
        interpreter::evaluate(ast.unwrap(), io::stdin().lock(), io::stdout())
    }
}
