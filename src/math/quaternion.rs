use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub struct Quaternion<T> {
    pub r: T, // real number
    pub i: T,
    pub j: T,
    pub k: T
}

impl Quaternion<T> where T == f32 || T == f64 {
    pub fn new(r: T, i: T, j: T, k: T) -> Quaternion<T> {
        Quaternion<T> { r: r, i: i, j: j, k: k}
    }

    pub fn normalize(&mut self) {
        let mut d = self.r * self.r + self.i * self.i   + self.j * self.j + self.k * self.k;
        if d == 0.0 {
            self.r = 1.0;
            return;
        }

        d = d.sqrt();
        d = 1.0 / d;

        self.r *= d;
        self.i *= d;
        self.j *= d;
        self.k *= d;
    }
    pub fn slerp(from: &Quaternion, to: &Quaternion, t: f32, result: &mut Quaternion) {
        let mut cosine: T;
        let angle: T;
        let a: T;
        let b: T;

        // Parameter checking
        if t<0.0f32 || t>1.0f32 {
            result.r = 0.0;
            result.i = 0.0;
            result.j = 0.0;
            result.k = 1.0;
            return;
        }

        // Find sine of Angle between Quaternion A and B (dot product between quaternion A and B)
        cosine = from.r*to.r + from.i*to.i + from.j*to.j + from.k*to.k;

        if cosine < 0.0
        {
            // <http://www.magic-software.com/Documentation/Quaternions.pdf>

            // "It is important to note that the quaternions q and -q represent
            // the same rotation... while either quaternion will do, the
            // interpolation methods require choosing one over the other.

            // "Although q1 and -q1 represent the same rotation, the values of
            // Slerp(t; q0, q1) and Slerp(t; q0,-q1) are not the same. It is
            // customary to choose the sign... on q1 so that... the angle
            // between q0 and q1 is acute. This choice avoids extra
            // spinning caused by the interpolated rotations."
            let qi = Quaternion<T>::new(-to.r, -to.i, -to.j, -to.k);

            Quaternion<T>::slerp(from, &qi, t, result);
            return;
        }

        cosine = cosine.min(1.0);
        angle = cosine.acos();

        // Avoid a division by zero
        if angle == 0.0f32
        {
            result.i = from.i;
            result.j = from.j;
            result.k = from.k;
            result.r = from.r;
            return;
        }

        // Precompute some values
        a = ((1.0 - t)*angle).sin() / angle.sin();
        b = (t*angle).sin() / angle.sin();

        // Compute resulting quaternion
        result.i = a * from.i + b * to.i;
        result.j = a * from.j + b * to.j;
        result.k = a * from.k + b * to.k;
        result.r = a * from.r + b * to.r;

        // Normalise result
        // result.normalize();
    }
}

impl Mul for Quaternion<T> {
    type Output = Quaternion<T>;
    fn mul(self, other: Quaternion<T>) -> Quaternion<T> {
        Quaternion<T> {
            r: self.r * other.r - self.i* other.i - self.j* other.j - self.k* other.k,
            i: self.r * other.i + self.i* other.r + self.j* other.k - self.k* other.j,
            j: self.r * other.j + self.j* other.r + self.k* other.i - self.i* other.k,
            k: self.r * other.k + self.k* other.r + self.i* other.j - self.j* other.i
        }
    }
}

///
/// compare to the result of http://www.euclideanspace.com/maths/algebra/realNormedAlgebra/quaternions/slerp/
///
#[test]
fn test_norm() {
    let mut q0 = Quaternion::<f64>::new(0.1, 0.2, 0.3, 0.4);
    q0.normalize();
    println!("norm:{:?}", q0);

    let q1 = Quaternion::<f64>::new(0.1, 0.2, 0.3, 0.4);
    let q2 = Quaternion::<f64>::new(0.1, 0.2, 0.3, 0.3);

    Quaternion::<f64>::slerp(&q1, &q2, 0.5, &mut q0);
    println!("slerp:{:?}", q0);

}