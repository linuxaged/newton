extern crate "gfx_gl" as gl;

struct MeshVertexAttribs {
    size: GLint,
    t: gl::types::GLenum,
    vertexAttrib: int,
    attribSizeBytes: int,
}

struct MeshData {
    let IndexArray = Vec<u16>,
    vertex: Vec<f32>,
    vertexSizeInFloat: int,
    subMeshIndices: Vec<IndexArray>,
    subMeshIds: Vec<String>,
    subMeshAABB: Vec<AABB>,
    numIndex: int,
    atttibs: Vec<MeshVertexAttribs>,
    attribCount: int
}

struct MeshDatas {
    meshDatas: Vec<MeshData>,
}
