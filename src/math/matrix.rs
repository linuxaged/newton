#![feature(core_simd)]
use std::simd::f32x4;
use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug)]
pub struct Matrix4x4 {
    pub x: f32x4,
    pub y: f32x4,
    pub z: f32x4,
    pub w: f32x4
}

impl Matrix4x4 {
    fn new(row00: f32, row01: f32, row02: f32, row03: f32, 
            row10: f32, row11: f32, row12: f32, row13: f32,
            row20: f32, row21: f32, row22: f32, row23: f32,
            row30: f32, row31: f32, row32: f32, row33: f32) -> Matrix4x4 {
        Matrix4x4{
            x: f32x4(row00 , row01 , row02 , row03),
            y: f32x4(row10 , row11 , row12 , row13),
            z: f32x4(row20 , row21 , row22 , row23),
            w: f32x4(row30 , row31 , row32 , row33)
        }
    }
}

impl Add for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, other: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, other: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: Matrix4x4) -> Matrix4x4 {
        let colume0 = f32x4(other.x.0, other.y.0, other.z.0, other.w.0);
        let colume1 = f32x4(other.x.1, other.y.1, other.z.1, other.w.1);
        let colume2 = f32x4(other.x.2, other.y.2, other.z.2, other.w.2);
        let colume3 = f32x4(other.x.3, other.y.3, other.z.3, other.w.3);
        let _x_0 = self.x * colume0;
        let _x_1 = self.x * colume1;
        let _x_2 = self.x * colume2;
        let _x_3 = self.x * colume3;

        let _y_0 = self.y * colume0;
        let _y_1 = self.y * colume1;
        let _y_2 = self.y * colume2;
        let _y_3 = self.y * colume3;

        let _z_0 = self.z * colume0;
        let _z_1 = self.z * colume1;
        let _z_2 = self.z * colume2;
        let _z_3 = self.z * colume3;

        let _w_0 = self.w * colume0;
        let _w_1 = self.w * colume1;
        let _w_2 = self.w * colume2;
        let _w_3 = self.w * colume3;

        Matrix4x4 {
            x: f32x4(_x_0.0 + _x_0.1 + _x_0.2 + _x_0.3, _x_1.0 + _x_1.1 + _x_1.2 + _x_1.3, _x_2.0 + _x_2.1 + _x_2.2 + _x_2.3, _x_3.0 + _x_3.1 + _x_3.2 + _x_3.3),
            y: f32x4(_y_0.0 + _y_0.1 + _y_0.2 + _y_0.3, _y_1.0 + _y_1.1 + _y_1.2 + _y_1.3, _y_2.0 + _y_2.1 + _y_2.2 + _y_2.3, _y_3.0 + _y_3.1 + _y_3.2 + _y_3.3),
            z: f32x4(_z_0.0 + _z_0.1 + _z_0.2 + _z_0.3, _z_1.0 + _z_1.1 + _z_1.2 + _z_1.3, _z_2.0 + _z_2.1 + _z_2.2 + _z_2.3, _z_3.0 + _z_3.1 + _z_3.2 + _z_3.3),
            w: f32x4(_w_0.0 + _w_0.1 + _w_0.2 + _w_0.3, _w_1.0 + _w_1.1 + _w_1.2 + _w_1.3, _w_2.0 + _w_2.1 + _w_2.2 + _w_2.3, _w_3.0 + _w_3.1 + _w_3.2 + _w_3.3),
        }
    }
}

#[test]
fn test() {
    let mat0 = Matrix4x4::new(1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0);
    let mat1 = Matrix4x4::new(11.0,22.0,33.0,44.0,55.0,66.0,77.0,88.0,88.0,77.0,66.0,55.0,44.0,33.0,22.0,11.0);
    println!("{:?}", mat0 * mat1);
}