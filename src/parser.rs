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
    dbg!(&pair);
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
        }
        Rule::DeclareBooleanStatement => {
            let mut pair = pair.into_inner();
            let identifier = pair.next().unwrap().as_str();
            let value = build_ast(pair.next().unwrap());
            Node::DeclareBoolean(identifier.to_string(), Box::new(value))
        }
        Rule::DeclareFloatStatement => {
            let mut pair = pair.into_inner();
            let identifier = pair.next().unwrap().as_str();
            let value = build_ast(pair.next().unwrap());
            Node::DeclareFloat(identifier.to_string(), Box::new(value))
        }
        Rule::DeclareStringStatement => {
            let mut pair = pair.into_inner();
            let identifier = pair.next().unwrap().as_str();
            let value = build_ast(pair.next().unwrap());
            Node::DeclareString(identifier.to_string(), Box::new(value))
        }
        Rule::PrintStatement => {
            let mut pair = pair.into_inner();
            Node::Print(Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::Value => {
            let mut pair = pair.into_inner();
            build_ast(pair.next().unwrap())
        }
        Rule::Boolean => {
            let pair = pair.into_inner().next().unwrap();
            let bool = pair.as_rule() == Rule::True;
            Node::Boolean(bool)
        }
        Rule::Float => {
            let float = pair.as_str();
            let float = float.parse::<f32>().unwrap();
            Node::Float(float)
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
                    Node::Print(
                        Box::new(Node::String("Hello there".to_string()))
                )))
            ]
        );
    }
    
    #[test]
    fn variable() {
        let source = r#"
        Do it!
            Yoda, you seek Yoda. porg
            Whoosa are youssa? -13.2
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());

        assert_eq!(
            ast.unwrap(),
            vec![Node::Main(vec!(
                    Node::DeclareFloat(
                        "porg".to_string(),
                        Box::new(Node::Float(-13.2))
                )))
            ]
        );

        let source = r#"
        Do it!
            Size matters not. ewok
            Whoosa are youssa? "Nub Nub"
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());

        assert_eq!(
            ast.unwrap(),
            vec![Node::Main(vec!(
                    Node::DeclareString(
                        "ewok".to_string(),
                        Box::new(Node::String("Nub Nub".to_string()))
                )))
            ]
        );

        let source = r#"
        Do it!
            I am the senate! darkSide
            Whoosa are youssa? From a certain point of view.
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());

        assert_eq!(
            ast.unwrap(),
            vec![Node::Main(vec!(
                    Node::DeclareBoolean(
                        "darkSide".to_string(),
                        Box::new(Node::Boolean(true))
                )))
            ]
        );
    }

    #[test]
    fn math() {
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
                    Node::Print(
                        Box::new(Node::String("Hello there".to_string()))
                )))
            ]
        );
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
