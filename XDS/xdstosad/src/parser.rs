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
            match args[idx].as_str() {
                "-h" | "-?" => {
                    usage();
                    return None;
                }
                "-v" => {
                    let verbosity = args[idx + 1].parse::<u8>();
                    myparser.verbosity_ = verbosity.expect("Error extracting verbosity level");
                }
                _ => (),
            }
        }

        Some(myparser)
    }
}
