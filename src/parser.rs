extern crate pest;
extern crate pest_derive;

use pest::Parser;

use crate::ast::{Node, UnaryOperation, BinaryOperation};

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
        Rule::Main => {
            let pairs = pair.into_inner();
            let mut statments = Vec::<Node>::new();
            for pair in pairs.into_iter() {
                statments.push(build_ast(pair));
            }
            Node::Main(statments)
        }
        Rule::AssignStatement => {
            let mut pairs = pair.into_inner();
            let identfier = pairs.next().unwrap().as_str();
            let value = build_ast(pairs.next().unwrap());
            let mut operations = Vec::<Node>::new();
            for pair in pairs.into_iter() {
                operations.push(build_ast(pair));
            }
            Node::AssignVariable(identfier.to_string(), Box::new(value), operations)
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
        Rule::WhileStatement => {
            let mut pairs = pair.into_inner();
            let value = build_ast(pairs.next().unwrap());
            let mut statments = Vec::<Node>::new();
            for pair in pairs.into_iter() {
                statments.push(build_ast(pair));
            }
            Node::While(Box::new(value), statments)
        }
        Rule::If => {
            let mut pairs = pair.into_inner();
            let value = build_ast(pairs.next().unwrap());
            let mut if_statments = Vec::<Node>::new();
            let mut else_statments = Vec::<Node>::new();
            for pair in pairs.into_iter() {
                if pair.as_rule() == Rule::ElseClause {
                    for pair in pair.into_inner().into_iter() {
                        else_statments.push(build_ast(pair));
                    }
                    break;
                } 
                if_statments.push(build_ast(pair));
            }
            Node::If(Box::new(value), if_statments, else_statments)
        }
        Rule::NotOperator => {
            Node::Unary(UnaryOperation::Not)
        }
        Rule::AddOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::Add, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::SubtractOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::Subtract, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::MultiplyOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::Multiply, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::DivideOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::Divide, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::ExponentOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::Exponent, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::ModulusOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::Modulus, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::EqualOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::Equal, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::GreaterThanOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::GreaterThan, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::LessThanOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::LessThan, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::OrOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::Or, Box::new(build_ast(pair.next().unwrap())))
        }
        Rule::AndOperator => {
            let mut pair = pair.into_inner();
            Node::Binary(BinaryOperation::And, Box::new(build_ast(pair.next().unwrap())))
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
        },
        Rule::VariableName => {
            let name = pair.as_str();
            Node::Variable(name.to_string())
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
            Yoda, you seek Yoda. jawa
            Whoosa are youssa? -13.2
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());

        assert_eq!(
            ast.unwrap(),
            vec![Node::Main(vec!(
                    Node::DeclareFloat(
                        "jawa".to_string(),
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
            Yoda, you seek Yoda. porg
            Whoosa are youssa? 4

            What a piece of junk! porg
                I am your father. porg
                This will make a fine addition to my collection. 2.0
                Proceed with the countdown. 1
                There's too many of them! 3
                Not to worry, at least we are flying half a ship. 5
                Unlimited power! 2
                Never tell me the odds! 10
            The garbage will do.
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());

        assert_eq!(
            ast.unwrap(),
            vec![Node::Main(vec!(
                    Node::DeclareFloat(
                        "porg".to_string(),
                        Box::new(Node::Float(4.0))),

                    Node::AssignVariable(
                        "porg".to_string(),
                        Box::new(Node::Variable("porg".to_string())),
                        vec!(
                            Node::Binary(BinaryOperation::Add, Box::new(Node::Float(2.0))),
                            Node::Binary(BinaryOperation::Subtract, Box::new(Node::Float(1.0))),
                            Node::Binary(BinaryOperation::Multiply, Box::new(Node::Float(3.0))),
                            Node::Binary(BinaryOperation::Divide, Box::new(Node::Float(5.0))),
                            Node::Binary(BinaryOperation::Exponent, Box::new(Node::Float(2.0))),
                            Node::Binary(BinaryOperation::Modulus, Box::new(Node::Float(10.0))),
                        )
                )))
            ]
        );
    }

    #[test]
    fn equality() {
        let source = r#"
        Do it!
            Yoda, you seek Yoda. anakin
            Whoosa are youssa? 27700

            Yoda, you seek Yoda. luke
            Whoosa are youssa? 14500

            Yoda, you seek Yoda. leia
            Whoosa are youssa? 14500

            I am the senate! midichlorian
            Whoosa are youssa? No, that's not true. That's impossible!

            What a piece of junk! midichlorian
                I am your father. luke
                There is always a bigger fish. anakin
            The garbage will do.

            What a piece of junk! midichlorian
                I am your father. anakin
                Impressive. Most impressive. leia
            The garbage will do.

            What a piece of junk! midichlorian
                I am your father. leia
                You're a Jedi too, nice to meet you. luke
            The garbage will do.
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());

        assert_eq!(
            ast.unwrap(),
            vec![Node::Main(vec!(
                    Node::DeclareFloat(
                        "anakin".to_string(),
                        Box::new(Node::Float(27700.0))),

                    Node::DeclareFloat(
                        "luke".to_string(),
                        Box::new(Node::Float(14500.0))),

                    Node::DeclareFloat(
                        "leia".to_string(),
                        Box::new(Node::Float(14500.0))),

                    Node::DeclareBoolean(
                        "midichlorian".to_string(),
                        Box::new(Node::Boolean(false))),

                    Node::AssignVariable(
                        "midichlorian".to_string(),
                        Box::new(Node::Variable("luke".to_string())),
                        vec!(
                            Node::Binary(BinaryOperation::GreaterThan, Box::new(Node::Variable("anakin".to_string()))),
                        )),

                    Node::AssignVariable(
                        "midichlorian".to_string(),
                        Box::new(Node::Variable("anakin".to_string())),
                        vec!(
                            Node::Binary(BinaryOperation::LessThan, Box::new(Node::Variable("leia".to_string()))),
                        )),

                    Node::AssignVariable(
                        "midichlorian".to_string(),
                        Box::new(Node::Variable("leia".to_string())),
                        vec!(
                            Node::Binary(BinaryOperation::Equal, Box::new(Node::Variable("luke".to_string()))),
                        )),
                ))
            ]
        );
    }

    #[test]
    fn logic() {
        let source = r#"
        Do it!
            I am the senate! lightside
            Whoosa are youssa? From a certain point of view.

            I am the senate! darkside
            Whoosa are youssa? No, that's not true. That's impossible!

            I am the senate! revan
            Whoosa are youssa? No, that's not true. That's impossible!

            What a piece of junk! revan
                I am your father. lightside
                As you wish. darkside
            The garbage will do.

            What a piece of junk! revan
                I am your father. revan
                There is another. lightside
            The garbage will do.

            What a piece of junk! revan
                I am your father. revan
                Always with you what cannot be done.
            The garbage will do.
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());

        assert_eq!(
            ast.unwrap(),
            vec![Node::Main(vec!(
                    Node::DeclareBoolean(
                        "lightside".to_string(),
                        Box::new(Node::Boolean(true))),

                    Node::DeclareBoolean(
                        "darkside".to_string(),
                        Box::new(Node::Boolean(false))),

                    Node::DeclareBoolean(
                        "revan".to_string(),
                        Box::new(Node::Boolean(false))),


                    Node::AssignVariable(
                        "revan".to_string(),
                        Box::new(Node::Variable("lightside".to_string())),
                        vec!(
                            Node::Binary(BinaryOperation::Or, Box::new(Node::Variable("darkside".to_string()))),
                        )),

                    Node::AssignVariable(
                        "revan".to_string(),
                        Box::new(Node::Variable("revan".to_string())),
                        vec!(
                            Node::Binary(BinaryOperation::And, Box::new(Node::Variable("lightside".to_string()))),
                        )),

                    Node::AssignVariable(
                        "revan".to_string(),
                        Box::new(Node::Variable("revan".to_string())),
                        vec!(
                            Node::Unary(UnaryOperation::Not),
                        )),
                ))
            ]
        );
    }

    #[test]
    fn while_loop() {
        let source = r#"
        Do it!
            Yoda, you seek Yoda. deathStars
            Whoosa are youssa? 3

            Here we go again. deathStars
                What a piece of junk! deathStars
                    I am your father. deathStars
                    Proceed with the countdown. 1
                The garbage will do.
            Let the past die.
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());

        assert_eq!(
            ast.unwrap(),
            vec![Node::Main(vec!(
                    Node::DeclareFloat(
                        "deathStars".to_string(),
                        Box::new(Node::Float(3.0))),

                    Node::While(
                        Box::new(Node::Variable("deathStars".to_string())),
                        vec!(Node::AssignVariable(
                            "deathStars".to_string(),
                            Box::new(Node::Variable("deathStars".to_string())),
                            vec!(
                                Node::Binary(BinaryOperation::Subtract, Box::new(Node::Float(1.0))),
                            ))
                        ),
                    )
                ))
            ]
        );
    }

    #[test]
    fn if_else() {
        let source = r#"
        Do it!
            Do or do not. From a certain point of view.
                The Sacred Texts! "Do"
            These aren’t the droids you’re looking for.
                The Sacred Texts! "Don't"
            You have failed me for the last time.
        May The Force be with you.
        "#;
        let ast = parse(source);
        assert!(ast.is_ok());
    }
    
    #[test]
    fn methods() {
    }

    #[test]
    fn input() {
    }

    #[test]
    fn other() {
        // Empty main
        // Noop
        // Nothing at all

    }

    #[test]
    fn error_type() {
    }

    #[test]
    fn error_method() {
    }
}
