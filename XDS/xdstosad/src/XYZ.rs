// 3D vectors
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct XYZ {
    pub xyz: [f32; 3],
}

impl XYZ {
    fn cross(x1: &XYZ, x2: &XYZ) -> XYZ {
        let x = &x1.xyz[1] * &x2.xyz[2] - &x1.xyz[2] * &x2.xyz[1];
        let y = &x1.xyz[2] * &x2.xyz[0] - &x1.xyz[0] * &x2.xyz[2];
        let z = &x1.xyz[0] * &x2.xyz[1] - &x1.xyz[1] * &x2.xyz[0];

        let xyz = XYZ { xyz: [x, y, z] };
        xyz
    }
    pub fn length(self) -> f32 {
    	let s = f32::sqrt(self * self);
	s
    }
    // normalises self and returns its length
    pub fn uvec(&mut self) -> f32 {
    	let me = *self;
    	let s = f32::sqrt( me * (*self));
	*self = (*self) / s;
        s
    }
    // computes the angle between self and another vector
    // stores angle in radians, its sine and cosine value
    pub fn rad_sin_cos(self, other: XYZ) -> [f32; 3] {
        let v1v2 = f32::sqrt((self * self) * (other * other));
        let cosine = (1. / v1v2) * (self * other);
        let sine = f32::sqrt(f32::max(0.0, 1.0 - cosine * cosine));
        let phi = f32::atan2(sine, cosine);
        [phi, sine, cosine]
    }
}

impl Mul for XYZ {
    type Output = f32;
    fn mul(self, other: XYZ) -> f32 {
        self.xyz[0] * other.xyz[0] + self.xyz[1] * other.xyz[1] + self.xyz[2] * other.xyz[2]
    }
}

impl Mul<f32> for XYZ {
    type Output = Self;
    fn mul(self, s: f32) -> Self {
        Self {
            xyz: [s * self.xyz[0], s * self.xyz[1], s * self.xyz[2]],
        }
    }
}

impl Mul<i32> for XYZ {
    type Output = Self;
    fn mul(self, si: i32) -> Self {
        let s = si as f32;
        Self {
            xyz: [s * self.xyz[0], s * self.xyz[1], s * self.xyz[2]],
        }
    }
}

impl Div<f32> for XYZ {
    type Output = Self;
    fn div(self, d: f32) -> Self {
        let s = 1.0 / d;
        Self {
            xyz: [s * self.xyz[0], s * self.xyz[1], s * self.xyz[2]],
        }
    }
}

impl Add for XYZ {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            xyz: [
                self.xyz[0] + other.xyz[0],
                self.xyz[1] + other.xyz[1],
                self.xyz[2] + other.xyz[2],
            ],
        }
    }
}

impl Sub for XYZ {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            xyz: [
                self.xyz[0] - other.xyz[0],
                self.xyz[1] - other.xyz[1],
                self.xyz[2] - other.xyz[2],
            ],
        }
    }
}
