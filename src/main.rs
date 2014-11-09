#![feature(phase)]
#[phase(plugin)] extern crate bindgen;

#[allow(dead_code, uppercase_variables, non_camel_case_types)]
mod mysql_bindings {
    bindgen!("/usr/include/mysql/mysql.h", match="mysql.h", link="mysql")
}