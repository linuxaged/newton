struct Partical {
    position:       Vector3,
    velocity:       Vector3,
    acceleration:   Vector3,
    damping:        f32,
    inverseMass:    f32,
    forceAccum:     Vector3,
}

impl Partical {
    fn addForce(&self, force: &Vector3) {
        self.forceAccum += force;
    }
    fn clearAccumulator() {
        forceAccum.clear();
    }
    fn intergrate(duration: f32) {
        assert!(duration > 0.0);
        position.add_scaled_vector(velocity, duration);

        Vector3 resultingAcc = acceleration;
        resultingAcc.add_scaled_vector(forceAccum, inverseMass);

        velocity.add_scaled_vector(resultingAcc, duration);

        velocity *= real_pow(damping, duration);

        clearAccumulator();
    }
}
