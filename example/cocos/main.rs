extern crate serialize;
use serialize::json;
use std::io::File;

fn main() {
    let path = Path::new("orc.c3t");
    let raw_string = File::open(&path).read_to_string().unwrap();

    match json::from_str(raw_string) {
        Ok(json) => println!("{}", json.find("vertion").unwrap()),
        Err(err) => println!("{}", err),
    }
}