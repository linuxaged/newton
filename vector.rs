struct Vector3 {
    x: f32;
    y: f32;
    z: f32;
}

impl Vector3 {
    fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3{ x: x, y: y, z: z }
    }
    fn magnitude() -> f32 {
        x * x + y * x + z * z;
    }
}

impl Add<Vector3, Vector3> for Vector3 {
    fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x + other.x, y: self.y + other.y + z: self.z + other.z}
    }
}

impl Sub<Vector3, Vector3> for Vector3 {
    fn sub(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x, y: self.y - other.y + z: self.z - other.z}
    }
}

impl Mul<Vector3, Vector3> for Vector3 {
    fn sub(&self, other: &Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x, y: self.y - other.y + z: self.z - other.z}
    }
}
