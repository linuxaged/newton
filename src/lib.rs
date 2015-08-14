#![feature(libc, convert, custom_derive, plugin, step_by, core_simd, static_recursion)]
#![plugin(serde_macros)]
extern crate serde;
#[macro_use]
extern crate glium;
extern crate image;
pub mod cocos;
pub mod math;