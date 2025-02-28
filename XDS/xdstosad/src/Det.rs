// detector information

use crate::XYZ::XYZ;

#[derive(Clone)]
pub struct Det {
    name: String,
    detx: XYZ,
    dety: XYZ,
    detz: XYZ,
    nx: u16,
    ny: u16,
    qx: f32,
    qy: f32,
    orgx: f32,
    orgy: f32,
}

impl Det {
    pub fn detz(&self) -> XYZ {
        crate::XYZ::cross(self.detx, self.dety)
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn detx(&self) -> XYZ {
        self.detx
    }
    pub fn dety(&self) -> XYZ {
        self.dety
    }
    pub fn nx(&self) -> u16 {
        self.nx
    }
    pub fn ny(&self) -> u16 {
        self.ny
    }
    pub fn qx(&self) -> f32 {
        self.qx
    }
    pub fn qy(&self) -> f32 {
        self.qy
    }
    pub fn orgx(&self) -> f32 {
        self.orgx
    }
    pub fn orgy(&self) -> f32 {
        self.orgy
    }
}
