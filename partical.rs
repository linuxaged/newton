struct Partical {
    position:       Vector3,
    velocity:       Vector3,
    acceleration:   Vector3,
    damping:        f32,
    inverseMass:    f32
}

impl Partical {
    fn intergrate(duration: f32) {
        assert!(duration > 0.0);
        // Update linear position.
        position.add_scaled_vector(velocity, duration);
        // Work out the acceleration from the force.
        Vector3 resultingAcc = acceleration;
        resultingAcc.addScaledVector(forceAccum, inverseMass);
        // Update linear velocity from the acceleration.
        velocity.add_scale_vector(resultingAcc, duration);
        // Impose drag.
        velocity *= real_pow(damping, duration);
    }
}
