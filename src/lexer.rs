#[derive(Debug, PartialEq)]
pub enum TokenType {
    BeginParethesis,
    CloseParenthesis,
    Sum,
    Asterisk,
    Integer(i64),
    Print,
    Eof
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenType,
    line: usize,
}

impl Token {
    pub fn symbol(kind: TokenType, line: usize) -> Token {
        Token { kind, line }
    }

    pub fn eof(line: usize) -> Token {
        Token { kind: TokenType::Eof, line }
    }
}

// TODO: use iterator peakable (https://doc.rust-lang.org/std/iter/struct.Peekable.html)
pub struct Buffer {
    opening: String,
    opening_chars: Vec<char>,
    pos: usize, // equals to size_t of C
    pub line: usize
}

impl Buffer {
    pub fn create_com_string(s: &str) -> Buffer {
        Buffer {
            opening: s.to_string(),
            opening_chars: s.to_string().chars().collect(),
            pos: 0,
            line: 1
        }
    }

    pub fn is_end(&self) -> bool {
        self.pos >= self.opening_chars.len()
    }

    pub fn next_char(&mut self) -> Option<char> {
        if self.is_end() {
            None
        } else {
            let result = self.opening_chars[self.pos];
            self.pos += 1;
            Some(result)
        }
    }

    pub fn back_pos(&mut self) {
        self.pos -= 1;
    }

    // check empty spaces in string, update position
    pub fn empty_space(&mut self) {
        while !self.is_end() && self.opening_chars[self.pos].is_whitespace() {
            if self.opening_chars[self.pos] == '\n' {
                self.line += 1;
            }
            self.pos += 1;
        }
    }
}

pub fn next_token(buffer: &mut Buffer) -> Token {
    if buffer.is_end() {
        return Token::eof(buffer.line);
    }

    buffer.empty_space();

    // check next character of the input
    match buffer.next_char() {
        Some('(') => Token::symbol(TokenType::BeginParethesis, buffer.line),
        Some(')') => Token::symbol(TokenType::CloseParenthesis, buffer.line),
        Some('+') => Token::symbol(TokenType::Sum, buffer.line),
        Some('*') => Token::symbol(TokenType::Asterisk, buffer.line),
        Some(c) if c.is_digit(10) => number_token(buffer, c),
        Some(c) if c.is_alphabetic() => word_token(buffer, c),
        None => Token::eof(buffer.line),
        Some(c) => panic!("unexpected character: {} at line {}", c, buffer.line)
    }
}

pub fn number_token(buffer: &mut Buffer, c: char) -> Token {
    // accumulate the digits in string, then convert to integer
    let mut digits: Vec<char> = vec!(c);
    let mut next = buffer.next_char();

    while next.is_some() && next.expect("q?").is_digit(10) {
        digits.push(next.expect("q?"));
        next = buffer.next_char();
    }

    // return the last character read to the buffer
    if next.is_some() {
        buffer.back_pos();
    }

    let s: String = digits.iter().collect();
    let value = i64::from_str_radix(&s, 10).
        expect("error while converting from string to number");

    Token { kind: TokenType::Integer(value), line: buffer.line }
}

pub fn word_token(buffer: &mut Buffer, c: char) -> Token {
    let mut letters: Vec<char> = vec!(c);
    let mut next = buffer.next_char();

    while next.is_some() && next.expect("q?").is_alphabetic() {
        letters.push(next.expect("q?"));
        next = buffer.next_char();
    }

    if next.is_some() {
        buffer.back_pos();
    }

    let s: String = letters.iter().collect();
    if s != "print" {
        panic!("unrecognized keyword: {} at line {}", s, buffer.line);
    }

    Token { kind: TokenType::Print, line: buffer.line }
}

#[test]
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
