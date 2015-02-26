struct MeshVertexAttribs {
    size: gfx::gfx_gl::types::GLint,
    t: gfx::gfx_gl::types::GLenum,
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
