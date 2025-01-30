mod usage;
mod parser;
pub use crate::usage::usage::hello;
// pub use crate::parser::parser::Parser;

fn main() {
    hello();
    let myparsed = parser::Parser::parseinput();
    println!("Hello, world!");
}
