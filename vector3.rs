use std::fmt;

struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vector3 {
    fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{ x: x, y: y, z: z }
    }
    fn magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Add<Vector3, Vector3> for Vector3 {
    fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub<Vector3, Vector3> for Vector3 {
    fn sub(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul<Vector3, Vector3> for Vector3 {
    fn mul(&self, other: &Vector3) -> Vector3 {
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
}
