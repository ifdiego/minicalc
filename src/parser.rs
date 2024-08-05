use crate::lexer::next_token;
use crate::lexer::Buffer;
use crate::lexer::TokenType;

// can be one:
// - constant
// - sum (two operands)
// - multiply (two operands)
#[derive(Debug, PartialEq, Eq)]
pub enum Expression {
    Constant(i64),
    Sum(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Sum,
    Multiply,
}

pub fn parse(buffer: &mut Buffer) -> Expression {
    let tok = next_token(buffer);
    if tok.kind != TokenType::Print {
        panic!("unexpected token: {:?} at line {}", tok, buffer.line);
    }
    evaluate(buffer)
}

fn evaluate(buffer: &mut Buffer) -> Expression {
    let mut tok = next_token(buffer);

    match tok.kind {
        TokenType::Integer(n) => Expression::Constant(n),
        TokenType::BeginParethesis => {
            let op1 = evaluate(buffer);
            tok = next_token(buffer);
            let operator = match tok.kind {
                TokenType::Sum => Operator::Sum,
                TokenType::Asterisk => Operator::Multiply,
                _ => panic!("unexpected operator, found: {:?} at line {}", tok, tok.line),
            };
            let op2 = evaluate(buffer);
            tok = next_token(buffer);
            if tok.kind != TokenType::CloseParenthesis {
                panic!(
                    "expected closing parenthesis, found: {:?} at line {}",
                    tok, tok.line
                );
            }
            match operator {
                Operator::Sum => Expression::Sum(Box::new(op1), Box::new(op2)),
                Operator::Multiply => Expression::Multiply(Box::new(op1), Box::new(op2)),
            }
        }
        _ => panic!(
            "unexpected token at the beginning of expression: {:?} at line {}",
            tok, tok.line
        ),
    }
}

#[test]
fn constant() {
    let mut buffer = Buffer::create_com_string("print 42");
    let tree = parse(&mut buffer);
    assert_eq!(tree, Expression::Constant(42));
}

#[test]
fn sum() {
    let mut buffer = Buffer::create_com_string("print (4 + 7)");
    let tree = parse(&mut buffer);
    assert_eq!(
        tree,
        Expression::Sum(
            Box::new(Expression::Constant(4)),
            Box::new(Expression::Constant(7))
        )
    );
}

#[test]
fn calc() {
    let mut buffer = Buffer::create_com_string("print (4 + (39 * 6))");
    let tree = parse(&mut buffer);
    assert_eq!(
        tree,
        Expression::Sum(
            Box::new(Expression::Constant(4)),
            Box::new(Expression::Multiply(
                Box::new(Expression::Constant(39)),
                Box::new(Expression::Constant(6))
            ))
        )
    );
}
