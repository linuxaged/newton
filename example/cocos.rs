#![feature(libc, convert)]
//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]
extern crate serde;
use serde::json::{self, Value};

// file io
use std::io::prelude::*;
use std::fs::File;

#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};

fn main() {
    let mut f = File::open("./example/cocos/orc.c3t").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s);

    let data: Value = json::from_str(&s).unwrap();


    // let obj = data.as_object().unwrap();
    let meshes = data.find("meshes").unwrap();
    let mesh_array = meshes.as_array().unwrap();
    let mesh = mesh_array[0].as_object().unwrap();

    // get vertex index
    let parts = mesh.get("parts").unwrap();
    let part_array = parts.as_array().unwrap();
    let part = part_array[0].as_object().unwrap();
    let indices = (json::from_value(part.get("indices").unwrap().clone()) ).unwrap();
    // get vertex
    #[derive(Copy, Clone)]
    struct Vertex {
        position:   [f64; 3],
        normal:     [f64; 3],
        texcood:    [f64; 2],
        blendweight:[f64; 4],
        blendindex: [f64; 4]
    }

    implement_vertex!(Vertex, position, normal, texcood, blendweight, blendindex);
    let mut vertices:Vec<Vertex> = Vec::with_capacity(512);

    vertices = (json::from_value(mesh.get("vertices").unwrap().clone()) ).unwrap();

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, vertices);
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let indices = glium::index::IndexBuffer::new(&indices , glium::index::IndexType::U32, glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t = -0.5;

    loop {
        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0],
            ]
        };

        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}