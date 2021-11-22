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
    interpreter::evaluate(ast.unwrap(), io::stdin().lock(), io::stdout())
}
