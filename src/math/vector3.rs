use std::fmt;
use std::ops::{Add, Sub, Mul};

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{ x: x, y: y, z: z }
    }
    pub fn magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn add_scaled_vector(&mut self, vector: &Vector3, scale: f32) {
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

impl fmt::Show for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}



#[test]
fn test_mul() {
    let v0 = Vector3{x: 1.0f32,y: 2.0f32,z: 3.0f32};
    let v1 = Vector3{x: 2.0f32,y: 3.0f32,z: 4.0f32};
    println!("{}",v0 * v1);
    println!("{}",Vector3::new(1.0,2.0,3.0).magnitude());
    println!("{}",Vector3::new(7.0,8.0,9.0).add_scaled_vector(&v0, 2.0));
}
