#![allow(dead_code, unused_variables)]
mod vertexdata;
mod keywords;
mod parser;
mod interpreter;

use parser::ObjParser;
fn main() {
    let test = ObjParser::new("test.obj").unwrap();
    for line in test {
        println!("{line:?}");
    }
}
