use parser::ObjParser;

mod vertexdata;
#[allow(unused_assignments, dead_code)]
mod keywords;
mod parser;

fn main() {
    let test = ObjParser::new("test.obj").unwrap();
    for line in test {
        println!("{line:?}");
    }
}
