use crate::ast::Node;

pub fn evaluate(ast: &Node) {
    match ast {
        Node::Main(statments) => {
            for statment in statments  {
                evaluate(statment);
            }
        }
        Node::Print(x) => println!("{}", x),
        _ => todo!()
    }
}
