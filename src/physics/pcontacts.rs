struct ParticalContact {
    partical: [Partical, ..2],
    restitution: f32,
    contactNormal: Vector3
}

impl ParticalContact {
    fn resolve(dutation: f32) {
        resolveVelocity(duration);
    }
    fn calculateSeparatingVelocity() -> f32 {
        let relativeVelocity = partical[0].getVelocity();
        if (particle[1])
            relativeVelocity -= particle[1].getVelocity();
        return relativeVelocity * contactNormal;
    }
    fn resolveVelocity(duration: f32) {

    }
}