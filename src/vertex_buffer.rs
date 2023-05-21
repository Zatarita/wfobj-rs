/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/11/2023)
 * 
 * OBJ Interpreter
 *  Responsible for aggregating vertex data objects
 * 
 * ------------------------------------------------------------------------------------*/

use std::rc::Rc;
use std::collections::VecDeque;

use crate::vertex_data::*;
use crate::index::Index;

type BufferObject<T> = Vec<Rc<T>>;

#[derive(Debug)]
pub enum VertexBufferError {
    InterpreterError,
    BoundsException
}

#[derive(Debug, Clone, Default)]
pub struct VertexBuffer {
    vertices:                 BufferObject<Vertex>,
    texture_coordinates:      BufferObject<TextureCoordinate>,
    normals:                  BufferObject<VertexNormal>,
    parameter_space_vertices: BufferObject<ParameterSpaceVertex>
}

impl VertexBuffer {
    pub fn new() -> VertexBuffer {
        VertexBuffer { vertices: vec![], texture_coordinates: vec![], normals: vec![], parameter_space_vertices: vec![] }
    }

    pub fn add_vertex(&mut self, new_vertex: Vertex) {
        self.vertices.push(Rc::new(new_vertex));
    }
    pub fn add_texture_coordinate(&mut self, new_coordinate: TextureCoordinate) {
        self.texture_coordinates.push(Rc::new(new_coordinate));
    }
    pub fn add_normal(&mut self, new_coordinate: VertexNormal) {
        self.normals.push(Rc::new(new_coordinate));
    }
    pub fn add_parameter_space_vertex(&mut self, new_coordinate: ParameterSpaceVertex) {
        self.parameter_space_vertices.push(Rc::new(new_coordinate));
    }

    pub fn create_vertex(&mut self, new_vertex_data: &VecDeque<String>) -> Result<(), VertexBufferError> {
        if let Some(vertex) = Vertex::from(new_vertex_data) {
            self.vertices.push(Rc::new(vertex));
            return Ok(());
        }
        Err(VertexBufferError::InterpreterError)
    }
    pub fn create_texture_coordinate(&mut self, new_coordinate_data: &VecDeque<String>) -> Result<(), VertexBufferError> {
        if let Some(texture_coordinate) = TextureCoordinate::from(new_coordinate_data) {
            self.texture_coordinates.push(Rc::new(texture_coordinate));
            return Ok(());
        }
        Err(VertexBufferError::InterpreterError)
    }
    pub fn create_normal(&mut self, new_normal_data: &VecDeque<String>) -> Result<(), VertexBufferError> {
        if let Some(normal) = VertexNormal::from(new_normal_data) {
            self.normals.push(Rc::new(normal));
            return Ok(());
        }
        Err(VertexBufferError::InterpreterError)
    }
    pub fn create_parameter_space_vertex(&mut self, new_parameter_space_vertex_data: &VecDeque<String>) -> Result<(), VertexBufferError> {
        if let Some(parameter_space_vertex) = ParameterSpaceVertex::from(new_parameter_space_vertex_data) {
            self.parameter_space_vertices.push(Rc::new(parameter_space_vertex));
            return Ok(());
        }
        Err(VertexBufferError::InterpreterError)
    }

    pub fn get_vertex(&self, index: Index) -> Result<Rc<Vertex>, VertexBufferError> {
        VertexBuffer::get_from_buffer(&self.vertices, index)       
    }
    pub fn get_texture_coordinate(&self, index: Index) -> Result<Rc<TextureCoordinate>, VertexBufferError> {
        VertexBuffer::get_from_buffer(&self.texture_coordinates, index)       
    }
    pub fn get_normal(&self, index: Index) -> Result<Rc<VertexNormal>, VertexBufferError> {
        VertexBuffer::get_from_buffer(&self.normals, index)       
    }
    pub fn get_paramter_space_vertex(&self, index: Index) -> Result<Rc<ParameterSpaceVertex>, VertexBufferError> {
        VertexBuffer::get_from_buffer(&self.parameter_space_vertices, index)       
    }

    fn get_from_buffer<ContentType>(buffer: &BufferObject<ContentType>, index: Index) -> Result<Rc<ContentType>, VertexBufferError> {
        let index = index.as_isize();
        if index < 0 {
            // If negative, the index is relative to the end of the buffer
            let index = index.abs() as usize;
            if let Some(value) = buffer.get(buffer.len() - index) {
                Ok(value.clone())
            } else {
                Err(VertexBufferError::BoundsException)
            }
        } else {
            // If positive, absolute position
            if let Some(value) = buffer.get(index as usize) {
                Ok(value.clone())
            } else {
                Err(VertexBufferError::BoundsException)
            }
        }
    }
}