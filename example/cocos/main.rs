extern crate serialize;
use serialize::json;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut raw_string = String::new();
    File::open("orc.c3t").unwrap().read_to_string(&mut raw_string);

    match json::from_str(&raw_string) {
        Ok(json) => println!("{}", json.find("version").unwrap()),
        Err(err) => println!("{}", err),
    }
}