use newton::math::vector3;
use newton::math::quaternion;

struct State {
    /// primary physics state

    position:           Vector3,     ///< the position of the cube center of mass in world coordinates (meters).
    momentum:           Vector3 ,    ///< the momentum of the cube in kilogram meters per second.
    orientation:        Quaternion,  ///< the orientation of the cube represented by a unit quaternion.
    angularMomentum:    Vector3,     ///< angular momentum vector.

    // secondary state

    velocity:           Vector3,     ///< velocity in meters per second (calculated from momentum).
    spin:               Quaternion,  ///< quaternion rate of change in orientation.
    angularVelocity:    Vector3,     ///< angular velocity (calculated from angularMomentum).
    bodyToWorld:        Matrix,      ///< body to world coordinates matrix.
    worldToBody:        Matrix,      ///< world to body coordinates matrix.

    /// constant state

    size:                 f32,      ///< length of the cube sides in meters.
    mass:                 f32,      ///< mass of the cube in kilograms.
    inverseMass:          f32,      ///< inverse of the mass used to convert momentum to velocity.
    inertiaTensor:        f32,      ///< inertia tensor of the cube (i have simplified it to a single value due to the mass properties a cube).
    inverseInertiaTensor: f32       ///< inverse inertia tensor used to convert angular momentum to angular velocity.

}

impl State {
    pub fn recalculate() {
        self.velocity = self.momentum * self.inverseMass;
        self.angularVelocity = self.angularMomentum * self.inverseInertiaTensor;
        self.orientation.normalize();
        self.spin = 0.5 * Quaternion(0, self.angularVelocity.x, self.angularVelocity.y, self.angularVelocity.z) * self.orientation;
        Matrix translation;
        self.translation.translate(position);
        self.bodyToWorld = translation * orientation.matrix();
        self.worldToBody = bodyToWorld.inverse();
    }
}

struct Derivative {
    velocity:   Vector3                ///< velocity is the derivative of position.
    force:      Vector3                   ///< force in the derivative of momentum.
    spin:       Quaternion                ///< spin is the derivative of the orientation quaternion.
    torque:     Vector3                  ///< torque is the derivative of angular momentum.
};

struct Cube {
    previous: State,
    current:  State,
}

impl Cube {
    pub fn new() -> Cube {
        Cube{previous: State{
                size = 1;
                mass = 1;
                inverseMass = 1.0f / current.mass;
                position = Vector(2,0,0);
                momentum = Vector(0,0,-10);
                orientation.identity();
                angularMomentum = Vector(0,0,0);
                inertiaTensor = current.mass * current.size * current.size * 1.0f / 6.0f;
                inverseInertiaTensor = 1.0f / current.inertiaTensor;
            },current: previous
        }
    }
}