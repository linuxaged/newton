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


    let obj = data.as_object().unwrap();
    let meshes = obj.get("meshes").unwrap();
    let mesh_array = meshes.as_array().unwrap();
    let mesh = mesh_array[0].as_object().unwrap();
    // get vertex
    let vertices = mesh.get("vertices").unwrap();
    let vertex_array = vertices.as_array().unwrap();
    // get vertex index
    let parts = mesh.get("parts").unwrap();
    let part_array = parts.as_array().unwrap();
    let part = part_array[0].as_object().unwrap();
    let indices = part.get("indices").unwrap();
    let index_array = indices.as_array().unwrap();

    println!("vertex len = {}", vertex_array.len() );
    println!("index len = {}", index_array.len() );


    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f64; 3],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    // let vertex_buffer = glium::VertexBuffer::new(&display, shape);
    let vertex_buffer = glium::VertexBuffer::new(&display, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    // let indices = glium::index::IndexBuffer::new(&index_array , glium::index::IndexType::U32, glium::index::PrimitiveType::TrianglesList);

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