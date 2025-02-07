pub mod usage {
    pub fn hello() {
        println!("!-----------------------------------------------------------------------!");
        println!(
            "! {:11}{:59}!",
            "xdstosad:", "Convert XDS_ASCII.HKL to xds.sad"
        );
        println!(
            "! {:11}{:59}!",
            ' ', "xds.sad is suitable as input to SADABS"
        );
        println!("! {:11}{:59}!", ' ', "all credits to George M. Sheldrick");
        println!("! {:11}{:59}!", "Copyright:", "Tim Gruene, 2024");
        println!("!-----------------------------------------------------------------------!");
    }

    pub fn usage() {
        println!("!-----------------------------------------------------------------------!");
        println!("! {:11}{:59}!", "Usage:", "xdstosad [Options] [filename]");
        println!(
            "! {:11}{:59}!",
            "", "reads <filename> (default: XDS_ASCII.HKL) and convert"
        );
        println!("! {:11}{:59}!", "", "it to xds.sad for SADABS");
        println!("! {:11}{:59}!", "", "Options:");
        println!(
            "! {:11}{:59}!",
            "", "-r     :  recursevily find all XDS_ASCII.HKL files and"
        );
        println!(
            "! {:11}{:59}!",
            "", "          convert xdsNN.sad, where NN is the counter"
        );
        println!(
            "! {:11}{:59}!",
            "", "-o file:  write output to file [xds.sad]"
        );
        println!("! {:11}{:59}!", "", "-d dir :  write file(s) to dir [./]");
        println!(
            "! {:11}{:59}!",
            "", "-h/ -? :  print this help message and exit"
        );
        println!("!-----------------------------------------------------------------------!");
    }
}
