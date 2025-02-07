//read XDS_ASCII.HKL

use crate::Geom::Geom;
use crate::XYZ::XYZ;
// computation of direction cosines

#[derive(Clone)]
pub struct XDSdatum {
    h_: f32,
    k_: f32,
    l_: f32,
    I_: f32,
    sigI_: f32,
    xyzd_: [f32; 3],
    rlp_: f32,
    peak_: f32,
    corr_: f32,
    psi_: f32,
    cosines_: [f32; 6],
    deviation_: f32,
}
pub fn readdata(filename: String, verbosity: u8) -> Option<Vec<XDSdatum>> {
    let inp = std::fs::read_to_string(filename);
    let mut xdsdatum = XDSdatum {
        h_: 0.0,
        k_: 0.0,
        l_: 0.0,
        I_: 0.0,
        sigI_: 0.0,
        xyzd_: [0.0; 3],
        rlp_: 0.0,
        peak_: 0.0,
        corr_: 0.0,
        psi_: 0.0,
        cosines_: [0.0; 6],
        deviation_: 0.0,
    };
    let mut xdsdata: Vec<XDSdatum> = Vec::new();
    let xdslines = match inp {
        Ok(inp) => inp.lines(),
        Err(_) => {
            println!("Error reading XDS_ASCII.HKL {}", filename);
            return None;
        }
    };
    // run over the header to find the first line
    let mut dline = String::new();
    while dline != "!END_OF_HEADER" {
        let dline = xdslines.next();
        dline = match dline {
            Some(dline) => Some(dline),
            None => {
                println!("Error parsing header of XDS_ASCII.HKL");
                return None;
            }
        };
        continue;
    }
    // now dline should be !END_OF_HEADER
    while dline != "!END_OF_DATA" {
        let dline = xdslines.next();
//        match dline {
 //           Some(dline) => dline,
  //          None => {
   //             println!("Error parsing header of XDS_ASCII.HKL");
    //            return None;
     //       }
      //  };
        xdsdatum.from_String(dline, verbosity);
        xdsdata.push(xdsdatum);
        if verbosity > 2 {
            println!("Total data lines so far: {}", xdsdata.len());
        }
        continue;
    }
}

impl XDSdatum {
    // extract entries from string
    pub fn from_String(mut self, dataline: String, verbosity: u8) {
        // H K L I sigI x y z rlp pk corr psi-angle
        let l: Vec<&str> = dataline.split_whitespace().collect();
        let x = l[0].trim().parse();
        match x {
            Ok(x) => {
                self.h_ = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[1].trim().parse();
        match x {
            Ok(x) => {
                self.k_ = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[2].trim().parse();
        match x {
            Ok(x) => {
                self.l_ = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[3].trim().parse();
        match x {
            Ok(x) => {
                self.I_ = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[4].trim().parse();
        match x {
            Ok(x) => {
                self.sigI_ = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[5].trim().parse();
        match x {
            Ok(x) => {
                self.xyzd_[0] = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[6].trim().parse();
        match x {
            Ok(x) => {
                self.xyzd_[1] = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[7].trim().parse();
        match x {
            Ok(x) => {
                self.xyzd_[2] = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[8].trim().parse();
        match x {
            Ok(x) => {
                self.rlp_ = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[9].trim().parse();
        match x {
            Ok(x) => {
                self.peak_ = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[10].trim().parse();
        match x {
            Ok(x) => {
                self.corr_ = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
        let x = l[11].trim().parse();
        match x {
            Ok(x) => {
                self.psi_ = x;
            }
            Err(_) => {
                println!("Cannot parse {dataline}");
            }
        };
    }

    fn cosines(mut self, U: [f32; 9], G: Geom) {
        // coordinates in reciprocal space
        let mut c = XYZ {
            xyz: [
                self.h_ * U[0] + self.k_ * U[1] + self.l_ * U[2],
                self.h_ * U[3] + self.k_ * U[4] + self.l_ * U[5],
                self.h_ * U[6] + self.k_ * U[7] + self.l_ * U[8],
            ],
        };
        // c.xyz[0] = self.h_ * U[0] + self.k_ * U[1] + self.l_ * U[2];
        // c.xyz[1] = self.h_ * U[3] + self.k_ * U[4] + self.l_ * U[5];
        // c.xyz[2] = self.h_ * U[6] + self.k_ * U[7] + self.l_ * U[8];
        let lc = c.uvec();

        let sinetheta = 0.5 * lc * G.lambda_;
        let phi = f32::atan2(
            f32::sqrt(f32::max(0.0, 1.0 - sinetheta * sinetheta)),
            sinetheta,
        );
        let phiRot = c.rad_sin_cos(G.R());
        let phiS0 = c.rad_sin_cos(G.clone().S0());
        let S0Rangle = G.S0().rad_sin_cos(G.R());
        let cs = (phiRot[2] * S0Rangle[2] - phiS0[2]) / (phiRot[1] * S0Rangle[1]);
    }
}
