mod ast;
mod compiler;
mod interpreter;
mod parser;

fn main() {
    let source = r#"
    Do it!
        The Sacred Texts! "Hello there"
    May The Force be with you.
    "#;
    let ast = parser::parse(source);
    interpreter::evaluate(ast.unwrap().first().unwrap());
}
