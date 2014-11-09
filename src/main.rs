#![feature(phase)]
#[phase(plugin)] extern crate bindgen;

#[allow(dead_code, uppercase_variables, non_camel_case_types)]
mod gl_bindings {
    bindgen!("/System/Library/Frameworks/OpenGL.framework/Headers/gl.h", match="gl.h", link_framework="OpenGL")
}