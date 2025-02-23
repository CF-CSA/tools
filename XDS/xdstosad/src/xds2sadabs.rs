use XYZ;

pub fn U_AB (a: XYZ, b: XYZ, c: XYZ, geom: Geom, s0: XYZ) -> [f32; 9] {
    let mut Uab: [f32; 9] = [0.0; 9];
    let astar: [f32; 3] = XYZ::cross(a, b);

}
