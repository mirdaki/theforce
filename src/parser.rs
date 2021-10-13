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
        if let Rule::Methods = pair.as_rule() {
            for pair in pair.into_inner() {
                ast.push(build_ast(pair));
            }
        }
    }
    Ok(ast)
}

fn build_ast(pair: pest::iterators::Pair<Rule>) -> Node {
    // dbg!(&pair);
    match pair.as_rule() {
        Rule::Method => {
            let mut pair = pair.into_inner();
            build_ast(pair.next().unwrap())
        }
        Rule::Main => {
            let pairs = pair.into_inner();
            let mut statments = Vec::<Node>::new();
            for pair in pairs.into_iter() {
                statments.push(build_ast(pair));
            }
            Node::Main(statments)
        },
        Rule::PrintStatement => {
            let mut pair = pair.into_inner();
            Node::Print(Box::new(build_ast(pair.next().unwrap())))
        },
        Rule::Value => {
            let mut pair = pair.into_inner();
            build_ast(pair.next().unwrap())
        }
        Rule::String => {
            let string = pair.as_str();
            // Remove parenthesis from string
            Node::String(string[1..string.len()-1].to_string())
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn hello_there() {
        let source = r#"
        Do it!
            The Sacred Texts! "Hello there"
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());

        assert_eq!(
            ast.unwrap(),
            vec![Node::Main(vec!(
                    Node::Print(Box::new(
                        Node::String("Hello there".to_string())
                ))))
            ]
        );
    }
    
    // TODO: Fill out
    #[test]
    fn variable() {
        let source = r#"
        Do it!
            Yoda, you seek Yoda. porg
            Whoosa are youssa? 42
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());
    }

    #[test]
    fn math() {
    }

    #[test]
    fn logic() {
    }

    #[test]
    fn while_loop() {
    }

    #[test]
    fn if_else() {
    }
    
    #[test]
    fn methods() {
    }

    #[test]
    fn input() {
    }

    #[test]
    fn error_type() {
    }

    #[test]
    fn error_method() {
    }
}
