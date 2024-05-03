use crate::lexer::Buffer;
use crate::lexer::next_token;
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

pub fn parse(buffer: &mut Buffer) -> Expression {
    let tok = next_token(buffer);
    if tok.kind != TokenType::Print {
        panic!("unexpected token: {:?} at line {}", tok, buffer.line);
    }
    // TODO: finish parser
    Expression::Constant(42)
}
