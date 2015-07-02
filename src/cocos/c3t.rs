use serde::json::{self, Value};

use std::io::prelude::*;
use std::fs::File;

use glium;
use glium::{DisplayBuild, Surface};

#[derive(Copy, Clone, Serialize, Deserialize, Display)]
pub struct C3tVertex {
    position:   [f64; 3],
    normal:     [f64; 3],
    texcoord:   [f64; 2],
    blendweight:[f64; 4],
    blendindex: [f64; 4]
}

pub struct C3t {
    pub vertices: Vec<C3tVertex>,
    pub indices: Vec<u32>,
    pub texture: Vec<String>
}

impl C3t {
    pub fn new(path: &str) -> C3t {
        
        implement_vertex!(C3tVertex, position, normal, texcoord, blendweight, blendindex);

        let mut f = File::open(path).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s);

        let data: Value = json::from_str(&s).unwrap();

        let meshes = data.find("meshes").unwrap();
        let mesh_array = meshes.as_array().unwrap();
        let mesh = mesh_array[0].as_object().unwrap();

        // get vertex indices
        let parts = mesh.get("parts").unwrap();
        let part_array = parts.as_array().unwrap();
        let part = part_array[0].as_object().unwrap();
        let index_array:Vec<u32> = (json::from_value(part.get("indices").unwrap().clone()) ).unwrap();
        // get vertex positions
        let vertices:Vec<f64> = (json::from_value(mesh.get("vertices").unwrap().clone()) ).unwrap();
        let mut vertex_array:Vec<C3tVertex> = Vec::<C3tVertex>::new();
        for i in (0..vertices.len()).step_by(16) {
            let vertex = C3tVertex{
                position:   [vertices[i+0], vertices[i+1],vertices[i+2]],
                normal:     [vertices[i+3], vertices[i+4],vertices[i+5]],
                texcoord:   [vertices[i+6], vertices[i+7]],
                blendweight:[vertices[i+8], vertices[i+9],vertices[i+10],vertices[i+11]],
                blendindex: [vertices[i+12],vertices[i+13],vertices[i+14],vertices[i+15]]
            };
            vertex_array.push(vertex);
        }
        C3t{vertices:vertex_array, indices: index_array, texture:vec!["path".to_string()]}
        
    }
}
