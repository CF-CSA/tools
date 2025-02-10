use crate::usage::usage::usage;
use std::env;

#[derive(Clone)]
pub struct Parser {
    xdsascii_file_: String,
    outdir_: String,
    outfiles_: Vec<String>,
    recursive_: bool,
    verbosity_: u8,
}

impl Parser {
    pub fn xdsascii(&self) -> String {
        return self.xdsascii_file_.clone();
    }
    pub fn verbosity(&self) -> u8 {
        return self.verbosity_;
    }
    pub fn parseinput() -> Option<Parser> {
        let args: Vec<String> = env::args().collect();
        let mut myparser = Parser {
            xdsascii_file_: String::from("XDS_ASCII.HKL"),
            outdir_: String::from("./"),
            outfiles_: Vec::new(),
            recursive_: true,
            verbosity_: 1,
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
                    myparser.verbosity_ = verbosity.expect("Error extracting verbosity level");
                    println!("Verbosity is {}", myparser.verbosity_);
                }
                _ => (),
            }
        }

        Some(myparser)
    }
}
