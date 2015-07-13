use std::simd::f32x4;
use std::ops::{Add, Sub, Mul};

pub struct Matrix4x4 {
    pub x: f32x4,
    pub y: f32x4,
    pub z: f32x4,
    pub w: f32x4
}

impl Add for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, other: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {m: self.m + other.m}
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, other: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {m: self.m - other.m}
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;
    fn Mul(self, other: Matrix4x4) -> Matrix4x4 {

    }
}