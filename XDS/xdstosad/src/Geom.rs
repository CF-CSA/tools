// geometry description of the diffraction experiment
use crate::XYZ::XYZ;

#[derive(Copy, Clone)]
pub struct Geom {
    rotaxis: XYZ,
    dir_beam: XYZ,
    pub dist: f32,
    pub lambda_: f32,
    pub S0R_: [f32; 3],
}

impl Geom {
    pub fn rotaxis(self) -> XYZ {
        self.rotaxis.clone()
    }
    pub fn dir_beam(self) -> XYZ {
        self.dir_beam.clone()
    }
    pub fn S0R(self) -> [f32; 3] {
        self.S0R_.clone()
    }
    pub fn det_dist(self) -> f32 {
        self.dist
    }
}
