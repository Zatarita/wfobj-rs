/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * Freeform Curve & Surface
 *  This is the exposed final curve/surface object that pulls everything together
 * 
 * ------------------------------------------------------------------------------------*/
use std::str::FromStr;
use std::collections::VecDeque;

use super::freeform_types::FreeFormType;
use super::Degree;
use super::uv_pair::UVPairError;

use crate::freeform_geometry::basis_matrix::basis_matrix_attributes::{BasisMatrixAttributes, BasisMatrixAttributesError};
use crate::parser::ObjParser;
use crate::keywords;
use crate::utility;

#[derive(Debug)]
pub enum FreeFormObjectError {
    MissingKeyword,
    CurveSurfaceMismatch,
    InvalidKeyword,
    InvalidFormType,
    InvalidParameters,
    InvalidFreeFormType,
    InvalidBufferSize,
    MalformedCurveDefinition,
    UnknownError
}

#[derive(Debug)]
pub enum FreeFormValidationExceptions {

}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreeFormObject {
    pub form_type:  FreeFormType,
    pub rational:   bool,
    pub degree:     Degree,
}

impl FreeFormObject {
    pub fn new(form_type: FreeFormType, rational: bool, degree: Degree) -> FreeFormObject {
        FreeFormObject{form_type, rational, degree}
    }

    #[allow(unused_assignments)]
    pub fn parse(parser: &mut ObjParser) -> Result<FreeFormObject, FreeFormObjectError> {
        let mut free_form_type: Option<FreeFormType>      = None;    // Required
        let mut degree: Option<Degree>                    = None;    // Required
        let mut rational: Option<bool>                    = None;    // Not Required

        while let Some(mut line) = parser.peek_line() {
            if let Some(keyword) = line.keyword {
                match keyword.as_str() {
                    keywords::CURVE_SURFACE_TYPE => (free_form_type, rational) = FreeFormObject::parse_curve_surface_type(&line.parameters)?,
                    keywords::DEGREE             => degree = FreeFormObject::parse_degree(&line.parameters)?,
                    keywords::BASIS_MATRIX       => {
                        if let Some(form_type) = free_form_type {
                            free_form_type = Some(FreeFormObject::parse_matrix(&form_type, &mut line.parameters)?);
                        } else {
                            return Err(FreeFormObjectError::InvalidFormType);
                        }
                    }
                    keywords::STEP_SIZE          => {
                        if let Some(form_type) = free_form_type {
                            free_form_type = Some(FreeFormObject::parse_matrix_step(&form_type, &mut line.parameters)?);
                        } else {
                            return Err(FreeFormObjectError::InvalidFormType);
                        }
                    }
                    _                            => break
                }
            }
            parser.skip_line();
        }

        if let (Some(form_type), Some(degree)) = (free_form_type, degree) {
            Ok(FreeFormObject { form_type, rational: rational.unwrap_or(false), degree })
        } else {
            Err(FreeFormObjectError::MalformedCurveDefinition)
        }

    }

    #[allow(unused_variables, unreachable_patterns)]    // Why is this marked unreachable & unused? Both aren't true
    fn parse_curve_surface_type(parameters: &VecDeque<String>) -> Result<(Option<FreeFormType>, Option<bool>),FreeFormObjectError> {
        let valid_curves: &[&str] = keywords::VALID_CURVE_TYPES.as_slice();
        
        let mut free_form_type: Option<FreeFormType> = None;
        let mut rational:       Option<bool>         = None;

        for parameter in parameters {
            match parameter.as_str() {
                keywords::RATIONAL_CURVE => rational = Some(true),
                valid_curves       => {
                    if let Ok(new_free_form_type) = FreeFormType::from_str(parameter) {
                        free_form_type = Some(new_free_form_type);
                    } else {
                        return Err(FreeFormObjectError::InvalidFreeFormType)
                    }
                }
                _ => return Err(FreeFormObjectError::InvalidParameters)
            }
        }
        Ok((free_form_type, rational))
    }

    fn parse_degree(parameters: &VecDeque<String>) -> Result<Option<Degree>, FreeFormObjectError> {
        match Degree::from(parameters) {
            Ok(value)                   => Ok(Some(value)),
            Err(UVPairError::InvalidBufferSize) => Err(FreeFormObjectError::InvalidBufferSize),
            Err(UVPairError::InvalidParameters) => Err(FreeFormObjectError::InvalidParameters)
        }
    }

    fn parse_matrix(current_type: &FreeFormType, parameters: &mut VecDeque<String>)  -> Result<FreeFormType, FreeFormObjectError> {
        let matrix_axis = parameters.pop_front().ok_or(FreeFormObjectError::InvalidParameters)?;
        match matrix_axis.as_str() {
            keywords::BASIS_MATRIX_U => Ok(FreeFormObject::parse_matrix_u(current_type, parameters)?),
            keywords::BASIS_MATRIX_V => Ok(FreeFormObject::parse_matrix_v(current_type, parameters)?),
            _ => Err(FreeFormObjectError::InvalidKeyword)
        }
    }

    fn parse_matrix_u(current_type: &FreeFormType, parameters: &VecDeque<String>) -> Result<FreeFormType, FreeFormObjectError> {
        let converted_parameters = utility::convert_vec::<f32>(parameters).ok().ok_or(FreeFormObjectError::InvalidParameters)?;
        let mut new_attributes: BasisMatrixAttributes;

        // If we're the right type, get the attributes
        if let FreeFormType::BasisMatrix(attributes) = &current_type {
            new_attributes = attributes.clone();
        } else {
            return Err(FreeFormObjectError::InvalidFormType);
        }

        // Matrices can have references to their elements. These references live as long as the matrix
        // To ensure that the references point to the proper place, we must create a new matrix
        // to invalidate any existing references. That and rust complains if I dont. c:
        new_attributes.set_matrix_u(&converted_parameters);

        Ok(FreeFormType::BasisMatrix(new_attributes))
    }

    fn parse_matrix_v(current_type: &FreeFormType, parameters: &VecDeque<String>) -> Result<FreeFormType,FreeFormObjectError> {
        let mut new_attributes: BasisMatrixAttributes;
        if let FreeFormType::BasisMatrix(attributes) = current_type {
            new_attributes = attributes.clone();
        } else {
            return Err(FreeFormObjectError::InvalidFormType);
        }

        let converted_parameters = utility::convert_vec::<f32>(parameters).ok().ok_or(FreeFormObjectError::InvalidParameters)?;
        // If matrix is a curve, this will cause an upgrade to surface
        new_attributes.set_matrix_v(&converted_parameters);

        Ok(FreeFormType::BasisMatrix(new_attributes))
    }

    fn parse_matrix_step(current_type: &FreeFormType, parameters: &VecDeque<String>) -> Result<FreeFormType, FreeFormObjectError> {
        let mut new_attributes: BasisMatrixAttributes;
        // If we're the right type, get the attributes
        if let FreeFormType::BasisMatrix(attributes) = &current_type {
            new_attributes = attributes.clone();
        } else {
            return Err(FreeFormObjectError::InvalidFormType);
        }

        let converted_parameters = utility::convert_vec::<usize>(parameters).ok().ok_or(FreeFormObjectError::InvalidParameters)?;
        if let Err(BasisMatrixAttributesError::InvalidBufferSize) = new_attributes.set_step(&converted_parameters) {
            return Err(FreeFormObjectError::InvalidBufferSize);
        }

        Ok(FreeFormType::BasisMatrix(new_attributes))
    }
}