use crate::XDSheader::Geom;
use crate::XDSheader::Det;
use crate::XDSdatum::XDSdatum;
use crate::XYZ::XYZ;

pub fn U_AB(a: XYZ, b: XYZ, c: XYZ, geom: Geom, s0: XYZ) -> [f32; 9] {
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
	a: XYZ, b: XYZ, c: XYZ, geom: Geom, s0: XYZ) {
    let mut content = String::new();

    let f = data.scale();
    let Uab = U_AB(a, b, c, geom, s0);
    for s in data {
    	let c = s.cosines(Uab, geom, det);
	let dx = 512.0*d.dx()/det.nx();
	let dy = 512.0*d.dy()/det.ny();
	let dz = d.dz();
	let sthl: i16 = 10000 *(d.sinetheta()/geom.lambda());
        let line = format!(
            "{:4}{:4}{:4}{:8.2}{:8.2}{:4}{:8.5}{:8.5}{:8.5}{:8.5}{:8.5}{:8.5}{:8.2}{:7.2}{:4}\n",
            s.h,
            s.k,
            s.l,
            f * s.I,
            f * s.sigI,
            1,
            c[0],
            c[1],
            c[2],
            c[3],
            c[4],
            c[5],
            dx,
            dy,
            dz,
            s.S0[0],
            sthl
        );
    }
}
