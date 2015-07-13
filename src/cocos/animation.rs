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

/// static binding pose tree of model
#[derive(Clone, Serialize, Deserialize, Display)]
pub struct Node {
    pub id: String,
    pub skeleton: bool,
    pub transform: [f64; 16],
    pub children: Option<Vec<Node>>
}

/// visit the nodes and store 
impl Node {
    pub fn visit(&self) {

    }
}

/// a key frame of animation
#[derive(Clone, Serialize, Deserialize, Display, Debug)]
pub struct KeyFrame {
    keytime: f32,
    rotation: [f32; 4],
    scale: [f32; 3],
    translation: [f32; 3]
}

pub struct Animation {
}

impl Animation {
    pub fn update(&self,t: f32) {
        
    }
}