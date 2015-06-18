//#![feature(custom_derive, plugin)]
//#![plugin(serde_macros)]
extern crate serde;

use serde::json::{self, Value};

fn main() {
    let data: Value = json::from_str("{\"foo\": 13, \"bar\": \"baz\"}").unwrap();
    println!("data: {:?}", data);
    // data: {"bar":"baz","foo":13}
    println!("object? {}", data.is_object());
    // object? true

    let obj = data.as_object().unwrap();
    let foo = obj.get("foo").unwrap();

    println!("array? {:?}", foo.as_array());
    // array? None
    println!("u64? {:?}", foo.as_u64());
    // u64? Some(13u64)

    for (key, value) in obj.iter() {
        println!("{}: {}", key, match *value {
            Value::U64(v) => format!("{} (u64)", v),
            Value::String(ref v) => format!("{} (string)", v),
            _ => format!("other")
        });
    }
    // bar: baz (string)
    // foo: 13 (u64)
}