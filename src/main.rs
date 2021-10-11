// use inkwell::context::Context;
// use inkwell::execution_engine::JitFunction;
// use inkwell::OptimizationLevel;
extern crate pest;
extern crate pest_derive;

use pest::Parser;

pub enum Node {
    // Int(i32),
    String(String),
    // Op1(String, Expression),
    // Op2(String, Expression, Expression),
    // Function(String, Vec<Expression>),
    BeginMain,
    EndMain,
    Print(Box<Node>)
}

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct ForceParser;

fn parse(source: &str) -> Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = ForceParser::parse(Rule::Program, source)?;
    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        // Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        // Rule::UnaryExpr => {
        //     let mut pair = pair.into_inner();
        //     let op = pair.next().unwrap();
        //     let child = pair.next().unwrap();
        //     let child = build_ast_from_term(child);
        //     parse_unary_expr(op, child)
        // }
        // Rule::BinaryExpr => {
        //     let mut pair = pair.into_inner();
        //     let lhspair = pair.next().unwrap();
        //     let lhs = build_ast_from_term(lhspair);
        //     let op = pair.next().unwrap();
        //     let rhspair = pair.next().unwrap();
        //     let rhs = build_ast_from_term(rhspair);
        //     parse_binary_expr(op, lhs, rhs)
        // }
        Rule::Main => {
            let mut pair = pair.into_inner();
            let begin_main = pair.next().unwrap();
            let expr = pair.next().unwrap();
            let end_main = pair.next().unwrap();
            build_ast_from_expr(expr)
        }
        Rule::Expr => {
            let mut pair = pair.into_inner();
            let print = pair.next().unwrap();
            let string = pair.next().unwrap();
            Node::Print(Box::new(Node::String(string.to_string())))
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn evaluate(exp: Node) {
    match exp {
        Node::String(x) => println!("{}", x),
    }
}

fn main() {
    let exp = Node::String("Hello there".to_string());
    evaluate(exp);
}

/* pub struct String {
    content: Vec<i8>
}

impl String {
    fn new() -> String {
        String {
            content: Vec::new()
        }
    } 
}

fn main() {
    // ANCHOR: first
    let context = Context::create();
    let module = context.create_module("addition");
    let i32_type = context.i32_type();
    // ANCHOR_END: first

    // ANCHOR: second
    let fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let fn_val = module.add_function("add", fn_type, None);
    let entry_basic_block = context.append_basic_block(fn_val, "entry");

    let builder = context.create_builder();
    builder.position_at_end(entry_basic_block);
    // ANCHOR_END: second

    // ANCHOR: third
    let x = fn_val.get_nth_param(0).unwrap().into_int_value();
    let y = fn_val.get_nth_param(1).unwrap().into_int_value();

    let ret = builder.build_int_add(x, y, "add");
    let return_instruction = builder.build_return(Some(&ret));
    // ANCHOR_END: third

    dbg!("module: {:?}", module.clone());
    dbg!("builder: {:?}", builder);
    assert_eq!(return_instruction.get_num_operands(), 1);
    
    // ANCHOR: fourth
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    unsafe {
        type Addition = unsafe extern "C" fn(i32, i32) -> i32;
        let add: JitFunction<Addition> = execution_engine.get_function("add").unwrap();
        let x = 1;
        let y = 2;
        assert_eq!(add.call(x, y), x + y);
    }
    // ANCHOR_END: fourth
}
 */
