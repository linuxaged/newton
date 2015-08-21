extern crate serde;
extern crate serde_json;

use math::{vector3, matrix, quaternion};
use std::collections::HashMap;
use std::collections::BTreeMap;

pub struct BoneBlendState {
    localTranslate: vector3::Vector3,
    localRot: quaternion::Quaternion,
    localScale: vector3::Vector3,
    weight: f32
}

struct Skeleton<'a> {
    bones: Vec<&'a Bone >,
    rootBones: Vec<&'a Bone >
}

impl<'a> Skeleton<'a> {
    // pub fn new(data: serde_json::Value) -> Skeleton<'a> {
    //     let nodes = data.find("nodes").unwrap().as_array().unwrap();
    //     for node in nodes {
    //         // is node or skeleton
    //         if node.as_object().unwrap().get("skeleton").unwrap().as_f64().unwrap() {

    //         }
    //     }
    // }
}

///
/// use flat struct instread of linked list,
///
pub struct Bone {
    id: String,
    transform: matrix::Matrix4x4,
    parent: u8
}

/// static binding pose tree of model
#[derive(Clone, Serialize, Deserialize, Display)]
pub struct Node {
    pub id: String,
    pub skeleton: bool,
    pub transform: [f32; 16],
    pub children: Option<Vec<Node>>
}

/// visit the nodes and store
impl Node {
    pub fn to_bones(self, bones: &mut Vec<Bone>, mut parent: u8, mut into_branch: bool) {
        bones.push(Bone{id: self.id.clone(), // TODO
                        transform: matrix::Matrix4x4::new(
                            self.transform[0],self.transform[1],self.transform[2],self.transform[3],
                            self.transform[4],self.transform[5],self.transform[6],self.transform[7],
                            self.transform[8],self.transform[9],self.transform[10],self.transform[11],
                            self.transform[12],self.transform[13],self.transform[14],self.transform[15]
                        ), 
                        parent: parent});
        match self.children {
            Some(bone_vec) => {
                println!("{:?}", parent);
                for bone in bone_vec {
                    if (into_branch) {
                        parent = parent + 1;
                        bone.to_bones(bones, parent, into_branch); // 递归；深度优先遍历
                    }
                    
                    into_branch = true;
                }
            },
            None => (into_branch = false)
        }
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

struct JointPose {
    rot: quaternion::Quaternion,
    trans: vector3::Vector3,
    scale: vector3::Vector3
}

struct SkeletonPose<'a> {
    skeleton: &'a Skeleton<'a>,
    localPoses: &'a [JointPose]
}

struct AnimationSample<'a> {
    jointPoses: &'a [JointPose]
}

struct AnimationClip<'a> {
    skeletons: &'a [Skeleton<'a>],
    fps: f32,
    frameCount: u32,
    samples: &'a [AnimationSample<'a>],
    isLooping: bool
}

struct SkinnedVertex {
    pos: [f64; 3],
    norm: [f64; 3],
    u: f32,
    v: f32,
    jointIndex: [u8; 4],
    weight: [f32; 3]
}

// 插值获得平滑曲线
struct AnimationCurve {
    curve: HashMap<String, Vec<KeyFrame> >
}

impl AnimationCurve {
    // 计算当前时间的 QTS
    pub fn evaluate(time: f32, dst: &[f32]) {
        
    }
}

pub struct Animation {
    curve: AnimationCurve,
    length: f32
}

impl Animation {
    // deserialize animation
    pub fn new(data: serde_json::Value) -> Animation {
        let bone_animation_array = data.find("animations").unwrap().as_array().unwrap();
        let _length = bone_animation_array[0].as_object().unwrap().get("length").unwrap().as_f64().unwrap() as f32;
        let bone_animations = bone_animation_array[0].as_object().unwrap().get("bones").unwrap().as_array().unwrap();
        let mut bone_keyframes = HashMap::<String, Vec<KeyFrame> >::new();
        for bone_anim in bone_animations {
            let bone_id = bone_anim.as_object().unwrap().get("boneId").unwrap().as_string().unwrap().to_string();
            let bone_keyframe_array = bone_anim.as_object().unwrap().get("keyframes").unwrap().as_array().unwrap();
            let mut kfs = Vec::<KeyFrame>::new();
            for bkf in bone_keyframe_array {
                let keyframe = serde_json::from_value(bkf.clone()).unwrap();
                kfs.push(keyframe);
            }
            bone_keyframes.insert(bone_id, kfs.clone());
            kfs.clear();
        }

        // get nodes
        let nodes = data.find("nodes").unwrap();
        let node_array = nodes.as_array().unwrap();
        let node = node_array[1].as_object().unwrap();
        let mut node_tree = Animation::parseNodes(node);

        // store nodes into Vec<Bone>
        let mut bones = Vec::<Bone>::new();
        let mut into_branch = true;
        node_tree.to_bones(&mut bones, 0xff, into_branch);


        // store into Vec<Bone>
        let bones = Vec::<Bone>::new();

        // for (bone_id, kfs) in bone_keyframes {
        //     println!("boneId: {}", bone_id);
        //     for kf in kfs {
        //         println!("kf: {:?}", kf);
        //     }
        // }

        Animation {
            curve: AnimationCurve{curve: bone_keyframes.clone()},
            length: _length
        }
    }

    ///
    /// we only parse the skeleton node here,
    /// TODO: parse Node nodes
    ///
    pub fn parseNodes(jnode: &BTreeMap<String, serde_json::Value>) -> Node {
        Node {
            id: jnode.get("id").unwrap().as_string().unwrap().to_string(),
            skeleton: jnode.get("skeleton").unwrap().as_boolean().unwrap(),
            transform: (serde_json::from_value(jnode.get("transform").unwrap().clone()) ).unwrap(),
            children: match jnode.get("children") {
                Some(children) => {
                    let mut nodes = Vec::<Node>::new();
                    for child in children.as_array().unwrap() {
                        println!("add a child");
                        nodes.push(Animation::parseNodes(child.as_object().unwrap()));
                    }
                    Some(nodes)
                },
                None => {
                    None
                }
            }
        }
    }

    // fn setAnimationValue(arg: typ) -> ret {
        
    // }
    pub fn update(&self,t: f32) {

        let moment = t % self.length;

        // 计算调色板矩阵
        // 


        // 用插值计算出 moment 时刻的变化，然后对各个关节施加对应的变换
        // for curve in bone_curves {
        //     setAnimationValue()
        // }

        //
        // 
        // 
    }
}