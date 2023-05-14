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
    FreeFormTypeMismatch,
    MatrixSizeMismatch,
    UnknownError
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BasisMatrixAttributes {
    step: Step,
    matrix: Matrix
}

impl BasisMatrixAttributes {
    pub fn new(step: Step, matrix: Matrix) -> BasisMatrixAttributes {
        BasisMatrixAttributes {step, matrix}
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