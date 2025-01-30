mod xyz;
// geometry description of the diffraction experiment

pub struct Geom {
	rotaxis_: XYZ,
	S0_: XYZ,
	D_: f32,
	lambda_: f32,
	S0R_: [f32; 3],
}

impl Geom {
   // computes the rotation angle between S0 and the given axis
   // stores angle in radians, its sine and cosine value
   pub fn S0_rotation_angle(raxis: XYZ) -> [f32;3] {
     let v1v2 = f32::sqrt(self.S0_ * self.S0_ * raxis*raxis);
     let cosine = (1./v1v2) * self.S0_ * raxis;
     let sine = f32::sqrt(f32::max(0.0, 1.0-cosine*cosine));
     let phi  = f32::atan2(sine, cosine);
     [phi, sine, cosine];
   }
}

