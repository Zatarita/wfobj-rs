/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * Matrix Elements
 *      Wrapper for Vector of floats
 *      Handles getting references to the elements as rows and columns
 * 
 * ------------------------------------------------------------------------------------*/

#[derive(Debug, Clone, PartialEq)]
pub struct MatrixElements ( Vec<f32> );

pub type MatrixRow<'a>    = Vec<&'a f32>;
pub type MatrixColumn<'a> = Vec<&'a f32>;

impl MatrixElements {
    pub fn new(elements: &[f32]) -> MatrixElements {
        MatrixElements(elements.to_vec())
    }

    pub fn empty(degree: usize) -> MatrixElements {
        MatrixElements(vec![0.0; degree.pow(2)])
    }

    pub fn as_slice(&self) -> &[f32] {
        &self.0
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