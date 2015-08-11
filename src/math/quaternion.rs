use std::ops::Mul;
use std::f32;
use std::cmp;

#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub r: f32,
    pub i: f32,
    pub j: f32,
    pub k: f32
}

impl Quaternion {
    pub fn new(r: f32, i: f32, j: f32, k: f32) -> Quaternion {
        Quaternion { r: r, i: i, j: j, k: k}
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
        let mut fCosine: f32;
        let fAngle: f32;
        let A: f32;
        let B: f32;

        // Parameter checking
        if (t<0.0f32 || t>1.0f32) {
            result.r = 0.0;
            result.i = 0.0;
            result.j = 0.0;
            result.k = 1.0;
            return;
        }

        // Find sine of Angle between Quaternion A and B (dot product between quaternion A and B)
        fCosine = from.r*to.r + from.i*to.i + from.j*to.j + from.k*to.k;

        if (fCosine < 0.0)
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
            let mut qi = Quaternion::new(-to.r, -to.i, -to.j, -to.k);

            Quaternion::slerp(from, &qi, t, result);
            return;
        }

        fCosine = fCosine.min(1.0);
        fAngle = fCosine.acos();

        // Avoid a division by zero
        if (fAngle==0.0f32)
        {
            result.i = from.i;
            result.j = from.j;
            result.k = from.k;
            result.r = from.r;
            return;
        }

        // Precompute some values
        A = ((1.0 - t)*fAngle).sin() / fAngle.sin();
        B = (t*fAngle).sin() / fAngle.sin();

        // Compute resulting quaternion
        result.i = A * from.i + B * to.i;
        result.j = A * from.j + B * to.j;
        result.k = A * from.k + B * to.k;
        result.r = A * from.r + B * to.r;

        // Normalise result
        result.normalize();
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion {
        Quaternion {
            r: self.r * other.r - self.i* other.i - self.j* other.j - self.k* other.k,
            i: self.r * other.i + self.i* other.r + self.j* other.k - self.k* other.j,
            j: self.r * other.j + self.j* other.r + self.k* other.i - self.i* other.k,
            k: self.r * other.k + self.k* other.r + self.i* other.j - self.j* other.i
        }
    }
}

#[test]
fn test_norm() {
    let mut q0 = Quaternion::new(0.1, 0.2, 0.3, 0.4);
    q0.normalize();
    println!("norm:{:?}", q0);

    let q1 = Quaternion::new(0.1, 0.2, 0.3, 0.4);
    let q2 = Quaternion::new(0.1, 0.2, 0.3, 0.3);

    Quaternion::slerp(&q1, &q2, 0.5, &mut q0);
    println!("slerp:{:?}", q0);

}