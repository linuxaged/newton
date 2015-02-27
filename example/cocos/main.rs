use std::io::prelude::*;
use std::fs::File;
extern crate serialize;
use serialize::json;

extern crate newton;
use newton::cocos::c3t;
use newton::math::vector3;

extern crate glutin;
extern crate libc;
extern crate gl;

fn get_vert_idx() {
    let mut f = File::open("orc.c3t").unwrap();
	let mut s = String::new();
	f.read_to_string(&mut s);

	match json::from_str(&s) {
        // Ok(json) => println!("{:?}", json.as_object().unwrap().get("meshes")
        // 	.unwrap().as_array().unwrap()[0].as_object()
        // 	.unwrap().get("vertices").unwrap().as_array().unwrap()),
		Ok(json) => println!("{:?}", json.as_object().unwrap().get("meshes")
        	.unwrap().as_array().unwrap()[0].as_object()
        	.unwrap().get("parts").unwrap().as_array().unwrap()[0].as_object().unwrap().get("indices").unwrap().as_array().unwrap()),
        Err(err) => println!("{}", err),
    }
}

fn main() {
	get_vert_idx();

	let window = glutin::Window::new().unwrap();

    unsafe { window.make_current() };

    unsafe {
        gl::load_with(|symbol| window.get_proc_address(symbol));

        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    while !window.is_closed() {
        window.wait_events();

        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };

        window.swap_buffers();
    }
    
}