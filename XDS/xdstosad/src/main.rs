mod usage;
mod parser;
use crate::XDSdatum;
use crate::usage::usage::hello;
// pub use crate::parser::parser::Parser;

fn main() {
    hello();
    let myparser = parser::Parser::parseinput();
    let xdsdata: Vec<XDSDatum> = XDSdatum::readdata(myparser.xdsascii_file_);
    println!("Hello, world!");
}
