mod Det;
mod Geom;
mod XDSdatum;
mod XDSheader;
mod XYZ;
mod parser;
mod usage;

use crate::usage::usage::hello;
// pub use crate::parser::parser::Parser;

fn main() {
    hello();
    let myparser = match parser::Parser::parseinput() {
        Some(myparser) => myparser,
        None => {
            usage::usage::usage();
            panic!("Error parsing command line parameters");
        }
    };
    let xdsheader = match XDSheader::readheader(&myparser.xdsascii()) {
        Some(xdsheader) => xdsheader,
        None => {
            panic!("Error reading XDSheader information");
        }
    };
    if myparser.verbosity() > 0 {
        println!("Read header from {}", myparser.xdsascii());
    }
    let xdsdata = XDSdatum::readdata(myparser.xdsascii(), myparser.verbosity());
    let xdsdata = match xdsdata {
        Some(xdsdata) => xdsdata,
        None => {
            println!("Error reading data from XDS_ASCII.HKL");
            return ();
        }
    };
    if myparser.verbosity() > 0 {
        println!(
            "Read {} lines of data from {}",
            xdsdata.len(),
            myparser.xdsascii()
        );
    }
    println!("Hello, world!");
}
