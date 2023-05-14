use crate::freeform_geometry::Degree;

pub struct MatrixElements ( Vec<f32> );

impl MatrixElements {
    pub fn new(degree: usize) -> MatrixElements {
        MatrixElements(vec![0.0; degree.pow(2)])
    }

    pub fn get_row<'a>(&'a self, row: usize, degree: &Degree) -> Option<MatrixRow<'a>> {
        None
    }
}

pub type MatrixRow<'a>    = Vec<&'a f32>;
pub type MatrixColumn<'a> = Vec<&'a f32>;