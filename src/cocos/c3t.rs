extern crate gl;
use math;

// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct Attribute {
//     size: u32,
//     mytype: String,
//     attribute: String
// }
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct Part {
//     id: String,
//     mytype: String,
//     indices: Vec<u32>
// }
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct Mesh {
//     attributes: Vec<Attribute>,
//     vertices: Vec<f64>,
//     parts: Vec<Part>
// }
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct Texture {
//     id: String,
//     filename: String,
//     mytype: String,
//     wrapModeU: String,
//     wrapModeV: String
// }
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct Material {
//     id: String,
//     ambient: Vector3,
//     diffuse: Vector3,
//     emissive: Vector3,
//     opacity: f64,
//     specular: Vector3,
//     shininess: f64,
//     textures: Vec<Texture>
// }
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct KeyFrame {
//     keytime: f64,
//     rotation: Quertanion,
//     scale: Vector3,
//     translation: Vector3
// }
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct Bone {
//     id: String,
//     keyframes: Vec<KeyFrame>
// }
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct Animation {
//     id: String,
//     length: f64,

// }
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// struct C3T {
//     version: String,
//     id: String,
//     meshes: Mesh
// }

struct AABB {
    _min: math::vector3::Vector3,
    _max: math::vector3::Vector3
}

struct MeshVertexAttribs {
    size: gl::types::GLint,
    t: gl::types::GLenum,
    vertexAttrib: i32,
    attribSizeBytes: i32,
}

struct MeshData {
    vertex: Vec<f64>,
    vertexSizeInFloat: i32,
    subMeshIndices: Vec<Vec<u16>>,
    subMeshIds: Vec<String>,
    subMeshAABB: Vec<AABB>,
    numIndex: i32,
    atttibs: Vec<MeshVertexAttribs>,
    attribCount: i32
}

struct MeshDatas {
    meshDatas: Vec<MeshData>,
}
