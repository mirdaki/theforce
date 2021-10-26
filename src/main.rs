use std::io;

mod ast;
mod compiler;
mod interpreter;
mod parser;

fn main() -> Result<(), String> {
    let source = r#"
    Do it!
        The Sacred Texts! "Hello there"
    May The Force be with you.
    "#;
    let ast = parser::parse(source);
    interpreter::evaluate(ast.unwrap(), io::stdin(), io::stdout())
}
