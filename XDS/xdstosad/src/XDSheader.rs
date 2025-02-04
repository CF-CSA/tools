use crate::XYZ::XYZ;
use std::f32::consts::PI;

struct Detector {
    name_: String,
    detx_: XYZ,
    dety_: XYZ,
    detz_: XYZ,
    nx_: u16,
    ny_: u16,
    qx_: f32,
    qy_: f32,
    orgx_: f32,
    orgy_: f32,
}

pub struct XDSheader {
    nameTemplate_: String,
    oscrange_: f32,
    // data_range
    drange_: [u16; 2],
    // starting angle
    phi0_: f32,
    // starting frame
    frameno0_: u16,
    // dmax, dmin
    dmin_: f32,
    dmax_: f32,
    // space group number, 0 for unknown
    SGnumber_: u8,
    // unit cell constants
    cell_: [f32; 6],
    vecA_: XYZ,
    vecB_: XYZ,
    vecC_: XYZ,
    rotaxis_: XYZ,
    S0_: XYZ,
    detdist_: f32,
    lambda_: f32,
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
    let mut i = 0;
    while i < recv.len() {
        let part = w[i].trim().parse::<f32>();
        recv[i] = match part {
            Ok(part) => part,
            Err(_) => {
                panic!("Cannot read data range from {}", keyw);
            }
        };
        i += 1;
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
    let (vecA, vecB, vecC) = abc2vector(10., 10., 10., 90., 90., 90.);
    let mut xdsheader = XDSheader {
        nameTemplate_: String::new(),
        oscrange_: 0.5,
        drange_: [1, 1000],
        phi0_: 0.0,
        frameno0_: 1,
        dmin_: 0.84,
        dmax_: 999.9,
        SGnumber_: 0,
        cell_: [10., 10., 10., 90., 90., 90.],
        vecA_: vecA,
        vecB_: vecB,
        vecC_: vecC,
        rotaxis_: XYZ {
            xyz: [1.0, 0.0, 0.0],
        },
        S0_: XYZ {
            xyz: [0.0, 0.0, 0.0],
        },
        detdist_: 580.0,
        lambda_: 0.02508,
    };

    for l in xdslines.lines() {
        if l.contains("!END_OF_HEADER") {
            break;
        }
        if l.contains("!DATA_RANGE=") {
            let mut r: [f32; 2] = [0.3; 2];
            getnums(l.to_string(), &mut r);
            xdsheader.drange_ = [r[0] as u16, r[1] as u16];
            continue;
        }
        if l.contains("!ROTATION_AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.rotaxis_ = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!OSCILLATION_RANGE=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.oscrange_ = r[0];
            continue;
        }
        if l.contains("!STARTING_ANGLE=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.phi0_ = r[0];
            continue;
        }
        if l.contains("!STARTING_FRAME=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.frameno0_ = r[0] as u16;
            continue;
        }
        if l.contains("!INCLUDE_RESOLUTION_RANGE=") {
            let mut r: [f32; 2] = [0.3; 2];
            getnums(l.to_string(), &mut r);
            xdsheader.dmin_ = r[0];
            xdsheader.dmax_ = r[1];
            continue;
        }
        if l.contains("!SPACE_GROUP_NUMBER=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.SGnumber_ = r[0] as u8;
            continue;
        }
        if l.contains("!UNIT_CELL_CONSTANTS=") {
            let mut r: [f32; 6] = [0.0; 6];
            getnums(l.to_string(), &mut r);
            xdsheader.cell_ = [r[0], r[1], r[2], r[3], r[4], r[5]];
            continue;
        }
        if l.contains("!UNIT_CELL_A-AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.vecA_ = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!UNIT_CELL_B-AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.vecB_ = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!UNIT_CELL_C-AXIS=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.vecC_ = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
        if l.contains("!X-RAY_WAVELENGTH=") {
            let mut r: [f32; 1] = [0.0; 1];
            getnums(l.to_string(), &mut r);
            xdsheader.lambda_ = r[0];
            continue;
        }
        if l.contains("!INCIDENT_BEAM_DIRECTION=") {
            let mut r: [f32; 3] = [0.0; 3];
            getnums(l.to_string(), &mut r);
            xdsheader.S0_ = XYZ {
                xyz: [r[0], r[1], r[2]],
            };
            continue;
        }
	// NX is in line with NY, QX, QY
	// !NX=  1028  NY=  1062    QX=  0.075000  QY=  0.075000
        if l.contains("!NX=") {
	let w: Vec<&str> = l.split_whitespace().collect();
	let nx = w[1].trim().parse::<u16>();
	let ny = w[3].trim().parse::<u16>();
	let qx = w[5].trim().parse::<f32>();
	let qy = w[5].trim().parse::<f32>();
	}

    }
    return Some(xdsheader);
}
