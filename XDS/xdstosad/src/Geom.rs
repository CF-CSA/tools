// geometry description of the diffraction experiment
use crate::XYZ::XYZ;

#[derive(Copy, Clone)]
pub struct Geom {
    rotaxis_: XYZ,
    S0_: XYZ,
    pub D_: f32,
    pub lambda_: f32,
    pub S0R_: [f32; 3],
}

impl Geom {
    pub fn R(self) -> XYZ {
        self.rotaxis_.clone()
    }
    pub fn S0(self) -> XYZ {
        self.S0_.clone()
    }
    pub fn S0R(self) -> [f32; 3] {
        self.S0R_.clone()
    }
    pub fn D(self) -> f32 {
        self.D_
    }
}
