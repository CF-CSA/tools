use std::f32::consts::PI;
use std::process;
use std::ops::{Add, Div, Mul, Sub};

use chrono::{TimeZone, Utc};
use std::env;

const PCFFILE: &str = "weightedcell.pcf";

// cell parameters including esu
#[derive(Clone)]
struct Cell {
    file: String,
    sg: i32,
    a: f32,
    b: f32,
    c: f32,
    alpha: f32,
    beta: f32,
    gamma: f32,
    a_esu: f32,
    b_esu: f32,
    c_esu: f32,
    alpha_esu: f32,
    beta_esu: f32,
    gamma_esu: f32,
}

// collection of experimental information for pcf-file
#[derive(Clone)]
struct Pcf {
    file: String,
    num_refl: i32,                           // from CORRECT.LP
    detector: String,                        // from CORRECT.LP
    nx: i32,                                 // from CORRECT.LP
    ny: i32,                                 // from CORRECT.LP
    qx: f32,                                 // from CORRECT.LP
    qy: f32,                                 // from CORRECT.LP
    distance: f32,                           // from CORRECT.LP
    wavelength: f32,                         // from CORRECT.LP
    cellabc: (f32, f32, f32, f32, f32, f32), // from CORRECT.LP (or Cell)
    cellesd: (f32, f32, f32, f32, f32, f32), // from CORRECT.LP ( or Cell)
}

// 3D vectors
#[derive(Clone)]
struct XYZ {
    xyz: [f32; 3],
}

fn cross(x1: &XYZ, x2: &XYZ) -> XYZ {
    let x = &x1.xyz[1] * &x2.xyz[2] - &x1.xyz[2] * &x2.xyz[1];
    let y = &x1.xyz[2] * &x2.xyz[0] - &x1.xyz[0] * &x2.xyz[2];
    let z = &x1.xyz[0] * &x2.xyz[1] - &x1.xyz[1] * &x2.xyz[0];

    let xyz = XYZ { xyz: [x, y, z] };
    xyz
}

impl Mul for XYZ {
    type Output = f32;
    fn mul(self, other: XYZ) -> f32 {
        self.xyz[0] * other.xyz[0] + self.xyz[1] * other.xyz[1] + self.xyz[2] * other.xyz[2]
    }
}

impl Mul<f32> for XYZ {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Self {
            xyz: [s * self.xyz[0], s * self.xyz[1], s * self.xyz[2]],
        }
    }
}

impl Mul<i32> for XYZ {
    type Output = Self;
    fn mul(self, si: i32) -> Self {
        let s = si as f32;
        Self {
            xyz: [s * self.xyz[0], s * self.xyz[1], s * self.xyz[2]],
        }
    }
}

impl Div<f32> for XYZ {
    type Output = Self;
    fn div(self, d: f32) -> Self {
        let s = 1.0 / d;
        Self {
            xyz: [s * self.xyz[0], s * self.xyz[1], s * self.xyz[2]],
        }
    }
}

impl Add for XYZ {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            xyz: [
                self.xyz[0] + other.xyz[0],
                self.xyz[1] + other.xyz[1],
                self.xyz[2] + other.xyz[2],
            ],
        }
    }
}

impl Sub for XYZ {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            xyz: [
                self.xyz[0] - other.xyz[0],
                self.xyz[1] - other.xyz[1],
                self.xyz[2] - other.xyz[2],
            ],
        }
    }
}

fn volume(a: &XYZ, b: &XYZ, c: &XYZ) -> f32 {
    let cstar = cross(a, b);
    cstar * c.clone()
}

fn rec_cell(a: XYZ, b: XYZ, c: XYZ) -> (XYZ, XYZ, XYZ) {
    let vol = volume(&a, &b, &c);
    let astar = cross(&b, &c) / vol;
    let bstar = cross(&c, &a) / vol;
    let cstar = cross(&a, &b) / vol;
    (astar, bstar, cstar)
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

// workflow:
// - read CORRECT.LP and accumulate into cells_*
// - compute weighted cell and esds
// - compute reciprocal cell
// - read XDS_ASCII.HKL to compute dmin and dmax from
//   weighted cell
// - output XSCALE.INP
// - write my.pcf
//   TODO: provide argument '-r base' for base.HKL and base.pcf
fn main() {
    welcome();
    let args: Vec<String> = env::args().collect();
    // let has_esds = false;
    let mut all_cells: Vec<Cell> = Vec::new();
    let mut all_pcfs: Vec<Pcf> = Vec::new();
    let mut cells_w_esu: Vec<Cell> = Vec::new();
    let mut cells_wo_esu: Vec<Cell> = Vec::new();
    let mut pcf_switch: bool = false;

    for idx in 1..args.len() {
	let mut filename: String;
    	match args[idx].as_str() {
		"-h" => {
			usage();
			process::exit(1);
			},
		"-w" => {
			pcf_switch = true;
			continue;
			},
		_ => filename = args[idx].clone(),
	}
        if std::path::Path::new(&filename).is_dir() == true {
            filename += "/CORRECT.LP";
        }
        let (cell, pcf) = match rd_correct(filename) {
		Some((cell, pcf)) => (cell, pcf),
		None  => continue,
	};
        all_cells.push(cell.clone());
        all_pcfs.push(pcf.clone());
        if cell.sg == -1 || cell.a_esu == -1.0 {
            cells_wo_esu.push(cell);
        } else {
            cells_w_esu.push(cell);
        }
    }
    if all_cells.len() == 0 {
    	usage();
    	println!("\n---> Empty list of CORRECT.LP files <---");
	std::process::exit(1);
	}

    let sg = cells_w_esu[0].sg;
    let mcell: Cell;
    // no esu's available, take standard average
    if cells_w_esu.len() < 1 {
        let sigmas = vec![1.0; cells_wo_esu.len()];

        let vals: Vec<_> = cells_wo_esu.iter().map(|p| p.a).collect();
        let (amean, aesu) = wmean(&vals, &sigmas);

        let vals: Vec<_> = cells_wo_esu.iter().map(|p| p.b).collect();
        let (bmean, besu) = wmean(&vals, &sigmas);

        let vals: Vec<_> = cells_wo_esu.iter().map(|p| p.c).collect();
        let (cmean, cesu) = wmean(&vals, &sigmas);

        let vals: Vec<_> = cells_wo_esu.iter().map(|p| p.alpha).collect();
        let (alphamean, alphaesu) = wmean(&vals, &sigmas);

        let vals: Vec<_> = cells_wo_esu.iter().map(|p| p.beta).collect();
        let (betamean, betaesu) = wmean(&vals, &sigmas);

        let vals: Vec<_> = cells_wo_esu.iter().map(|p| p.gamma).collect();
        let (gammamean, gammaesu) = wmean(&vals, &sigmas);
        mcell = Cell {
            file: String::new(),
            sg: sg,
            a: amean,
            b: bmean,
            c: cmean,
            alpha: alphamean,
            beta: betamean,
            gamma: gammamean,
            a_esu: aesu,
            b_esu: besu,
            c_esu: cesu,
            alpha_esu: alphaesu,
            beta_esu: betaesu,
            gamma_esu: gammaesu,
        };
    } else {
        let vals: Vec<_> = cells_w_esu.iter().map(|p| p.a).collect();
        let esus: Vec<_> = cells_w_esu.iter().map(|p| p.a_esu).collect();
        let (amean, aesu) = wmean(&vals, &esus);

        let vals: Vec<_> = cells_w_esu.iter().map(|p| p.b).collect();
        let esus: Vec<_> = cells_w_esu.iter().map(|p| p.b_esu).collect();
        let (bmean, besu) = wmean(&vals, &esus);

        let vals: Vec<_> = cells_w_esu.iter().map(|p| p.c).collect();
        let esus: Vec<_> = cells_w_esu.iter().map(|p| p.c_esu).collect();
        let (cmean, cesu) = wmean(&vals, &esus);

        let vals: Vec<_> = cells_w_esu.iter().map(|p| p.alpha).collect();
        let esus: Vec<_> = cells_w_esu.iter().map(|p| p.alpha_esu).collect();
        let (alphamean, alphaesu) = wmean(&vals, &esus);

        let vals: Vec<_> = cells_w_esu.iter().map(|p| p.beta).collect();
        let esus: Vec<_> = cells_w_esu.iter().map(|p| p.beta_esu).collect();
        let (betamean, betaesu) = wmean(&vals, &esus);

        let vals: Vec<_> = cells_w_esu.iter().map(|p| p.gamma).collect();
        let esus: Vec<_> = cells_w_esu.iter().map(|p| p.gamma_esu).collect();
        let (gammamean, gammaesu) = wmean(&vals, &esus);

        mcell = Cell {
            file: String::new(),
            sg: sg,
            a: amean,
            b: bmean,
            c: cmean,
            alpha: alphamean,
            beta: betamean,
            gamma: gammamean,
            a_esu: aesu,
            b_esu: besu,
            c_esu: cesu,
            alpha_esu: alphaesu,
            beta_esu: betaesu,
            gamma_esu: gammaesu,
        };
    }

    xscaleinp(all_cells, mcell.clone());
    if pcf_switch == true {
    	write_pcf(all_pcfs, &mcell);
    }
}

// compute weighted mean
// if one sigma == 0, assume this is constraint, return
// first value and 0
fn wmean(vals: &Vec<f32>, sigmas: &Vec<f32>) -> (f32, f32) {
    if sigmas[0] == 0.0 {
        return (vals[0], 0.0);
    }
    let mut mean: f32 = 0.0;

    let mut sumsigma2: f32 = 0.0;

    for it in vals.iter().zip(sigmas.iter()) {
        let (x, s) = it;
        mean += *x / (*s * *s);
        sumsigma2 += 1.0 / (*s * *s);
    }

    mean /= sumsigma2;

    let mut sums: f32 = 0.0;
    let mut sumw: f32 = 0.0;
    for it in vals.iter().zip(sigmas.iter()) {
        let (x, s) = it;
	let w = 1.0 - (mean - *x) * (mean - *x) / (mean*mean);
        sums += w * *s;
        sumw += w;
    }
    let sigma: f32 = sums/ sumw;

    (mean, sigma)
}

fn usage() {
	println!("Usage: weightedcell <one or more CORRECT.LP> [-w]\n");
	println!("      -w: Create file weightedcell.pcf with CIF keywords");
	println!("          including some experimental data\n");
	println!("       e.g. #> weightedcell ../run | tee XSCALE.INP");
	println!("       or   #> weightedcell ../run/CORRECT.LP | tee XSCALE.INP");
}

fn welcome() {
    let now = match env::var("SOURCE_DATE_EPOCH") {
    Ok(val) => { Utc.timestamp_opt(val.parse::<i64>().unwrap(), 0).unwrap() }
        Err(_) => Utc::now(),
        };
    let now = now.to_string();
    println!("! ----------------> XSCALE.INP from weightedcell <--------------!");
    println!("!  Weighted cell parameters from XDS CORRECT.LP                 !");
    println!("!  Version 01/2025, (c) Tim Gruene                              !");
    println!("!  tim.gruene@univie.ac.at                                      !");
    println!("!  Experimental CIF entries written to {:10}         !", PCFFILE);
    println!("!  Built {:-30}                      !", now);
    println!("! --------------------------------------------------------------!");
}

// read CORRECT.LP; path provided by filename
// assume to be valid path
// return true if esds are available
// if
fn rd_correct(filename: String) -> Option<(Cell, Pcf) > {
    let mut mycell = Cell {
        file: filename.clone(),
        sg: -1,
        a: 10.0,
        b: 10.0,
        c: 10.0,
        alpha: 90.0,
        beta: 90.0,
        gamma: 90.0,
        a_esu: -1.0,
        b_esu: -1.0,
        c_esu: -1.0,
        alpha_esu: -1.0,
        beta_esu: -1.0,
        gamma_esu: -1.0,
    };
    let mut mypcf = Pcf {
        file: filename.clone(),
        num_refl: 0,
        detector: String::new(),
        nx: 0,
        ny: 0,
        qx: 0.0,
        qy: 0.0,
        distance: 0.0,
        wavelength: 0.0,
        cellabc: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        cellesd: (0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    };

    let correctlp_result = std::fs::read_to_string(filename);
    let correctlp = match correctlp_result {
    	Ok(lines) => lines,
	Err(_) => String::new(),
	};
    if correctlp.len() == 0 {
    	return None;
	}

    // error handling is done, can use lines() directly
    for l in correctlp.lines() {
        ///////////////////////////////////////////////
        // PCF Details                               //
        ///////////////////////////////////////////////
        if l.contains(" X-RAY_WAVELENGTH=") {
            let w: Vec<&str> = l.split_whitespace().collect();
            let wavelength = w[1].trim().parse::<f32>();
            mypcf.wavelength =
                wavelength.expect("Error: unable to read {wavelength} as wavelength");
            continue;
        }
        if l.contains(" DETECTOR=") {
            let w: Vec<&str> = l.split('=').collect();
            let detector = w[1];
            mypcf.detector = detector.to_string();
            continue;
        }
        if l.contains(" NX=") {
            let w: Vec<&str> = l.split_whitespace().collect();
            let x = w[1].trim().parse::<i32>();
            mypcf.nx = x.expect("Error: unable to read {x} as NX.");
            let x = w[3].trim().parse::<i32>();
            mypcf.ny = x.expect("Error: unable to read {x} as NY.");
            let x = w[5].trim().parse::<f32>();
            mypcf.qx = x.expect("Error: unable to read {x} as QX.");
            let x = w[7].trim().parse::<f32>();
            mypcf.qy = x.expect("Error: unable to read {x} as QY.");
            continue;
        }
        if l.contains(" DETECTOR_DISTANCE=") {
            let w: Vec<&str> = l.split_whitespace().collect();
            let dist = w[1].trim().parse::<f32>();
            mypcf.distance = dist.expect("Error: unable to read {D} as detector distance.");
            continue;
        }
        if l.contains(" INDEXED SPOTS") {
            let w: Vec<&str> = l.split_whitespace().collect();
            let x = w[7].trim().parse::<i32>();
            mypcf.num_refl = x.expect("Error: unable to extract indexed spots from {x}.");
            continue;
        }

        ///////////////////////////////////////////////
        // XSCALE.INP Details                        //
        ///////////////////////////////////////////////
        if l.contains(" SPACE GROUP NUMBER ") {
            let w: Vec<&str> = l.split_whitespace().collect();
            let sg = w[3].trim().parse::<i32>();
            mycell.sg = sg.expect("Error: unable to convert {sg} to SG number");
            continue;
            // next line should be cell and ESDs
        }
        if l.contains(" UNIT CELL PARAMETERS ") {
            let w: Vec<&str> = l.split_whitespace().collect();
            let a = w[3].trim().parse::<f32>();
            let b = w[4].trim().parse::<f32>();
            let c = w[5].trim().parse::<f32>();
            let alpha = w[6].trim().parse::<f32>();
            let beta = w[7].trim().parse::<f32>();
            let gamma = w[8].trim().parse::<f32>();
            mycell.a = a.expect("Error Cell: unable to convert {a} to float");
            mycell.b = b.expect("Error Cell: unable to convert {a} to float");
            mycell.c = c.expect("Error Cell: unable to convert {a} to float");
            mycell.alpha = alpha.expect("Error Cell: unable to convert {a} to float");
            mycell.beta = beta.expect("Error Cell: unable to convert {a} to float");
            mycell.gamma = gamma.expect("Error Cell: unable to convert {a} to float");
            continue;
        }
        if l.contains(" E.S.D. OF CELL PARAMETERS") {
            let w: Vec<&str> = l.split_whitespace().collect();
            if w.len() == 5 {
                mycell.a_esu = -1.0;
                mycell.b_esu = -1.0;
                mycell.c_esu = -1.0;
                mycell.alpha_esu = -1.0;
                mycell.beta_esu = -1.0;
                mycell.gamma_esu = -1.0;
            } else {
                let a = w[4].trim().parse::<f32>();
                let b = w[5].trim().parse::<f32>();
                let c = w[6].trim().parse::<f32>();
                let alpha = w[7].trim().parse::<f32>();
                let beta = w[8].trim().parse::<f32>();
                let gamma = w[9].trim().parse::<f32>();
                mycell.a_esu = a.expect("Error ESU: unable to convert {a} to float");
                mycell.b_esu = b.expect("Error ESU: unable to convert {a} to float");
                mycell.c_esu = c.expect("Error ESU: unable to convert {a} to float");
                mycell.alpha_esu = alpha.expect("Error ESU: unable to convert {a} to float");
                mycell.beta_esu = beta.expect("Error ESU: unable to convert {a} to float");
                mycell.gamma_esu = gamma.expect("Error ESU: unable to convert {a} to float");
            }
            continue;
        }
    }
    // update mypcf with cell information
    mypcf.cellabc = (
        mycell.a,
        mycell.b,
        mycell.c,
        mycell.alpha,
        mycell.beta,
        mycell.gamma,
    );
    mypcf.cellesd = (
        mycell.a_esu,
        mycell.b_esu,
        mycell.c_esu,
        mycell.alpha_esu,
        mycell.beta_esu,
        mycell.gamma_esu,
    );
    Some((mycell, mypcf))
}

fn printcell(cell: &Cell) {
    println!("!---> {}", cell.file);
    println!(
        "!     cell {:7.3}{:7.3}{:7.3}{:7.3}{:7.3}{:7.3}",
        cell.a, cell.b, cell.c, cell.alpha, cell.beta, cell.gamma
    );
    println!(
        "!     esu  {:7.3}{:7.3}{:7.3}{:7.3}{:7.3}{:7.3}",
        cell.a_esu, cell.b_esu, cell.c_esu, cell.alpha_esu, cell.beta_esu, cell.gamma_esu
    );
}

// replace CORRECT.LP with XDS_ASCII.HKL and print
fn printinp(cell: &Cell) {
    let s = cell.file.replace("CORRECT.LP", "XDS_ASCII.HKL");
    println!(" INPUT_FILE= {s}");
}

// summarise information and print as valid XSCALE.INP
// to stdout
fn xscaleinp(cells: Vec<Cell>, mcell: Cell) {
    for c in &cells {
        printcell(c);
    }
    println!("!=========================================================================");
    println!(
        "! Mean cell: {:8.4} {:8.4} {:8.4} {:8.3} {:8.3} {:8.3}",
        mcell.a, mcell.b, mcell.c, mcell.alpha, mcell.beta, mcell.gamma
    );
    println!(
        "!   e.s.u's: {:8.4} {:8.4} {:8.4} {:8.3} {:8.3} {:8.3}",
        mcell.a_esu, mcell.b_esu, mcell.c_esu, mcell.alpha_esu, mcell.beta_esu, mcell.gamma_esu
    );

    println!("\n OUTPUT_FILE= my.HKL");
    println!("\n SPACE_GROUP_NUMBER= {}", mcell.sg);
    println!(
        " UNIT_CELL_CONSTANTS= {:8.4} {:8.4} {:8.4} {:8.3} {:8.3} {:8.3}",
        mcell.a, mcell.b, mcell.c, mcell.alpha, mcell.beta, mcell.gamma
    );
    for c in &cells {
        printinp(c);
    }
}

// extract dstarmin and dstarmax from XDS_ASCII.HKL
fn resolution_range(xdsascii: &String, cell: &Cell) -> (f32, f32) {
    let mut dstarmin = f32::INFINITY;
    let mut dstarmax = -f32::INFINITY;

    let (avec, bvec, cvec) = abc2vector(cell.a, cell.b, cell.c, cell.alpha, cell.beta, cell.gamma);
    let (astar, bstar, cstar): (XYZ, XYZ, XYZ) = rec_cell(avec, bvec, cvec);

    let inp = std::fs::read_to_string(xdsascii);
    for l in inp.expect("Invalid line").lines() {
        let p = l.chars().nth(0);
        if p == Some('!') {
            continue;
        }
        let p: Vec<&str> = l.split_whitespace().collect();
        let h = p[0].trim().parse::<i32>();
        let h = h.expect("Error: unable to extract h from {p[0]}");
        let k = p[1].trim().parse::<i32>();
        let k = k.expect("Error: unable to extract h from {p[1]}");
        let l = p[2].trim().parse::<i32>();
        let l = l.expect("Error: unable to extract h from {p[2]}");
        let dstar = reciprocal_d_spacing(h, k, l, astar.clone(), bstar.clone(), cstar.clone());
        if dstar > dstarmax {
            dstarmax = dstar;
        }
        if dstar < dstarmin {
            dstarmin = dstar;
        }
    }
    let dstarmin = f32::sqrt(dstarmin);
    let dstarmax = f32::sqrt(dstarmax);

    (dstarmin, dstarmax)
}

// compute reciprocal resolution squared
fn reciprocal_d_spacing(h: i32, k: i32, l: i32, a: XYZ, b: XYZ, c: XYZ) -> f32 {
    let p: XYZ = a * h + b * k + c * l;
    p.clone() * p
}

fn write_pcf(pcfs: Vec<Pcf>, mcell: &Cell) {
    let mut content = String::from("data_my\n");
    content += &String::from("loop_\n");
    content += &String::from("_exptl_crystal_id\n");
    content += &String::from("_cell_length_a\n");
    content += &String::from("_cell_length_b\n");
    content += &String::from("_cell_length_c\n");
    content += &String::from("_cell_angle_alpha\n");
    content += &String::from("_cell_angle_beta\n");
    content += &String::from("_cell_angle_gamma\n");
    content += &String::from("_cell_measurement_reflns_used\n");
    content += &String::from("_cell_measurement_theta_min\n");
    content += &String::from("_cell_measurement_theta_max\n");
    let mut id = 1;
    for x in pcfs {
        // get dmin and dmax from XDS_ASCII.HKL
        let mut filename = x.file;
        if std::path::Path::new(&filename).is_dir() == true {
            filename += "/CORRECT.LP";
        }
        let filename = filename.replace("CORRECT.LP", "XDS_ASCII.HKL");
        let (dstarmin, dstarmax) = resolution_range(&filename, mcell);
        let thetamin = f32::asin(0.5 * dstarmin * x.wavelength);
        let thetamax = f32::asin(0.5 * dstarmax * x.wavelength);
        let (a, b, c, al, be, ga) = x.cellesd;
        let (pa, a) = precision(a);
        let (pb, b) = precision(b);
        let (pc, c) = precision(c);
        let (pal, al) = precision(al);
        let (pbe, be) = precision(be);
        let (pga, ga) = precision(ga);
        let s = format!(
            "{id:-3} \
		 {0:0>.1$}({a}) \
		 {2:0>.3$}({b}) \
		 {4:0>.5$}({c}) \
		 {6:0>.7$}({al}) \
		 {8:0>.9$}({be}) \
		 {10:0>.11$}({ga}) \
		 {12:6} \
		 {13:3.2} \
		 {14:3.2} \
		\n",
            x.cellabc.0,
            pa,
            x.cellabc.1,
            pb,
            x.cellabc.2,
            pc,
            x.cellabc.3,
            pal,
            x.cellabc.4,
            pbe,
            x.cellabc.5,
            pga,
            x.num_refl,
            thetamin,
            thetamax
        );
        content += &s;
        id += 1;
    }

    std::fs::write(PCFFILE, content).expect("Unable to write to PCF file");
}

// for a number < 1 return its precision
fn precision(x: f32) -> (usize, i32) {
    if x <= 0.0 {
        return (0, 0);
    }
    let mut v = x;
    let mut precision = 0;
    while v < 1.0 {
        v *= 10.0;
        precision += 1;
    }
    let p = v as i32;
    (precision, p)
}
