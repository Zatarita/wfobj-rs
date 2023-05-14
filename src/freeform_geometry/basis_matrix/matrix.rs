/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * Basis Matrix
 *      Holds the defining matrix for the basis matrix type
 *      Sometimes referred to as the characteristic matrix.
 *      
 *      The matrix size is determined by the degree of the curve/surface.
 *      The matrix must have a U component at all times.
 *      The matrix must have a V component when defining a surface
 * 
 * ------------------------------------------------------------------------------------*/

use std::collections::VecDeque;

use crate::freeform_geometry::Degree;
use super::matrix_elements::{MatrixElements, MatrixRow, MatrixColumn};

pub enum MatrixError {
    InvalidDegree
}

#[derive(Debug, Clone, PartialEq)]
pub enum Matrix {
    Curve  (MatrixElements),
    Surface(MatrixElements, MatrixElements)
}

impl Matrix {
    pub fn new_curve(u: &VecDeque<f32>) -> Matrix {
        Matrix::Curve( 
            MatrixElements::from_deque(u) 
        )
    }

    pub fn new_surface(u: &VecDeque<f32>, v: &VecDeque<f32>) -> Matrix {
        Matrix::Surface( 
            MatrixElements::from_deque(u),
            MatrixElements::from_deque(v),
        )
    }

    pub fn is_curve(&self) -> bool {
        match self {
            Matrix::Curve(_) => true,
            _                => false
        }
    }

    pub fn is_surface(&self) -> bool {
        match self {
            Matrix::Surface(_, _) => true,
            _                     => false
        }
    }

    pub fn get_row(&self, row: usize, degree: &Degree) -> Option<Vec<MatrixRow>> {
        if let Err(_) = self.validate_matrix(degree) {
            return None;
        }
        
        match self {
            // Get row from u matrix
            Matrix::Curve(u) => {
                let row = u.get_row(row, degree.u().to_owned())?;
                Some(vec![row])
            }
            // Get row from u,v matrix
            Matrix::Surface(u, v) => {
                let row_1 = u.get_row(row, degree.u().to_owned() )?;
                let row_2 = v.get_row(row, degree.v()?.to_owned())?;
                Some(vec![row_1, row_2])
            }
        }
    }

    pub fn get_column(&self, column: usize, degree: &Degree) -> Option<Vec<MatrixColumn>> {
        if let Err(_) = self.validate_matrix(degree) {
            return None;
        }
        
        match self {
            // Get column from u matrix
            Matrix::Curve(u) => {
                let column = u.get_column(column, degree.u().to_owned())?;
                Some(vec![column])
            }
            // Get column from u,v matrix
            Matrix::Surface(u, v) => {
                let column_1 = u.get_column(column, degree.u().to_owned() )?;
                let column_2 = v.get_column(column, degree.v()?.to_owned())?;
                Some(vec![column_1, column_2])
            }
        }
    }

    pub fn validate_matrix(&self, degree: &Degree) -> Result<(), MatrixError> {
        // Verify the degree matches the matrix type
        if ( self.is_curve() && degree.is_surface() ) || ( self.is_surface() && degree.is_curve() ) {
            return Err(MatrixError::InvalidDegree);
        } 

        // Verify the matrix size is correct for the degree.
        // Required to be (degree + 1) x (degree + 1)
        // V only required if matrix is a surface
        match self {
            Matrix::Curve  (u) => {
                if u.len() == (degree.u() + 1).pow(2) { 
                    return Ok(()); 
                }
            }
            Matrix::Surface(u, v) => {
                if u.len() != (degree.u() + 1).pow(2) { 
                    return Err(MatrixError::InvalidDegree);
                }

                if let Some(degree_v) = degree.v() {
                    if v.len() == (degree_v + 1).pow(2) { 
                        return Ok(()); 
                    }
                }
            }
        };

        Err(MatrixError::InvalidDegree)
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Matrix::Curve(MatrixElements::empty(3))
    }
}