use crate::Det::Det;
use crate::XYZ::XYZ;
use std::f32::consts::PI;

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
    rotaxis: XYZ,
    s0: XYZ,
    detector: Det,
    detdist: f32,
    lambda: f32,
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
    let mut det = Det {
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
        rotaxis: XYZ {
            xyz: [1.0, 0.0, 0.0],
        },
        s0: XYZ {
            xyz: [0.0, 0.0, 0.0],
        },
        detector: det,
        detdist: 580.0,
        lambda: 0.02508,
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
            xdsheader.rotaxis = XYZ {
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
            xdsheader.lambda = r[0];
            continue;
        }
        if l.contains("!INCIDENT_BEAM_DIRECTION=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.s0 = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        // NX is in line with NY, QX, QY
        // !NX=  1028  NY=  1062    QX=  0.075000  QY=  0.075000
        if l.contains("!NX=") {
            let w: Vec<&str> = l.split_whitespace().collect();
            let nx = w[1].trim().parse::<u16>();
	    det.nx = Some(nx);
            let ny = w[3].trim().parse::<u16>();
            let qx = w[5].trim().parse::<f32>();
            let qy = w[5].trim().parse::<f32>();
        }
    }
    return Some(xdsheader);
}
