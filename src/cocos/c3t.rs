extern crate "gfx_gl" as gl;

extern crate serialize;
use serialize::json;
use std::io::File;

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

fn main() {
    let path = Path::new("orc.c3t");
    let raw_string = File::open(&path).read_to_string().unwrap();

    match json::from_str(raw_string) {
        Ok(json) => println!("{}", json.find("vertion").unwrap()),
        Err(err) => println!("{}", err),
    }
}