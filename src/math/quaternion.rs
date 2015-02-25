mod quaternion {

    pub struct Quaternion {
        r: f32,
        i: f32,
        j: f32,
        k: f32,
        data: [f32; 4]
    }

    impl Quaternion {
        fn new(r: f32, i: f32, j: f32, k: f32) -> Quaternion {
            Quaternion { r: r, i: i, j: j, k: k, data: [0, ..4]}
        }
        fn normalize(&self) -> ret {
            let d = self.r * self.r + self.i * self.i   + self.j * self.j + self.k * self.k;
            if (d == 0) {
                r =1;
                return;
            }
            d = 1.0 / real_sqrt(d);
            self.r *= d;
            self.i *= d;
            self.j *= d;
            self.k *= d;
        }
    }

    impl Mul<Quaternion, Quaternion> for Quaternion {
        fn mul(&self, other: &Quaternion) -> Quaternion {
            self.r = self.r * other.r - self.i* other.i - self.j* other.j - self.k* other.k;
            self.i = self.r * other.i + self.i* other.r + self.j* other.k - self.k* other.j;
            self.j = self.r * other.j + self.j* other.r + self.k* other.i - self.i* other.k;
            self.k = self.r * other.k + self.k* other.r + self.i* other.j - self.j* other.i;
        }
    }

}