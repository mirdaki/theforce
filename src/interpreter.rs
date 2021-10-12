use crate::ast::Node;

pub fn evaluate(ast: &Node) {
    match ast {
        Node::Print(x) => println!("{}", x),
    }
}
