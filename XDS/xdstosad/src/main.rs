mod usage;
mod parser;
mod XYZ;
mod Geom;
mod XDSdatum;

use crate::usage::usage::hello;
// pub use crate::parser::parser::Parser;

fn main() {
    hello();
    let myparser = parser::Parser::parseinput();
    // let xdsdata: Vec<XDSdatum> = XDSdatum::readdata(myparser.xdsascii_file_);
    println!("Hello, world!");
}
