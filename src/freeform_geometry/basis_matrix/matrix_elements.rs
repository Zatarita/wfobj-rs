use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixElements ( Vec<f32> );

impl MatrixElements {
    // Vec deque doesn't support slicing, deque was used for the pop-front and ease of use with keyword parsing. 
    pub fn from_deque(elements: &VecDeque<f32>) -> MatrixElements {
        let converted_vector: Vec<f32> = elements.into_iter().map(|e| e.to_owned()).collect();
        MatrixElements::new(&converted_vector)
    }

    pub fn new(elements: &[f32]) -> MatrixElements {
        MatrixElements(elements.to_vec())
    }

    pub fn empty(degree: usize) -> MatrixElements {
        MatrixElements(vec![0.0; degree.pow(2)])
    }

    pub fn get_column<'a>(&'a self, column: usize, degree: usize) -> Option<MatrixColumn<'a>> {
        if column == 0 || column > degree + 1 {
            return None;
        }

        let matrix_size = (degree + 1).pow(2);

        if self.len() == matrix_size {
            let mut ret = Vec::<&'a f32>::new();
            ret.reserve(degree + 1);

            for i in ((column - 1)..matrix_size).step_by(degree + 1) {
                ret.push(&self.0[i])
            }

            return Some(ret);
        }
        None
    }

    pub fn get_row<'a>(&'a self, row: usize, degree: usize) -> Option<MatrixRow<'a>> {
        if row == 0 || row > degree + 1 {
            return None;
        }

        let matrix_size = (degree + 1).pow(2);

        if self.len() == matrix_size {
            let mut ret = Vec::<&'a f32>::new();
            ret.reserve(degree + 1);

            let starting_index = (row - 1) * (degree + 1);

            for i in starting_index..starting_index + degree + 1 {
                ret.push(&self.0[i])
            }

            return Some(ret);
        }
        None
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub type MatrixRow<'a>    = Vec<&'a f32>;
pub type MatrixColumn<'a> = Vec<&'a f32>;