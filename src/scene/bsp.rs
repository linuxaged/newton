struct Plane {
    position: Vector3,
    direction: Vector3
}

struct BSPNode {
    plane: Plane,
    front: BSPNode,
    back: BSPNode
}