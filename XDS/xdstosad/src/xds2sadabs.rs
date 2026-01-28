use crate::XDSdatum::XDSdatum;
use crate::XDSheader::XDSheader;
use crate::XYZ::XYZ;

pub fn U_AB(a: XYZ, b: XYZ, c: XYZ) -> [f32; 9] {
    let mut uab: [f32; 9] = [0.0; 9];
    let astar = crate::XYZ::cross(b, c);
    let bstar = crate::XYZ::cross(c, a);
    let cstar = crate::XYZ::cross(a, b);

    let vstar = 1. / (a * astar);
    println!("Comuting Uab from reciprocal lattice:");
    println!("a = {:4.3} {:4.3} {:4.3}", a.xyz[0], a.xyz[1], a.xyz[2]);
    println!("b = {:4.3} {:4.3} {:4.3}", b.xyz[0], b.xyz[1], b.xyz[2]);
    println!("c = {:4.3} {:4.3} {:4.3}", c.xyz[0], c.xyz[1], c.xyz[2]);
    uab[0] = vstar * astar.xyz[0];
    uab[3] = vstar * astar.xyz[1];
    uab[6] = vstar * astar.xyz[2];

    uab[1] = vstar * bstar.xyz[0];
    uab[4] = vstar * bstar.xyz[1];
    uab[7] = vstar * bstar.xyz[2];

    uab[2] = vstar * cstar.xyz[0];
    uab[5] = vstar * cstar.xyz[1];
    uab[8] = vstar * cstar.xyz[2];
    uab
}

// each line of xds.sad consists of 17 entries in fixed format
// h k l I sigI 1 6x direction cosines 2d detector position x-value of
// beam direction and sin(theta)/lambda) as integer, multiplied by 10000
// 3(4I) 2(F8.2)  4I 6(F8.5) 2(F7.2) F8.2 F7.2 I5
pub fn write_xds2sad(filename: String, data: Vec<XDSdatum>, header: XDSheader, dscale: f32) {
    let mut content = String::new();

    let f = 9999. / dscale;
    let a: XYZ = *header.a();
    let b: XYZ = *header.b();
    let c: XYZ = *header.c();
    // direct beam
    let s0: XYZ = *header.dir_beam();
    // detector normal
    let detz: XYZ = *header.detz();
    // angle between detector normal detz and direct beam
    let mu = detz.rad_sin_cos(&s0);
    let uab = U_AB(a, b, c);
    for s in data {
        let c = s.cosines(uab, &header);
        let xd = 512.0 * s.xd() / (*header.nx() as f32);
        let yd = 512.0 * s.yd() / (*header.ny() as f32);
        let zd = s.zd();
        let sthl: i16 = f32::round(10000.0 * (c[6])) as i16;
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
            mu[0],
            sthl
        );
        content += &line;
    }
    std::fs::write(filename, content).expect("Unable to write xds.sad");
}
