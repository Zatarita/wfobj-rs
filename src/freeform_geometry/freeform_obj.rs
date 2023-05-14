use std::collections::VecDeque;
use std::str::FromStr;
use std::todo;

/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * Freeform Curve & Surface
 *  This is the exposed final curve/surface object that pulls everything together
 * 
 * ------------------------------------------------------------------------------------*/
use super::freeform_types::FreeFormType;
use super::Degree;
use super::uv_pair::UVPairError;

use crate::freeform_geometry::basis_matrix::basis_matrix_attributes::{BasisMatrixAttributes, BasisMatrixAttributesError};
use crate::parser::ObjParser;
use crate::keywords;
use crate::utility;

pub enum FreeFormObjectError {
    MissingKeyword,
    CurveSurfaceMismatch,
    InvalidKeyword,
    InvalidFormType,
    InvalidParameters,
    InvalidFreeFormType,
    InvalidDegreeBufferSize
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreeformObject {
    form_type:  FreeFormType,
    rational:   bool,
    degree:     Degree,
}

impl FreeformObject {
    pub fn parse(parser: &mut ObjParser) -> Result<FreeformObject, FreeFormObjectError> {
        let mut ret = FreeformObject::default();

        while let Some(line) = parser.get_line() {
            let keyword = line.keyword.ok_or(FreeFormObjectError::MissingKeyword)?;
            match keyword.as_str() {
                keywords::CURVE_SURFACE_TYPE => ret.parse_curve_surface_type(&line.parameters)?,
                keywords::DEGREE             => ret.parse_degree(&line.parameters)?,
                keywords::BASIS_MATRIX_U     => ret.parse_matrix_u(&line.parameters)?,
                keywords::BASIS_MATRIX_V     => ret.parse_matrix_v(&line.parameters)?,
                _                            => return Err(FreeFormObjectError::InvalidKeyword)
            }
        }
        Ok(ret)
    }

    fn parse_degree(&mut self, parameters: &VecDeque<String>) -> Result<(), FreeFormObjectError> {
        match Degree::from(parameters) {
            Ok(value)                   => self.degree = value,
            Err(UVPairError::InvalidBufferSize) => return Err(FreeFormObjectError::InvalidDegreeBufferSize),
            Err(UVPairError::InvalidParameters) => return Err(FreeFormObjectError::InvalidParameters)
        }
        Ok(())
    }

    #[allow(unused_variables, unreachable_patterns)]    // Why is this marked unreachable & unused? Both aren't true
    fn parse_curve_surface_type(&mut self, parameters: &VecDeque<String>) -> Result<(),FreeFormObjectError> {
        let valid_curves = keywords::VALID_CURVE_TYPES.as_slice();

        for parameter in parameters {
            match parameter.as_str() {
                keywords::RATIONAL_CURVE => self.rational = true,
                valid_curves       => {
                    if let Ok(free_form_type) = FreeFormType::from_str(parameter) {
                        self.form_type = free_form_type
                    } else {
                        return Err(FreeFormObjectError::InvalidFreeFormType)
                    }
                }
                _ => return Err(FreeFormObjectError::InvalidParameters)
            }
        }
        Ok(())
    }

    fn parse_matrix_u(&mut self, parameters: &VecDeque<String>) -> Result<(), FreeFormObjectError> {
        let converted_parameters = utility::convert_to_f32(parameters).ok().ok_or(FreeFormObjectError::InvalidParameters)?;
        let mut new_attributes: BasisMatrixAttributes;

        // If we're the right type, get the attributes
        if let FreeFormType::BasisMatrix(attributes) = &self.form_type {
            new_attributes = attributes.clone();
        } else {
            return Err(FreeFormObjectError::InvalidFormType);
        }

        // Matrices can have references to their elements. These references live as long as the matrix
        // To ensure that the references point to the proper place, we must create a new matrix
        // to invalidate any existing references. That and rust complains if I dont. c:
        new_attributes.set_matrix_u(&converted_parameters);
        self.form_type = FreeFormType::BasisMatrix(new_attributes);

        Ok(())
    }

    fn parse_matrix_v(&mut self, parameters: &VecDeque<String>) -> Result<(),FreeFormObjectError> {
        let converted_parameters = utility::convert_to_f32(parameters).ok().ok_or(FreeFormObjectError::InvalidParameters)?;
        let mut new_attributes: BasisMatrixAttributes;

        // If we're the right type, get the attributes
        if let FreeFormType::BasisMatrix(attributes) = &self.form_type {
            new_attributes = attributes.clone();
        } else {
            return Err(FreeFormObjectError::InvalidFormType);
        }

        // Matrix v is only used for curves, there is a chance that an error is thrown
        if let Err(BasisMatrixAttributesError::InvalidMatrixType) = new_attributes.set_matrix_v(&converted_parameters) {
            return Err(FreeFormObjectError::CurveSurfaceMismatch);
        }
        self.form_type = FreeFormType::BasisMatrix(new_attributes);

        Ok(())
    }
}