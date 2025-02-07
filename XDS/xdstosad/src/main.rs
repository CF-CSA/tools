mod usage;
mod parser;
mod XYZ;
mod Geom;
mod XDSdatum;
mod XDSheader;

use crate::usage::usage::hello;
// pub use crate::parser::parser::Parser;

fn main() {
    hello();
    let myparser = match parser::Parser::parseinput() {
    	Some(myparser) => myparser,
	None       => { 
		usage::usage::usage();
		panic!("Error parsing command line parameters");
		},
		};
    let xdsheader = match XDSheader::readheader(myparser.xdsascii()) {
    	Some(xdsheader) => xdsheader,
	None => { panic!("Error reading XDSheader information"); },
};
    let xdsdata: Vec<XDSdatum::XDSdatum> = XDSdatum::readdata(myparser.xdsascii());
    println!("Hello, world!");
}
