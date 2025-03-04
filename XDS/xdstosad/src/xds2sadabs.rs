use crate::XDSdatum::XDSdatum;
use crate::XYZ::XYZ;
use crate::XDSheader::XDSheader;

pub fn U_AB(a: XYZ, b: XYZ, c: XYZ) -> [f32; 9] {
    let mut Uab: [f32; 9] = [0.0; 9];
    let astar = crate::XYZ::cross(b, c);
    let bstar = crate::XYZ::cross(c, a);
    let cstar = crate::XYZ::cross(a, b);

    let Vstar = 1. / (a * astar);
    Uab[0] = Vstar * astar.xyz[0];
    Uab[3] = Vstar * astar.xyz[1];
    Uab[6] = Vstar * astar.xyz[2];

    Uab[1] = Vstar * bstar.xyz[0];
    Uab[4] = Vstar * bstar.xyz[1];
    Uab[7] = Vstar * bstar.xyz[2];

    Uab[2] = Vstar * cstar.xyz[0];
    Uab[5] = Vstar * cstar.xyz[1];
    Uab[8] = Vstar * cstar.xyz[2];
    Uab
}

// each line of xds.sad consists of 17 entries in fixed format
// h k l I sigI 1 6x direction cosines 2d detector position x-value of
// beam direction and sin(theta)/lambda) as integer, multiplied by 10000
// 3(4I) 2(F8.2)  4I 6(F8.5) 2(F7.2) F8.2 F7.2 I5
pub fn write_xds2sad(filename: String, data: Vec<XDSdatum>, 
	header: XDSheader, dscale: f32) {
    let mut content = String::new();

    let f = dscale;
    let a: XYZ = header.a();
    let b: XYZ = header.a();
    let c: XYZ = header.a();
    // direct beam
    let s0: XYZ = header.dir_beam();
    // detector normal
    let detz: XYZ = header.detz();
    // angle between detector normal detz and direct beam
    let mu = detz.rad_sin_cos(s0);
    let Uab = U_AB(a, b, c);
    for s in data {
    	let c = s.cosines(Uab, geom, det);
	let xd = 512.0*s.xd()/det.nx();
	let yd = 512.0*s.yd()/det.ny();
	let zd = s.zd();
	let sthl: i16 = 10000 *(s.sinetheta()/geom.lambda());
	// the '1' is part of the original source of xds2sad
	// 2nd to last: refined angle between direct beam and detector normal
        let line = format!(
            "{:4}{:4}{:4}{:8.2}{:8.2}   1{:8.5}{:8.5}{:8.5}{:8.5}{:8.5}{:8.5}{:7.2}{:7.2}{:8.2}{:7.2}{:5}\n",
            s.h(),
            s.k(),
            s.l(),
            f * s.I(),
            f * s.sigI(),
            c[0],
            c[1],
            c[2],
            c[3],
            c[4],
            c[5],
            xd,
            yd,
            zd,
            mu,
            sthl
        );
    }
}
