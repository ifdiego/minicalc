use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    BeginParethesis,
    CloseParenthesis,
    Sum,
    Asterisk,
    Integer(i64),
    Print,
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
}

impl Token {
    pub fn symbol(kind: TokenType, line: usize) -> Token {
        Token { kind, line }
    }
    pub fn eof(line: usize) -> Token {
        Token {
            kind: TokenType::Eof,
            line,
        }
    }
}

pub struct Buffer<'a> {
    chars: Peekable<Chars<'a>>,
    pub line: usize,
}

impl<'a> Buffer<'a> {
    pub fn create_com_string(s: &'a str) -> Buffer<'a> {
        Buffer {
            chars: s.chars().peekable(),
            line: 1,
        }
    }

    pub fn is_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    pub fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    pub fn empty_space(&mut self) {
        while let Some(&c) = self.peek_char() {
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                }
                self.next_char();
            } else {
                break;
            }
        }
    }
}

pub fn next_token(buffer: &mut Buffer) -> Token {
    if buffer.is_end() {
        return Token::eof(buffer.line);
    }

    buffer.empty_space();
    match buffer.next_char() {
        Some('(') => Token::symbol(TokenType::BeginParethesis, buffer.line),
        Some(')') => Token::symbol(TokenType::CloseParenthesis, buffer.line),
        Some('+') => Token::symbol(TokenType::Sum, buffer.line),
        Some('*') => Token::symbol(TokenType::Asterisk, buffer.line),
        Some(c) if c.is_digit(10) => number_token(buffer, c),
        Some(c) if c.is_alphabetic() => word_token(buffer, c),
        None => Token::eof(buffer.line),
        Some(c) => panic!("unexpected character: {} at line {}", c, buffer.line),
    }
}

pub fn number_token(buffer: &mut Buffer, c: char) -> Token {
    let mut digits: Vec<char> = vec![c];

    while let Some(&next) = buffer.peek_char() {
        if next.is_digit(10) {
            digits.push(next);
            buffer.next_char();
        } else {
            break;
        }
    }

    let s: String = digits.iter().collect();
    let value = i64::from_str_radix(&s, 10).expect("error while converting from string to number");

    Token {
        kind: TokenType::Integer(value),
        line: buffer.line,
    }
}

pub fn word_token(buffer: &mut Buffer, c: char) -> Token {
    let mut letters: Vec<char> = vec![c];

    while let Some(&next) = buffer.peek_char() {
        if next.is_alphabetic() {
            letters.push(next);
            buffer.next_char();
        } else {
            break;
        }
    }

    let s: String = letters.iter().collect();
    if s != "print" {
        panic!("unrecognized keyword: {} at line {}", s, buffer.line);
    }

    Token {
        kind: TokenType::Print,
        line: buffer.line,
    }
}

#[test]
#[rustfmt::skip]
fn test1_lexer() {
    let test1 = "( + * print  \n\n43257)   ";
    let mut buffer1 = Buffer::create_com_string(test1);
    assert_eq!(next_token(&mut buffer1), Token { kind: TokenType::BeginParethesis, line: 1 });
    assert_eq!(next_token(&mut buffer1), Token { kind: TokenType::Sum, line: 1 });
    assert_eq!(next_token(&mut buffer1), Token { kind: TokenType::Asterisk, line: 1 });
    assert_eq!(next_token(&mut buffer1), Token { kind: TokenType::Print, line: 1 });
    assert_eq!(next_token(&mut buffer1), Token { kind: TokenType::Integer(43257), line: 3 });
    assert_eq!(next_token(&mut buffer1), Token { kind: TokenType::CloseParenthesis, line: 3 });
    assert_eq!(next_token(&mut buffer1), Token { kind: TokenType::Eof, line: 3 });
}

#[test]
#[rustfmt::skip]
fn test2_lexer() {
    let test2 = "print (4 * (39 + 3))";
    let mut buffer2 = Buffer::create_com_string(test2);
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::Print, line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::BeginParethesis, line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::Integer(4), line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::Asterisk, line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::BeginParethesis, line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::Integer(39), line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::Sum, line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::Integer(3), line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::CloseParenthesis, line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::CloseParenthesis, line: 1 });
    assert_eq!(next_token(&mut buffer2), Token { kind: TokenType::Eof, line: 1 });
}
