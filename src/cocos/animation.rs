use math::{vector3, quaternion};
use serde::json::{self, Value};

#[derive(Clone, Serialize, Deserialize, Display)]
pub struct Bone {
    node: String,
    transform: [f64; 16]
}

pub struct BoneBlendState {
    localTranslate: vector3::Vector3,
    localRot: quaternion::Quaternion,
    localScale: vector3::Vector3,
    weight: f32
}
#[derive(Clone, Serialize, Deserialize, Display)]
pub struct Node {
    pub id: String,
    pub skeleton: bool,
    pub transform: [f64; 16],
    pub children: Option<Vec<Node>>
}

#[derive(Clone, Serialize, Deserialize, Display, Debug)]
pub struct KeyFrame {
    keytime: f32,
    rotation: [f32; 4],
    scale: [f32; 3],
    translation: [f32; 3]
}