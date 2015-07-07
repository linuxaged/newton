use serde::json::{self, Value};
use serde;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;
use std::error::Error;
use glium;
use glium::{DisplayBuild, Surface};
use std::collections::BTreeMap;

#[derive(Copy, Clone, Serialize, Display)]
pub struct C3tVertex {
    position:   [f64; 3],
    normal:     [f64; 3],
    texcoord:   [f64; 2],
    blendweight:[f64; 4],
    blendindex: [f64; 4]
}

struct Bone {
    id: String,
    transform: [f64; 16]
}
#[derive(Clone, Serialize, Deserialize, Display)]
struct Node {
    id: String,
    skeleton: bool,
    transform: [f64; 16],
    children: Option<Vec<Node>>
}

struct SkeletalAnimation {
    bones: Vec<Bone>,

}

// impl serde::Deserialize for C3tVertex {
//     #[inline]
//     fn deserialize<D>(deserializer: &mut D) -> Result<C3tVertex, D::Error>
//         where D: serde::Deserializer {

//         let vertices: [f64; 16] = try!(serde::Deserialize::visit_seq(visitor));

//         Ok(C3tVertex {
//             position:[vertices[0], vertices[1],vertices[2]],
//             normal:[vertices[3], vertices[4], vertices[5]],
//             texcood:[vertices[6], vertices[7]],
//             blendweight:[vertices[8], vertices[9], vertices[10], vertices[11]],
//             blendindex:[vertices[12], vertices[13],vertices[14], vertices[15]]
//         })
//     }
// }

pub struct C3t {
    pub vertices: Vec<C3tVertex>,
    pub indices: Vec<u32>,
    pub texture: Vec<String>
}

impl C3t {
    fn parseNodes(jnode: &BTreeMap<String, Value>) -> Node {
        Node {
            id: jnode.get("id").unwrap().as_string().unwrap().to_string(), 
            skeleton: jnode.get("skeleton").unwrap().as_boolean().unwrap(),
            transform: (json::from_value(jnode.get("transform").unwrap().clone()) ).unwrap(),
            children: match jnode.get("children") {
                Some(children) => {
                    let mut nodes = Vec::<Node>::new();
                    for child in children.as_array().unwrap() {
                        println!("add a child");
                        nodes.push(C3t::parseNodes(child.as_object().unwrap()));
                    }
                    Some(nodes)
                },
                None => {
                    None
                }
            }
        }
    }

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
        // get nodes
        let nodes = data.find("nodes").unwrap();
        let node_array = nodes.as_array().unwrap();
        let node = node_array[1].as_object().unwrap();
        let node_tree = C3t::parseNodes(node);
        // TODO
        C3t{vertices:vertex_array, indices: index_array, texture:vec!["path".to_string()]}
        
    }

}
