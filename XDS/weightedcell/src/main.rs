use std::env;
use std::ops::{Add, Div, Mul, Sub};

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
    num_refl: i32,
    detector: String,
    nx: i32,
    ny: i32,
    qx: f32,
    qy: f32,
    distance: f32,
    wavelength: f32,
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

fn Vol(a: &XYZ, b: &XYZ, c: &XYZ) -> f32 {
    let cstar = cross(a, b);
    cstar * c.clone()
}

fn recCell(a: XYZ, b: XYZ, c: XYZ) -> (XYZ, XYZ, XYZ) {
    let V = Vol(&a, &b, &c);
    let astar = cross(&b, &c) / V;
    let bstar = cross(&c, &a) / V;
    let cstar = cross(&a, &b) / V;
    (astar, bstar, cstar)
}

fn main() {
    welcome();
    let args: Vec<String> = env::args().collect();
    // let has_esds = false;
    let mut all_cells: Vec<Cell> = Vec::new();
    let mut all_pcfs: Vec<Pcf> = Vec::new();
    let mut cells_w_esu: Vec<Cell> = Vec::new();
    let mut cells_wo_esu: Vec<Cell> = Vec::new();

    for idx in 1..args.len() {
        let mut filename = args[idx].clone();
        if std::path::Path::new(&filename).is_dir() == true {
            filename += "/CORRECT.LP";
        }
        let (cell, pcf) = rd_correct(filename);
        all_cells.push(cell.clone());
        all_pcfs.push(pcf.clone());
        if cell.sg == -1 || cell.a_esu == -1.0 {
            cells_wo_esu.push(cell);
        } else {
            cells_w_esu.push(cell);
        }
    }

    // no esu's available, take standard average
    if cells_wo_esu.len() > 1 {
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
        println!("<a> = {amean} +/- {aesu}");
        println!("<b> = {bmean} +/- {besu}");
        println!("<c> = {cmean} +/- {cesu}");
        println!("<alpha> = {alphamean} +/- {alphaesu}");
        println!("<beta> = {betamean} +/- {betaesu}");
        println!("<gamma> = {gammamean} +/- {gammaesu}");
    }

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

    let sg = cells_w_esu[0].sg;
    let mcell = Cell {
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

    xscaleinp(all_cells, mcell);
    mypcf(all_pcfs);
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

    let mut varw: f32 = 0.0;
    let mut sumw: f32 = 0.0;
    for it in vals.iter().zip(sigmas.iter()) {
        let (x, s) = it;
        varw += *s * (*x - mean) * (*x - mean);
        sumw += *s;
    }
    let sigma: f32 = (varw / sumw).sqrt();

    (mean, sigma)
}

fn welcome() {
    println!("! ---> Weighted cell parameters from XDS CORRECT.LP\n");
}

// read CORRECT.LP; path provided by filename
// assume to be valid path
// return true if esds are available
// if
fn rd_correct(filename: String) -> (Cell, Pcf) {
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
        num_refl: 0,
        detector: String::new(),
        nx: 0,
        ny: 0,
        qx: 0.0,
        qy: 0.0,
        distance: 0.0,
        wavelength: 0.0,
    };

    let correctlp = std::fs::read_to_string(filename);

    for l in correctlp.expect("Invalid line").lines() {
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
    (mycell, mypcf)
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
    println!("! ----------------> XSCALE.INP from weightedcell <----------------");
    println!("! ---> Weighted cell parameters from XDS CORRECT.LP");
    println!("!      Version 01/2025, (c) Tim Gruene");
    println!("!      tim.gruene@univie.ac.at");
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

// extract dmin and dmax from XDS_ASCII.HKL
fn resolution_range(xdsascii: &String, cell: &Cell) -> (f32, f32) {
    let dmax = -f32::INFINITY;
    let dmin = f32::INFINITY;

    (dmin, dmax)
}

fn mypcf(pcfs: Vec<Pcf>) {
    let mut content = String::from("data_my\n");
    let mut id = 1;
    for x in pcfs {
        let s = format!("{:34}{}\n", "_exptl_crystal_id", id);
        content = content + &s;
        id = id + 1;
        let s = format!("{:34}{}\n", "_cell_measurement_reflns_used", x.num_refl);
        content = content + &s;
        let s = format!("{:34}{}\n", "_exptl_absorpt_correction_type", "empirical");
        content = content + &s;
        let s = format!("{:34}{}\n", "_exptl_absorpt_correction_T_min", ".");
        content = content + &s;
        let s = format!("{:34}{}\n", "_exptl_absorpt_correction_T_max", ".");
        content = content + &s;
    }

    std::fs::write("My.pcf", content).expect("Unable to write to PCF file");
}
