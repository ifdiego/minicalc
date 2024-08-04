pub mod lexer;
pub mod parser;

use lexer::Buffer;
use parser::Expression;

fn main() {
    let test1 = "print (4 + (39 * 6))";
    let mut buffer1 = Buffer::create_com_string(test1);

    println!("Minicalc interpreter");

    let ast = parser::parse(&mut buffer1);
    println!("ast: {:?}", ast);
    println!("expression value: {}", eval(&ast));
}

fn eval(e: &Expression) -> i64 {
    match e {
        Expression::Constant(n) => *n,
        Expression::Sum(op1, op2) => eval(op1) + eval(op2),
        Expression::Multiply(op1, op2) => eval(op1) * eval(op2),
    }
}

#[test]
fn test1_main() {
    let mut buffer = Buffer::create_com_string("print 42");
    let e = parser::parse(&mut buffer);
    assert_eq!(eval(&e), 42);
}

#[test]
fn test2_main() {
    let mut buffer = Buffer::create_com_string("print (4 + (39 * 6))");
    let e = parser::parse(&mut buffer);
    assert_eq!(eval(&e), 238);
}
