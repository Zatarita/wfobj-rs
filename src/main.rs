#![allow(dead_code, unused_variables)]
mod vertex_data;
mod keywords;
mod parser;
mod interpreter;
mod vertex_buffer;
mod index;

use vertex_buffer::VertexBuffer;
use vertex_data::Vertex;
fn main() {
    let mut x = index::Index::new(3).unwrap();

    let negx = index::Index::new(-1).unwrap();
    let mut test = VertexBuffer::new();
    test.add_vertex(Vertex::new());
    test.add_vertex(Vertex::new());

    let vertex: Vertex = Vertex {x: 1.0, y: 2.0 , z: 3.0, w:1.0 };
    test.add_vertex(vertex);

    let retrieved_vert = test.get_vertex(x).unwrap();
    let retrieved_vert = test.get_vertex(negx).unwrap();

    assert_eq!(test.get_vertex(x).unwrap(), test.get_vertex(negx).unwrap());
}
