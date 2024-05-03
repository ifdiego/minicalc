pub mod lexer;
pub mod parser;

use lexer::Buffer;

fn main() {
    let test1 = "print (4 + 3)";
    let mut buffer1 = Buffer::create_com_string(test1);

    println!("Minicalc interpreter");
    let ast = parser::parse(&mut buffer1);
    // TODO: make treewalk, to obtain programs result
    println!("ast: {:?}", ast);
}
