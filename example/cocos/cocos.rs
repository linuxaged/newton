extern crate cgmath;
use cgmath::FixedArray;

extern crate newton;
use newton::cocos::c3t;
#[macro_use]
extern crate glium;
use glium::{DisplayBuild, Surface};
extern crate image;
use std::io::Cursor;
use std::path::Path;

fn main() {
    let c3t = c3t::C3t::new(Path::new("./example/cocos/orc.c3t"));

    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, c3t.vertices);
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          c3t.indices);
    // load texture
    let image = image::load(Cursor::new(&include_bytes!("./cocos/monguger.tga")[..]),
                            image::ImageFormat::TGA).unwrap();
    let texture = glium::texture::Texture2d::new(&display, image);

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec2 texcoord;
        out vec2 v_tex_coords;

        uniform mat4 perspective_matrix;
        uniform mat4 view_matrix;
        uniform mat4 model_matrix;

        void main() {
            v_tex_coords = texcoord;
            gl_Position = perspective_matrix * view_matrix * model_matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let perspective_matrix: cgmath::Matrix4<f32> = cgmath::perspective(cgmath::deg(45.0), 1.333, 0.0001, 100.0);
    let fixed_perspective_matrix = perspective_matrix.as_fixed();
    let view_eye: cgmath::Point3<f32> = cgmath::Point3::new(0.0, 40.0, -5.0);
    let view_center: cgmath::Point3<f32> = cgmath::Point3::new(0.0, 0.0, 0.0);
    let view_up: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 1.0, 0.0);
    let view_matrix: cgmath::Matrix4<f32> = cgmath::Matrix4::look_at(&view_eye, &view_center, &view_up);
    let fixed_view_matrix = view_matrix.as_fixed();
    let model_matrix: cgmath::Matrix4<f32> = cgmath::Matrix4::identity();
    let fixed_model_matrix = model_matrix.as_fixed();

    loop {
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 0.0);

        let uniforms = uniform! {
            perspective_matrix: *fixed_perspective_matrix,
            view_matrix: *fixed_view_matrix,
            model_matrix: *fixed_model_matrix,
            tex: &texture
        };

        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms,
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