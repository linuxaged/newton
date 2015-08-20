extern crate serde;
extern crate serde_json;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;
use std::error::Error;
use glium;
use glium::{DisplayBuild, Surface};


use cocos::animation;

#[derive(Copy, Clone, Serialize, Display)]
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


    pub fn new(path: &Path) -> C3t {

        implement_vertex!(C3tVertex, position, normal, texcoord, blendweight, blendindex);

        let mut f = match File::open(path) {
            Err(why) => panic!("{}. could not open {}, current dir: {}",
                Error::description(&why),
                path.display(),
                env::current_dir().unwrap().display()),
            Ok(file) => file,
        };
        let mut s = String::new();
        f.read_to_string(&mut s);

        let data: serde_json::Value = serde_json::from_str(&s).unwrap();

        let meshes = data.find("meshes").unwrap();
        let mesh_array = meshes.as_array().unwrap();
        let mesh = mesh_array[0].as_object().unwrap();

        // get vertex indices
        let parts = mesh.get("parts").unwrap();
        let part_array = parts.as_array().unwrap();
        let part = part_array[0].as_object().unwrap();
        let index_array:Vec<u32> = (serde_json::from_value(part.get("indices").unwrap().clone()) ).unwrap();
        // get vertex positions
        let vertices: Vec<f64> = (serde_json::from_value(mesh.get("vertices").unwrap().clone()) ).unwrap();

        let mut vertex_array:Vec<C3tVertex> = Vec::<C3tVertex>::new();

        for i in (0..vertices.len()).step_by(16) {
            let vertex = C3tVertex{
                position:[vertices[i+0], vertices[i+1],vertices[i+2]],
                normal:[vertices[i+3],vertices[i+4],vertices[i+5]],
                texcoord:[vertices[i+6],vertices[i+7]],
                blendweight:[vertices[i+8], vertices[i+9],vertices[i+10],vertices[i+11]],
                blendindex:[vertices[i+12], vertices[i+13],vertices[i+14],vertices[i+15]]
            };
            vertex_array.push(vertex);
        }

        // get bones
        // let node_part = node_array[0].as_object().unwrap();
        // let parts = node_part.get("parts").unwrap().as_array().unwrap();
        // let bones = parts[0].as_object().unwrap().get("bones").unwrap().as_array().unwrap();
        // let mut bone_array = Vec::<animation::Bone>::new();

        // fill bone curves

        // fill blendstate

        // caculate MatrixPalette: iv_binded_matrix * currentPose

        // TODO
        C3t{vertices:vertex_array, indices: index_array, texture:vec!["path".to_string()]}

    }

}
