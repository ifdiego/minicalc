pub mod lexer;
pub mod parser;

use lexer::Buffer;
use parser::Expression;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("should be able to read the file");

    let mut buffer1 = Buffer::create_com_string(&content);
    println!("Minicalc interpreter");

    let ast = parser::parse(&mut buffer1);
    println!("Ast: {:?}", ast);
    println!("Expression value: {}", eval(&ast));
}

fn eval(e: &Expression) -> i64 {
    match e {
        Expression::Constant(n) => *n,
        Expression::Sum(op1, op2) => eval(op1) + eval(op2),
        Expression::Multiply(op1, op2) => eval(op1) * eval(op2),
    }
}

#[test]
fn constant() {
    let mut buffer = Buffer::create_com_string("print 42");
    let e = parser::parse(&mut buffer);
    assert_eq!(eval(&e), 42);
}

#[test]
fn calc() {
    let mut buffer = Buffer::create_com_string("print (4 + (39 * 6))");
    let e = parser::parse(&mut buffer);
    assert_eq!(eval(&e), 238);
}
