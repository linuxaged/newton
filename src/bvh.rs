struct BoundingSphere {
    center: Vector3,
    radius: f32
}

struct BoundingBox {
	center: Vector3,
	halfSize: Vector3
}

struct BVHNode<'r, BoundingVolumeClass> {
    children: [&'r BVHNode, ..2],
    volume: BoundingVolumeClass,
    body: &RigidBody
}

struct PotentialContact {
    body: [&'r RigidBody, ..2]
}

impl BVHNode {
	fn insert(body: &mut RigidBody, volume: &BoundingVolumeClass) -> ret {
		// add code here
	}
	fn isLeaf() -> bool {
		return body != 
	}
}