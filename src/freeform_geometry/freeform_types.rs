/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * Freeform Types
 *      keyword: cstype
 *      High level attribute for freeform objects
 *      Defines curve/surface metadata
 * 
 * ------------------------------------------------------------------------------------*/

use std::str::FromStr;

use crate::keywords;
use super::basis_matrix::basis_matrix_attributes::BasisMatrixAttributes;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FreeFormTypesErrors {
    InvalidFreeFormType
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FreeFormType {
   BasisMatrix(BasisMatrixAttributes),
   #[default]
   Bezier,
   BSpline,
   Cardinal,
   Taylor
}

impl FromStr for FreeFormType {
    type Err = FreeFormTypesErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            keywords::CURVE_TYPE_BMATRIX  => Ok(FreeFormType::BasisMatrix( BasisMatrixAttributes::default() )),
            keywords::CURVE_TYPE_BEZIER   => Ok(FreeFormType::Bezier),
            keywords::CURVE_TYPE_BSPLINE  => Ok(FreeFormType::BSpline),
            keywords::CURVE_TYPE_CARDINAL => Ok(FreeFormType::Cardinal),
            keywords::CURVE_TYPE_TAYLOR   => Ok(FreeFormType::Taylor),
            _                             => Err(FreeFormTypesErrors::InvalidFreeFormType)
        }
    }
}