pub mod lexer;

use lexer::Buffer;
use lexer::TokenType;

fn main() {
    let test1 = "(+*43257)";
    let mut buffer1 = Buffer::create_com_string(test1);

    println!("Minicalc interpreter");
    println!("Initializing Lexical analysis");

    let mut tok = lexer::next_token(&mut buffer1);
    while tok.kind != TokenType::Eof {
        println!("token: {:?}", tok);
        tok = lexer::next_token(&mut buffer1);
    }

    println!("Ending Lexical analysis");
}
