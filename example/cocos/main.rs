//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]
extern crate serde;

// file io
use std::io::prelude::*;
use std::fs::File;

use serde::json::{self, Value};

fn main() {
    let mut f = File::open("./example/cocos/orc.c3t").unwrap(); // relative path to target
    let mut s = String::new();
    f.read_to_string(&mut s);

    let data: Value = json::from_str(&s).unwrap();


    let obj = data.as_object().unwrap();
    let meshes = obj.get("meshes").unwrap();
    let mesh_array = meshes.as_array().unwrap();
    let mesh = mesh_array[0].as_object().unwrap();
    let vertices = mesh.get("vertices").unwrap();
    let vertex_array = vertices.as_array();

    println!("array? {:?}", vertex_array);

    // // array? None
    // println!("u64? {:?}", foo.as_u64());
    // // u64? Some(13u64)

    // for (key, value) in obj.iter() {
    //     println!("{}: {}", key, match *value {
    //         Value::U64(v) => format!("{} (u64)", v),
    //         Value::String(ref v) => format!("{} (string)", v),
    //         _ => format!("other")
    //     });
    // }
    // bar: baz (string)
    // foo: 13 (u64)
}