extern crate pest;
extern crate pest_derive;

use pest::Parser;

use crate::ast::Node;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct ForceParser;

pub fn parse(source: &str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    
    // Parser tests
    #[test]
    fn hello_there() {
        let source = r#"
        Do it
            The Sacred Texts! "Hello there"
        May The Force be with you
        "#;
        let hello_there = parse(source);
        assert!(hello_there.is_ok());

        assert_eq!(
            hello_there.clone().unwrap(),
            vec![Node::Print("\"Hello there\"".to_string())]
        );
    }
}
