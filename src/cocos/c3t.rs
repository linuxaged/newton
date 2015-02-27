extern crate gl;
use math;

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
    vertex: Vec<f32>,
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
