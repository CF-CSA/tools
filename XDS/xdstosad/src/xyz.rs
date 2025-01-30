// 3D vectors
#[derive(Clone)]
struct XYZ {
    xyz: [f32; 3],
}

fn cross(x1: &XYZ, x2: &XYZ) -> XYZ {
    let x = &x1.xyz[1] * &x2.xyz[2] - &x1.xyz[2] * &x2.xyz[1];
    let y = &x1.xyz[2] * &x2.xyz[0] - &x1.xyz[0] * &x2.xyz[2];
    let z = &x1.xyz[0] * &x2.xyz[1] - &x1.xyz[1] * &x2.xyz[0];

    let xyz = XYZ { xyz: [x, y, z] };
    xyz
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

impl XYZ {
    fn uvec(&mut self) {
        let s = sqrt(self * self);
        u = 1. / s * u;
        s
    }
}
