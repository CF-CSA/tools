// detector information

use crate::XYZ::XYZ;

#[derive(Clone)]
pub struct Det {
    name_: String,
    detx_: XYZ,
    dety_: XYZ,
    detz_: XYZ,
    nx_: u16,
    ny_: u16,
    qx_: f32,
    qy_: f32,
    orgx_: f32,
    orgy_: f32,
}

impl Det {
    pub fn detz(&self) -> XYZ {
        crate::XYZ::cross(self.detx_, self.dety_)
    }
    pub fn name(&self) -> String {
        self.name_.clone()
    }
    pub fn detx(&self) -> XYZ {
        self.detx_
    }
    pub fn dety(&self) -> XYZ {
        self.dety_
    }
    pub fn nx(&self) -> u16 {
        self.nx_
    }
    pub fn ny(&self) -> u16 {
        self.ny_
    }
    pub fn qx(&self) -> f32 {
        self.qx_
    }
    pub fn qy(&self) -> f32 {
        self.qy_
    }
    pub fn orgx(&self) -> f32 {
        self.orgx_
    }
    pub fn orgy(&self) -> f32 {
        self.orgy_
    }
}
