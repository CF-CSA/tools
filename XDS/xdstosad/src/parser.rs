use std::env;
pub use crate::usage::usage::usage;

#[derive(Clone)]
pub struct Parser {
	xdsascii_files_: Vec<String>,
	outdir_: String,
	outfiles_: Vec<String>,
	recursive_: bool,
	verbosity_: u8,
}

impl Parser {
pub fn parseinput() -> Option<Parser> {
   let args: Vec<String> = env::args().collect();
   let mut myparser = Parser {
    xdsascii_files_: Vec::new(),
    outdir_: String::from("./"),
    outfiles_: Vec::new(),
    recursive_: true,
    verbosity_: 1,
   };

   for idx in 1..args.len() {
   match args[idx].as_str() {
   	"-h" | "-?" => {
		usage();
		return None
		},
	"-v" => {
		let verbosity = args[idx+1].parse::<u8>();
		myparser.verbosity_ = 
			verbosity.expect("Error extracting verbosity level");
	       },
	_ => (),
   }
   }


   Some(myparser)

}
}
