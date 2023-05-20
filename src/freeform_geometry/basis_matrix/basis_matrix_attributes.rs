/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * Basis Matrix Attributes
 *      Basis matrices have extra attribute data
 *      this defines the Step and Matrix that define the curve/surface
 * 
 * ------------------------------------------------------------------------------------*/

use crate::freeform_geometry::Degree;
use super::Step;
use super::matrix::{Matrix, MatrixError};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BasisMatrixAttributesError {
    InvalidMatrixType,
    InvalidBufferSize,
    FreeFormTypeMismatch,
    MatrixSizeMismatch,
    UnknownError
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BasisMatrixAttributes {
    pub step: Step,
    pub matrix: Matrix
}

impl BasisMatrixAttributes {
    pub fn new(step: Step, matrix: Matrix) -> BasisMatrixAttributes {
        BasisMatrixAttributes {step, matrix}
    }

    pub fn set_matrix(&mut self, new_matrix: Matrix) {
        self.matrix = new_matrix;
    }

    pub fn set_matrix_u(&mut self, elements: &[f32]) {
        match &self.matrix {
            Matrix::Curve(_)                       => self.matrix = Matrix::new_curve(elements),
            Matrix::Surface(_, v) => self.matrix = Matrix::new_surface(elements, v.as_slice())
        }
    }

    pub fn set_matrix_v(&mut self, elements: &[f32]) {
        match &self.matrix {
            Matrix::Curve(u)      => self.matrix = Matrix::new_surface(u.as_slice(), elements),
            Matrix::Surface(u, _) => self.matrix = Matrix::new_surface(u.as_slice(), elements)
        }
    }

    pub fn set_step(&mut self, elements: &[usize]) -> Result<(), BasisMatrixAttributesError> {
        match elements.len() {
            1      => {
                self.step = Step::new_curve(elements).ok().ok_or(BasisMatrixAttributesError::InvalidBufferSize)?;
            },
            2 => {
                self.step = Step::new_surface(elements).ok().ok_or(BasisMatrixAttributesError::InvalidBufferSize)?;
            }
            _ => return Err(BasisMatrixAttributesError::InvalidBufferSize)
        }
        Ok(())
    }

    // Make sure the matrix/step matches curve/surface type & The matrix size matches degree
    pub fn validate(&self, degree: &Degree) -> Result<(), BasisMatrixAttributesError> {
        if let Err(MatrixError::InvalidDegree) = self.matrix.validate_matrix(degree) {
            return Err(BasisMatrixAttributesError::MatrixSizeMismatch);
        } 

        if self.step.is_curve() && self.matrix.is_curve() {
            Ok(())
        } else if self.step.is_surface() && self.matrix.is_surface() {
            Ok(())
        } else {
            Err(BasisMatrixAttributesError::FreeFormTypeMismatch)
        }
    }
}