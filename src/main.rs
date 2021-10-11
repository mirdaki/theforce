extern crate pest;
extern crate pest_derive;

use pest::Parser;

#[derive(Debug)]
pub enum Node {
    Print(String)
}

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct ForceParser;

fn parse(source: &str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = ForceParser::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Main = pair.as_rule() {
            ast.push(build_ast(pair));
        }
    }
    Ok(ast)
}

fn build_ast(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Main => {
            let mut pair = pair.into_inner();
            let string = pair.next().unwrap();
            Node::Print(string.as_span().as_str().to_string())
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn evaluate(ast: &Node) {
    match ast {
        Node::Print(x) => println!("{}", x),
    }
}

fn main() {
    let source = r#"
    Do it
        The Sacred Texts! "Hello there"
    May The Force be with you
    "#;
    let ast = parse(source);
    evaluate(ast.unwrap().first().unwrap());
}
