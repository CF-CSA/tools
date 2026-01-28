use crate::usage::usage::usage;
use std::env;

#[derive(Clone)]
pub struct Parser {
    xdsascii_file: String,
    outdir: String,
    outfile: String,
    recursive: bool,
    verbosity: u8,
}

impl Parser {
    pub fn xdsascii(&self) -> String {
        return self.xdsascii_file.clone();
    }
    pub fn verbosity(&self) -> u8 {
        return self.verbosity;
    }
    pub fn outfile(&self) -> String {
        return self.outfile.clone();
    }
    pub fn parseinput() -> Option<Parser> {
        let args: Vec<String> = env::args().collect();
        let mut myparser = Parser {
            xdsascii_file: String::from("XDS_ASCII.HKL"),
            outdir: String::from("./"),
            outfile: String::from("xds.sad"),
            recursive: true,
            verbosity: 1,
        };

        for idx in 1..args.len() {
            println!("Option: {}", args[idx]);
            match args[idx].as_str() {
                "-h" | "-?" => {
                    usage();
                    return None;
                }
                // this code does not work in verbosity w/o spaces
                "-v" => {
                    let verbosity = match args[idx].len() {
                        2 => args[idx + 1].parse::<u8>(),
                        _ => (args[idx])[2..].parse::<u8>(),
                    };
                    myparser.verbosity = verbosity.expect("Error extracting verbosity level");
                    println!("Verbosity is {}", myparser.verbosity);
                }
                _ => (),
            }
        }

        Some(myparser)
    }
}
