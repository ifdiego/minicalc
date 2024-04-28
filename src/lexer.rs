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

pub struct Buffer {
    opening: String,
    opening_chars: Vec<char>,
    pos: usize, // equals to size_t of C
    line: usize
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

    pub fn next_char(&mut self) -> char {
        let result = self.opening_chars[self.pos];
        self.pos += 1;
        result
    }

    pub fn back_pos(&mut self) {
        self.pos -= 1;
    }
}

pub fn next_token(buffer: &mut Buffer) -> Token {
    if buffer.is_end() {
        return Token::eof(buffer.line);
    }

    // check next character of the input
    match buffer.next_char() {
        '(' => Token::symbol(TokenType::BeginParethesis, buffer.line),
        ')' => Token::symbol(TokenType::CloseParenthesis, buffer.line),
        '+' => Token::symbol(TokenType::Sum, buffer.line),
        '*' => Token::symbol(TokenType::Asterisk, buffer.line),
        c if c.is_digit(10) => number_token(buffer, c),
        c if c.is_alphabetic() => word_token(buffer, c),
        _ => panic!("unexpected character")
    }
}

pub fn number_token(buffer: &mut Buffer, c: char) -> Token {
    // accumulate the digits in string, then convert to integer
    let mut digits: Vec<char> = vec!(c);

    let mut next = buffer.next_char();
    while next.is_digit(10) {
        digits.push(next);
        next = buffer.next_char();
    }

    // return the last character read to the buffer
    buffer.back_pos();

    let s: String = digits.iter().collect();
    let value = i64::from_str_radix(&s, 10).
        expect("error while converting from string to number");

    Token { kind: TokenType::Integer(value), line: buffer.line }
}

pub fn word_token(buffer: &mut Buffer, c: char) -> Token {
    let mut letters: Vec<char> = vec!(c);
    let mut next = buffer.next_char();

    while next.is_alphabetic() {
        letters.push(next);
        next = buffer.next_char();
    }

    let s: String = letters.iter().collect();
    if s != "print" {
        panic!("unrecognized keyword");
    }

    Token { kind: TokenType::Print, line: buffer.line }
}
