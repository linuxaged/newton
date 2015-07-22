use std::ops::Mul;
use std::f32;
use std::cmp;

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
        let fCosine: f32;
        let fAngle: f32;
        let A: f32;
        let B: f32;

        /* Parameter checking */
        if (t<0.0f || t>1.0f) {
            result.r = 0;
            result.i = 0;
            result.j = 0;
            result.k = 1;
            return;
        }

        /* Find sine of Angle between Quaternion A and B (dot product between quaternion A and B) */
        fCosine = from.r*to.r + from.i*to.i + from.j*to.j + from.k*to.k;

        if (fCosine < 0)
        {
            let qi: Quaternion;

            /*
                <http://www.magic-software.com/Documentation/Quaternions.pdf>

                "It is important to note that the quaternions q and -q represent
                the same rotation... while either quaternion will do, the
                interpolation methods require choosing one over the other.

                "Although q1 and -q1 represent the same rotation, the values of
                Slerp(t; q0, q1) and Slerp(t; q0,-q1) are not the same. It is
                customary to choose the sign... on q1 so that... the angle
                between q0 and q1 is acute. This choice avoids extra
                spinning caused by the interpolated rotations."
            */
            qi.r = -to.r;
            qi.i = -to.i;
            qi.j = -to.j;
            qi.k = -to.k;
            
            slerp(from, qi, t, result);
            return;
        }

        fCosine = cmp::min(fCosine, 1.0f);
        fAngle = ACOS_VAL ((fCosine * 65536) as i32);

        /* Avoid a division by zero */
        if (fAngle==0.0f)
        {
            qOut = from;
            return;
        }

        /* Precompute some values */
        A = (float)(PVRTFSIN((1.0f-t)*fAngle) / PVRTFSIN(fAngle));
        B = (float)(PVRTFSIN(t*fAngle) / PVRTFSIN(fAngle));

        /* Compute resulting quaternion */
        qOut.i = A * from.i + B * to.i;
        qOut.j = A * from.j + B * to.j;
        qOut.k = A * from.k + B * to.k;
        qOut.r = A * from.r + B * to.r;

        /* Normalise result */
        PVRTMatrixQuaternionNormalizeF(qOut);
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
