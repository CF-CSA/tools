//read XDS_ASCII.HKL

use crate::XDSheader::XDSheader;
use crate::XYZ::XYZ;

// computation of direction cosines

#[derive(Clone)]
pub struct XDSdatum {
    h: f32,
    k: f32,
    l: f32,
    I: f32,
    sigI: f32,
    xyzd: [f32; 3],
    rlp: f32,
    peak: f32,
    corr: f32,
    psi: f32,
}

impl XDSdatum {
    fn new() -> XDSdatum {
        XDSdatum {
            h: 0.0,
            k: 0.0,
            l: 0.0,
            I: 0.0,
            sigI: -1.0,
            xyzd: [0.0, 0.0, 0.0],
            rlp: 0.0,
            peak: 0.0,
            corr: 0.0,
            psi: 0.0,
        }
    }
    pub fn h(&self) -> &f32 {
        &self.h
    }
    pub fn k(&self) -> &f32 {
        &self.k
    }
    pub fn l(&self) -> &f32 {
        &self.l
    }
    pub fn I(&self) -> &f32 {
        &self.I
    }
    pub fn sigI(&self) -> &f32 {
        &self.sigI
    }
    pub fn xd(&self) -> &f32 {
        &self.xyzd[0]
    }
    pub fn yd(&self) -> &f32 {
        &self.xyzd[1]
    }
    pub fn zd(&self) -> &f32 {
        &self.xyzd[2]
    }
}

// read data items from XDS_ASCII.HKL
pub fn readdata(filename: String, dscale: &mut f32, verbosity: u8) -> Option<Vec<XDSdatum>> {
    *dscale = -1.0 * f32::INFINITY;
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
        let xdsdatum = from_dataline(it, &mut *dscale, verbosity);
        xdsdata.push(xdsdatum);
        if verbosity > 1 {
            println!("Total data lines so far: {}", xdsdata.len());
        }
        continue;
    }
    if verbosity > 1 {
        println!("Data scale factor = {}", *dscale);
    }

    return Some(xdsdata);
}

// extract entries from string
fn from_dataline(dataline: String, dscale: &mut f32, verbosity: u8) -> XDSdatum {
    let mut xdsdatum = XDSdatum::new();
    // H K L I sigI x y z rlp pk corr psi-angle
    let l: Vec<&str> = dataline.split_whitespace().collect();
    let x = l[0].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.h = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[1].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.k = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[2].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.l = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[3].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.I = x;
            *dscale = f32::max(*dscale, x);
            *dscale = f32::max(*dscale, -10.0 * x);
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[4].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.sigI = x;
            *dscale = f32::max(*dscale, x);
            *dscale = f32::max(*dscale, -10.0 * x);
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[5].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.xyzd[0] = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[6].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.xyzd[1] = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[7].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.xyzd[2] = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[8].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.rlp = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[9].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.peak = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[10].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.corr = x;
        }
        Err(_) => {
            println!("Cannot parse {dataline}");
        }
    };
    let x = l[11].trim().parse();
    match x {
        Ok(x) => {
            xdsdatum.psi = x;
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
    // computes direction cosines as well as sinetheta / lambda
    // direction cosines stored in 0-6, sthl in 7 of return array
    pub fn cosines(&self, matrix_u: [f32; 9], header: &XDSheader) -> [f32; 7] {
        println!("Matrix Uab:");
	println!("   {:4.3} {:4.3} {:4.3}", matrix_u[0], matrix_u[1], matrix_u[2]);
	println!("   {:4.3} {:4.3} {:4.3}", matrix_u[3], matrix_u[4], matrix_u[5]);
	println!("   {:4.3} {:4.3} {:4.3}", matrix_u[6], matrix_u[7], matrix_u[8]);
        // coordinates in reciprocal space
        let mut c = XYZ {
            xyz: [
                self.h * matrix_u[0] + self.k * matrix_u[1] + self.l * matrix_u[2],
                self.h * matrix_u[3] + self.k * matrix_u[4] + self.l * matrix_u[5],
                self.h * matrix_u[6] + self.k * matrix_u[7] + self.l * matrix_u[8],
            ],
        };
        let lc = c.uvec();
        let sthl = 0.5 * lc;
        let sinetheta = sthl * header.lambda();

        // angle of reciprocal beam (from sine theta)
        let phi = f32::atan2(
            f32::sqrt(f32::max(0.0, 1.0 - sinetheta * sinetheta)),
            sinetheta,
        );
        // angle of vector w.r.t. rotation axis
        let phi_rot = c.rad_sin_cos(header.rotaxis());
        // angle of vector w.r.t. direct beam
        let phi_s0 = c.rad_sin_cos(header.dir_beam());
        let s0r_angle = header.dir_beam().rad_sin_cos(header.rotaxis());
        let cs = (phi_rot[2] * s0r_angle[2] - phi_s0[2]) / (phi_rot[1] * s0r_angle[1]);
        let s = f32::atan2(f32::sqrt(f32::max(0.0, 1.0 - cs * cs)), cs);
        let r = (sinetheta - phi_rot[2] * header.S0R()[2]) / (phi_rot[1] * header.S0R()[1]);
        let t = f32::atan2(f32::sqrt(f32::max(0.0, 1. - r * r)), r);

        // predicted x,y coordinates of this reflection
        let x = header.qx() * (self.xyzd[0] - header.orgx());
        let y = header.qy() * (self.xyzd[1] - header.orgy());
        let e3: XYZ = *header.detx() * x + *header.dety() * y + *header.detz() * *header.distance();
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
            let crot: XYZ = crate::XYZ::rotate(c, *header.rotaxis(), v);
            let mut e1: XYZ = crate::XYZ::cross(crot, *header.dir_beam());
            e1.uvec();
            e2 = crate::XYZ::rotate(crot, e1, phi);
            let xyz = e2.rad_sin_cos(&e3);
            if xyz[0] < lim {
                lim = xyz[0];
                phi_rot[0] = v;
            }
            continue;
        }
        let mut cosines: [f32; 7] = [0.0; 7];
        cosines[6] = sthl;
        for i in 0..3 {
            let mut e1 = XYZ {
                xyz: [matrix_u[i + 0], matrix_u[i + 3], matrix_u[i + 6]],
            };
            e1.uvec();
            let e3 = crate::XYZ::rotate(e1, *header.rotaxis(), phi_rot[0]);
            let xyz = e3.rad_sin_cos(header.dir_beam());
            let j = 2 * i;
            cosines[j] = xyz[2] * (-1.0);
            let xyz = e3.rad_sin_cos(&e2);
            cosines[j + 1] = xyz[2];
        }
        cosines
    }
}
