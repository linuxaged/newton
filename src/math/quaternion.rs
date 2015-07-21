use std::ops::Mul;
use std::f32;

pub struct Quaternion {
    pub r: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32,
    pub data: [f32; 4]
}

impl Quaternion {
    pub fn new(r: f32, i: f32, j: f32, k: f32) -> Quaternion {
        Quaternion { r: r, i: i, j: j, k: k, data: [0f32; 4]}
    }
    pub fn normalize(&mut self) {
        let mut d = self.r * self.r + self.i * self.i   + self.j * self.j + self.k * self.k;
        if (d == 0f32) {
            self.r = 1.0f32;
            return;
        }
        d = d.sqrt();
        self.r *= d;
        self.i *= d;
        self.j *= d;
        self.k *= d;
    }
    pub fn slerp(from: &Quaternion, to: &Quaternion, t: f32, result: &mut Quaternion) {
        
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            r: self.r * other.r - self.i* other.i - self.j* other.j - self.k* other.k,
            i: self.r * other.i + self.i* other.r + self.j* other.k - self.k* other.j,
            j: self.r * other.j + self.j* other.r + self.k* other.i - self.i* other.k,
            k: self.r * other.k + self.k* other.r + self.i* other.j - self.j* other.i,
            data: self.data
        }
    }
}
