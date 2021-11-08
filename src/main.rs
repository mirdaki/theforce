use std::io;

mod ast;
mod cli;
mod interpreter;
mod parser;

#[cfg(feature = "llvm")]
mod compiler;

fn main() -> Result<(), String> {
    let config = cli::parse_arguments()?;

    let source = cli::read_source(config)?;

    let ast = parser::parse(source.as_str());
    interpreter::evaluate(ast.unwrap(), io::stdin(), io::stdout())
}
