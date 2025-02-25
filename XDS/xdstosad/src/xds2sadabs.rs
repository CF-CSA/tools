use crate::XYZ::XYZ;
use crate::Geom::Geom;

pub fn U_AB (a: XYZ, b: XYZ, c: XYZ, geom: Geom, s0: XYZ) -> [f32; 9] {
    let mut Uab: [f32; 9] = [0.0; 9];
    let astar = crate::XYZ::cross(b, c);
    let bstar = crate::XYZ::cross(c, a);
    let cstar = crate::XYZ::cross(a, b);

    let Vstar = 1./ (a*astar);
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
