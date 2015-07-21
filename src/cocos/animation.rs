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

struct Joint {
    invBindPose: matrix::Matrix4x4,
    name: &str,
    parent: u8
}

struct Skeleton {
    joints: &[Joint],
    jointCount: u32
}

struct JointPose {
    rot: quaternion::Quaternion,
    trans: vector3::Vector3,
    scale: vector3::Vector3
}

struct SkeletonPose {
    skeleton: &Skeleton,
    localPoses: &[JointPose]
}

struct AnimationSample {
    jointPoses: &[JointPose]
}

struct AnimationClip {
    skeletons: &[Skeleton],
    fps: f32,
    frameCount: u32,
    samples: &[AnimationSample],
    isLooping: bool
}

struct SkinnedVertex {
    pos: [f64; 3],
    norm: [f64; 3],
    u: float,
    v: float,
    jointIndex: [u8; 4],
    weight: [f32; 3]
}

pub struct Animation {
    length: f32
}

impl Animation {
    pub fn new(data: Value) -> Animation {
        let bone_animation_array = data.find("animations").unwrap().as_array().unwrap();
        let _length = bone_animation_array[0].as_object().unwrap().get("length").unwrap();
        let bone_animations = bone_animation_array[0].as_object().unwrap().get("bones").unwrap().as_array().unwrap();
        let mut bone_keyframes = HashMap::<&str, Vec<animation::KeyFrame> >::new();

        for bone_anim in bone_animations {
            let bone_id = bone_anim.as_object().unwrap().get("boneId").unwrap().as_string().unwrap();
            let bone_keyframe_array = bone_anim.as_object().unwrap().get("keyframes").unwrap().as_array().unwrap();
            let mut kfs = Vec::<animation::KeyFrame>::new();
            for bkf in bone_keyframe_array {
                let keyframe = json::from_value(bkf.clone()).unwrap();
                kfs.push(keyframe);
            }
            bone_keyframes.insert(bone_id, kfs.clone());
            kfs.clear();
        }

        for (bone_id, kfs) in bone_keyframes {
            println!("boneId: {}", bone_id);
            for kf in kfs {
                println!("kf: {:?}", kf);
            }
        }
        Animation {
            length: _length
        }
    }
    pub fn update(&self,t: f32) {

        let moment = t % self.length;
        // 用插值计算出 moment 时刻的变化，然后对各个关节施加对应的变换
        for curve in bone_curves {
            setAnimationValue()
        }
    }
}