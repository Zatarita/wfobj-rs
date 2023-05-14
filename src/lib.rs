#![allow(dead_code)]

mod vertex_data;
mod keywords;
mod parser;
mod interpreter;
mod vertex_buffer;
mod index;
mod freeform_geometry;
mod utility;


#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::{println, vec};

    use super::*;
    
    use freeform_geometry::basis_matrix::matrix_elements::MatrixElements;

    #[test]
    fn test_matrix() {
        let test: VecDeque<f32> = vec![1.0, 1.0, 1.0, 1.0, 2.0, 4.0, 2.0, 2.0, 3.0, 8.0, 3.0, 3.0, 4.0, 16.0, 4.0, 4.0].into();
        let x = MatrixElements::from_deque(&test);

        let x_row = x.get_row(4, 3);
        let x_column: Option<Vec<&f32>> = x.get_column(2, 3);
        println!("{x_row:?}");
        println!("{x_column:?}");
    }
}