//read XDS_ASCII.HKL

use crate::Det::Det;
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
    let mut xdsdata: Vec<XDSdatum> = Vec::new();
    let mut xdslines: Vec<String> = std::fs::read_to_string(filename)
        .expect("Failed to read XDS_ASCII.HKL")
        .split("\n")
        .map(|line| line.to_string())
        .collect();
    xdslines.retain(|x| x.chars().nth(0) != Some('!'));

    // only data lines should remain
    for it in xdslines {
        if it.len() == 0 {
            break;
        }
        let xdsdatum = from_dataline(it, verbosity);
        xdsdata.push(xdsdatum);
        if verbosity > 2 {
            println!("Total data lines so far: {}", xdsdata.len());
        }
        continue;
    }
    return Some(xdsdata);
}

// extract entries from string
fn from_dataline(dataline: String, verbosity: u8) -> XDSdatum {
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
    // H K L I sigI x y z rlp pk corr psi-angle
    let l: Vec<&str> = dataline.split_whitespace().collect();
    let x = l[0].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.h_ = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[1].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.k_ = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[2].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.l_ = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[3].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.I_ = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[4].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.sigI_ = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[5].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.xyzd_[0] = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[6].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.xyzd_[1] = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[7].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.xyzd_[2] = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[8].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.rlp_ = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[9].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.peak_ = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[10].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.corr_ = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[11].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.psi_ = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    if verbosity > 2 {
        println!("Data extracted from {dataline}");
    }
    xdsdatum
}

impl XDSdatum {
    fn cosines(mut self, matrix_u: [f32; 9], geom: Geom, det: Det) {
        // coordinates in reciprocal space
        let mut c = XYZ {
            xyz: [
                self.h_ * matrix_u[0] + self.k_ * matrix_u[1] + self.l_ * matrix_u[2],
                self.h_ * matrix_u[3] + self.k_ * matrix_u[4] + self.l_ * matrix_u[5],
                self.h_ * matrix_u[6] + self.k_ * matrix_u[7] + self.l_ * matrix_u[8],
            ],
        };
        let lc = c.uvec();

        let sinetheta = 0.5 * lc * geom.lambda_;
        // angle of reciprocal beam (from sine theta)
        let phi = f32::atan2(
            f32::sqrt(f32::max(0.0, 1.0 - sinetheta * sinetheta)),
            sinetheta,
        );
        // angle of vector w.r.t. rotation axis
        let phi_rot = c.rad_sin_cos(geom.R());
        // angle of vector w.r.t. direct beam
        let phi_s0 = c.rad_sin_cos(geom.clone().S0());
        let s0r_angle = geom.S0().rad_sin_cos(geom.R());
        let cs = (phi_rot[2] * s0r_angle[2] - phi_s0[2]) / (phi_rot[1] * s0r_angle[1]);
        let s = f32::atan2(f32::sqrt(f32::max(0.0, 1.0 - cs * cs)), cs);
        let r = (sinetheta - phi_rot[2] * geom.S0R()[2]) / (phi_rot[1] * geom.S0R()[1]);
        let t = f32::atan2(f32::sqrt(f32::max(0.0, 1. - r * r)), r);

        // predicted x,y coordinates of this reflection
        let x = det.qx() * (self.xyzd_[0] - det.orgx());
        let y = det.qy() * (self.xyzd_[1] - det.orgy());
        let e3: XYZ = det.detx() * x + det.dety() * y + det.detz() * geom.D();
        let mut lim = f32::INFINITY;

        let mut phi_rot = phi_rot.clone();
        let mut e2 = XYZ { xyz: [0.0; 3] };
        for idx in 0..5 {
            let v = match idx {
                // this construct is unclear to me and copied from the fortran source
                0 => -t - s,
                1 => t - s,
                2 => s - t,
                4 => phi_rot[0],
                _other => t + s,
            };
            let crot: XYZ = crate::XYZ::rotate(c, geom.R(), v);
            let mut e1: XYZ = crate::XYZ::cross(crot, geom.S0());
            e1.uvec();
            e2 = crate::XYZ::rotate(crot, e1, phi);
            let xyz = e2.rad_sin_cos(e3);
            if xyz[0] < lim {
                lim = xyz[0];
                phi_rot[0] = v;
            }
            continue;
        }
        for i in 0..3 {
            let mut e1 = XYZ {
                xyz: [matrix_u[i + 0], matrix_u[i + 3], matrix_u[i + 6]],
            };
            e1.uvec();
            let e3 = crate::XYZ::rotate(e1, geom.R(), phi_rot[0]);
            let xyz = e3.rad_sin_cos(geom.S0());
            let j = 2 * i;
            self.cosines_[j] = xyz[2] * (-1.0);
            let xyz = e3.rad_sin_cos(e2);
            self.cosines_[j + 1] = xyz[2];
        }
    }
}
