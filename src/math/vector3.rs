use std::fmt;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3{ x: x, y: y, z: z }
    }
    pub fn magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn add_scaled_vector(&mut self, vector: &Vector3, scale: f64) {
        self.x += vector.x * scale;
        self.y += vector.y * scale;
        self.z += vector.z * scale;
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

#[test]
fn test_mul() {
    let v0 = Vector3{x: 1.0f64,y: 2.0f64,z: 3.0f64};
    let v1 = Vector3{x: 2.0f64,y: 3.0f64,z: 4.0f64};
    println!("{:?}",v0 * v1);
    println!("{:?}",Vector3::new(1.0,2.0,3.0).magnitude());
    println!("{:?}",Vector3::new(7.0,8.0,9.0).add_scaled_vector(&v0, 2.0));
}
