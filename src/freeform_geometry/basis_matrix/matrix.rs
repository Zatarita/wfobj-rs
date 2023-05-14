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

use crate::freeform_geometry::Degree;

pub enum MatrixError {
    InvalidDegree
}

#[derive(Debug, Clone, PartialEq)]
pub enum Matrix {
    Curve  (Vec<f32>),
    Surface(Vec<f32>, Vec<f32>)
}

impl Matrix {
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
        Matrix::Curve(vec![])
    }
}