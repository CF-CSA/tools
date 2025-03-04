use crate::XYZ::XYZ;
use std::f32::consts::PI;

// detector information
#[derive(Clone)]
pub struct Det {
    name: String,
    detx: XYZ,
    dety: XYZ,
    detz: XYZ,
    nx: u16,
    ny: u16,
    qx: f32,
    qy: f32,
    orgx: f32,
    orgy: f32,
}

// geometry description of the diffraction experiment
#[derive(Copy, Clone)]
pub struct Geom {
    rotaxis: XYZ,
    dir_beam: XYZ,
    dist: f32,
    lambda: f32,
    S0R: [f32; 3],
}

impl Det {
    pub fn detz(&self) -> XYZ {
        crate::XYZ::cross(self.detx, self.dety)
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn detx(&self) -> XYZ {
        self.detx
    }
    pub fn dety(&self) -> XYZ {
        self.dety
    }
    pub fn nx(&self) -> u16 {
        self.nx
    }
    pub fn ny(&self) -> u16 {
        self.ny
    }
    pub fn qx(&self) -> f32 {
        self.qx
    }
    pub fn qy(&self) -> f32 {
        self.qy
    }
    pub fn orgx(&self) -> f32 {
        self.orgx
    }
    pub fn orgy(&self) -> f32 {
        self.orgy
    }
}

impl Geom {
    pub fn rotaxis(self) -> XYZ {
        self.rotaxis.clone()
    }
    pub fn dir_beam(self) -> XYZ {
        self.dir_beam.clone()
    }
    pub fn S0R(self) -> [f32; 3] {
        self.S0R.clone()
    }
    pub fn det_dist(self) -> f32 {
        self.dist
    }
    pub fn lambda(self) -> f32 {
        self.lambda
    }
}

pub struct XDSheader {
    name_template: String,
    oscrange: f32,
    // data_range
    drange: [u16; 2],
    // starting angle
    phi0: f32,
    // starting frame
    frameno0: u16,
    // dmax, dmin
    dmin: f32,
    dmax: f32,
    // space group number, 0 for unknown
    sg_number: u8,
    // unit cell constants
    cell: [f32; 6],
    vec_a: XYZ,
    vec_b: XYZ,
    vec_c: XYZ,
    detector: Det,
    geometry: Geom,
}

impl XDSheader {
    pub fn a(&self) -> &XYZ {
        &self.vec_a
    }
    pub fn b(&self) -> &XYZ {
        &self.vec_b
    }
    pub fn c(&self) -> &XYZ {
        &self.vec_c
    }
    pub fn dir_beam(&self) -> &XYZ {
        &self.geometry.dir_beam
    }
    pub fn detz(&self) -> &XYZ {
        &self.detector.detz;
    }
}

fn abc2vector(a: f32, b: f32, c: f32, alpha: f32, beta: f32, gamma: f32) -> (XYZ, XYZ, XYZ) {
    let alpha = PI / 180.0 * alpha;
    let beta = PI / 180.0 * beta;
    let gamma = PI / 180.0 * gamma;
    let avec = XYZ { xyz: [a, 0.0, 0.0] };
    let bvec = XYZ {
        xyz: [b * f32::cos(gamma), b * f32::sin(gamma), 0.0],
    };
    let c0 = c * f32::cos(beta);
    let c1 = b * c * f32::cos(alpha) - bvec.xyz[0] * c0 / bvec.xyz[1];
    let c2 = f32::sqrt(c * c - c0 * c0 - c1 * c1);
    let cvec = XYZ { xyz: [c0, c1, c2] };

    (avec, bvec, cvec)
}

fn getnums<const W: usize>(keyw: String, recv: &mut [f32; W]) {
    let w: Vec<&str> = keyw.split_whitespace().collect();
    for i in 0..recv.len() {
        let part = w[i + 1].trim().parse::<f32>();
        recv[i] = match part {
            Ok(part) => part,
            Err(_) => {
                panic!("Cannot read data range from {}, got {}", keyw, recv[i]);
            }
        };
    }
}

pub fn readheader(filename: &String) -> Option<XDSheader> {
    let inp = std::fs::read_to_string(filename);
    let xdslines = match inp {
        Ok(inp) => inp,
        Err(_) => {
            println!("Error reading XDS_ASCII.HKL {}", filename);
            return None;
        }
    };
    let (vec_a, vec_b, vec_c) = abc2vector(10., 10., 10., 90., 90., 90.);
    let geom = Geom {
        rotaxis: XYZ {
            xyz: [1.0, 0.0, 0.0],
        },
        dir_beam: XYZ {
            xyz: [0.0, 0.0, 1.0],
        },
        dist: 123.4,
        lambda: 0.02508,
        S0R: [0.1, 0.1, 0.1],
    };

    let det = Det {
        name: String::new(),
        detx: XYZ {
            xyz: [1.0, 0.0, 0.0],
        },
        dety: XYZ {
            xyz: [1.0, 0.0, 0.0],
        },
        detz: XYZ {
            xyz: [1.0, 0.0, 0.0],
        },
        nx: 1024,
        ny: 512,
        qx: 0.075,
        qy: 0.075,
        orgx: 512.0,
        orgy: 256.0,
    };
    let mut xdsheader = XDSheader {
        name_template: String::new(),
        oscrange: 0.5,
        drange: [1, 1000],
        phi0: 0.0,
        frameno0: 1,
        dmin: 0.84,
        dmax: 999.9,
        sg_number: 0,
        cell: [10., 10., 10., 90., 90., 90.],
        vec_a,
        vec_b,
        vec_c,
        geometry: geom,
        detector: det,
    };

    for l in xdslines.lines() {
        if l.contains("!END_OF_HEADER") {
            break;
        }
        if l.contains("!DATA_RANGE=") {
            let mut r: [f32; 2] = [0.3; 2];
            getnums(l.to_string(), &mut r);
            xdsheader.drange = [r[0] as u16, r[1] as u16];
            continue;
        }
        if l.contains("!ROTATION_AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.geometry.rotaxis = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!OSCILLATION_RANGE=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.oscrange = r[0];
            continue;
        }
        if l.contains("!STARTING_ANGLE=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.phi0 = r[0];
            continue;
        }
        if l.contains("!STARTING_FRAME=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.frameno0 = r[0] as u16;
            continue;
        }
        if l.contains("!INCLUDE_RESOLUTION_RANGE=") {
            let mut r: [f32; 2] = [0.3; 2];
            getnums(l.to_string(), &mut r);
            xdsheader.dmin = r[0];
            xdsheader.dmax = r[1];
            continue;
        }
        if l.contains("!SPACE_GROUP_NUMBER=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.sg_number = r[0] as u8;
            continue;
        }
        if l.contains("!UNIT_CELL_CONSTANTS=") {
            let mut r: [f32; 6] = [0.0; 6];
            getnums(l.to_string(), &mut r);
            xdsheader.cell = [r[0], r[1], r[2], r[3], r[4], r[5]];
            continue;
        }
        if l.contains("!UNIT_CELL_A-AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.vec_a = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!UNIT_CELL_B-AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.vec_b = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!UNIT_CELL_C-AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.vec_c = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!X-RAY_WAVELENGTH=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.geometry.lambda = r[0];
            continue;
        }
        if l.contains("!INCIDENT_BEAM_DIRECTION=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.geometry.dir_beam = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!DIRECTION_OF_DETECTOR_X-AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.det.detx = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!DIRECTION_OF_DETECTOR_Y-AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.det.dety = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        // NX is in line with NY, QX, QY
        // !NX=  1028  NY=  1062    QX=  0.075000  QY=  0.075000
        if l.contains("!NX=") {
            let w: Vec<&str> = l.split_whitespace().collect();
            let nx = match w[1].trim().parse::<u16>() {
                Ok(num) => num,
                Err(_) => panic!("Format error, NX not found in {l}"),
            };
            xdsheader.detector.nx = nx;
            let ny = match w[3].trim().parse::<u16>() {
                Ok(num) => num,
                Err(_) => panic!("Format error, NY not found in {l}"),
            };
            xdsheader.detector.ny = ny;
            let qx = match w[5].trim().parse::<f32>() {
                Ok(num) => num,
                Err(_) => panic!("Format error, QX not found in {l}"),
            };
            xdsheader.detector.qx = qx;
            let qy = match w[7].trim().parse::<f32>() {
                Ok(num) => num,
                Err(_) => panic!("Format error, QY not found in {l}"),
            };
            xdsheader.detector.qy = qy;
        }
    }
    // detector z-direction defined to complete a right-handed coordinate system
    xdsheader.detector.detz = XYZ::cross(xdsheader.detector.detx.uvec(), xdsheader.detector.dety.uvec());

    return Some(xdsheader);
}
