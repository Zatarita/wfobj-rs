/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * Freeform Curve & Surface
 *  This is the exposed final curve/surface definition
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FreeFormDefinitionError {
    MissingKeyword,
    CurveSurfaceMismatch,
    InvalidKeyword,
    InvalidFormType,
    InvalidParameters,
    InvalidFreeFormType,
    InvalidBufferSize,
    MalformedDefinition,
    UnknownError
}

#[derive(Debug, PartialEq)]
pub enum FreeFormValidationExceptions {
    CardinalDegreeNotEqualToThree,
    CurveSurfaceMismatch,
    InvalidMatrixSize,
    UnknownException
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreeFormDefinition {
    pub form_type:  FreeFormType,
    pub rational:   bool,
    pub degree:     Degree,
}

impl FreeFormDefinition {
    pub fn new(form_type: FreeFormType, rational: bool, degree: Degree) -> FreeFormDefinition {
        FreeFormDefinition{form_type, rational, degree}
    }

    #[allow(unused_assignments)]
    pub fn parse(parser: &mut ObjParser) -> Result<FreeFormDefinition, FreeFormDefinitionError> {
        let mut free_form_type: Option<FreeFormType>      = None;    // Required
        let mut degree: Option<Degree>                    = None;    // Required
        let mut rational: Option<bool>                    = None;    // Not Required

        while let Some(mut line) = parser.peek_line() {
            if let Some(keyword) = line.keyword {
                match keyword.as_str() {
                    keywords::CURVE_SURFACE_TYPE => (free_form_type, rational) = FreeFormDefinition::parse_curve_surface_type(&line.parameters)?,
                    keywords::DEGREE             => degree = FreeFormDefinition::parse_degree(&line.parameters)?,
                    keywords::BASIS_MATRIX       => {
                        if let Some(form_type) = free_form_type {
                            free_form_type = Some(FreeFormDefinition::parse_matrix(&form_type, &mut line.parameters)?);
                        } else {
                            return Err(FreeFormDefinitionError::InvalidFormType);
                        }
                    }
                    keywords::STEP_SIZE          => {
                        if let Some(form_type) = free_form_type {
                            free_form_type = Some(FreeFormDefinition::parse_matrix_step(&form_type, &mut line.parameters)?);
                        } else {
                            return Err(FreeFormDefinitionError::InvalidFormType);
                        }
                    }
                    _                            => break
                }
            }
            parser.skip_line();
        }

        if let (Some(form_type), Some(degree)) = (free_form_type, degree) {
            Ok(FreeFormDefinition { form_type, rational: rational.unwrap_or(false), degree })
        } else {
            Err(FreeFormDefinitionError::MalformedDefinition)
        }

    }

    pub fn validate(&self) -> Result<(), FreeFormValidationExceptions> {
        if let FreeFormType::Cardinal = &self.form_type {
            match self.degree {
                Degree::Curve(u) => if u != 3 { return Err(FreeFormValidationExceptions::CardinalDegreeNotEqualToThree) },
                Degree::Surface(u, v) => if (u, v) != (3, 3) { return Err(FreeFormValidationExceptions::CardinalDegreeNotEqualToThree) },
            }
        }

        if let FreeFormType::BasisMatrix(atributes) = &self.form_type {
            match atributes.validate(&self.degree) {
                Ok(())                                                => return Ok(()),
                Err(BasisMatrixAttributesError::MatrixSizeMismatch)   => return Err(FreeFormValidationExceptions::InvalidMatrixSize),
                Err(BasisMatrixAttributesError::CurveSurfaceMismatch) => return Err(FreeFormValidationExceptions::CurveSurfaceMismatch),
                _                                                     => return Err(FreeFormValidationExceptions::UnknownException)
            }
        } else {
            Ok(())
        }
    }

    #[allow(unused_variables, unreachable_patterns)]    // Why is this marked unreachable & unused? Both aren't true
    fn parse_curve_surface_type(parameters: &VecDeque<String>) -> Result<(Option<FreeFormType>, Option<bool>),FreeFormDefinitionError> {
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
                        return Err(FreeFormDefinitionError::InvalidFreeFormType)
                    }
                }
                _ => return Err(FreeFormDefinitionError::InvalidParameters)
            }
        }
        Ok((free_form_type, rational))
    }

    fn parse_degree(parameters: &VecDeque<String>) -> Result<Option<Degree>, FreeFormDefinitionError> {
        match Degree::from(parameters) {
            Ok(value)                   => Ok(Some(value)),
            Err(UVPairError::InvalidBufferSize) => Err(FreeFormDefinitionError::InvalidBufferSize),
            Err(UVPairError::InvalidParameters) => Err(FreeFormDefinitionError::InvalidParameters)
        }
    }

    fn parse_matrix(current_type: &FreeFormType, parameters: &mut VecDeque<String>)  -> Result<FreeFormType, FreeFormDefinitionError> {
        let matrix_axis = parameters.pop_front().ok_or(FreeFormDefinitionError::InvalidParameters)?;
        match matrix_axis.as_str() {
            keywords::BASIS_MATRIX_U => Ok(FreeFormDefinition::parse_matrix_u(current_type, parameters)?),
            keywords::BASIS_MATRIX_V => Ok(FreeFormDefinition::parse_matrix_v(current_type, parameters)?),
            _ => Err(FreeFormDefinitionError::InvalidKeyword)
        }
    }

    fn parse_matrix_u(current_type: &FreeFormType, parameters: &VecDeque<String>) -> Result<FreeFormType, FreeFormDefinitionError> {
        let converted_parameters = utility::convert_vec::<f32>(parameters).ok().ok_or(FreeFormDefinitionError::InvalidParameters)?;
        let mut new_attributes: BasisMatrixAttributes;

        // If we're the right type, get the attributes
        if let FreeFormType::BasisMatrix(attributes) = &current_type {
            new_attributes = attributes.clone();
        } else {
            return Err(FreeFormDefinitionError::InvalidFormType);
        }

        // Matrices can have references to their elements. These references live as long as the matrix
        // To ensure that the references point to the proper place, we must create a new matrix
        // to invalidate any existing references. That and rust complains if I dont. c:
        new_attributes.set_matrix_u(&converted_parameters);

        Ok(FreeFormType::BasisMatrix(new_attributes))
    }

    fn parse_matrix_v(current_type: &FreeFormType, parameters: &VecDeque<String>) -> Result<FreeFormType,FreeFormDefinitionError> {
        let mut new_attributes: BasisMatrixAttributes;
        if let FreeFormType::BasisMatrix(attributes) = current_type {
            new_attributes = attributes.clone();
        } else {
            return Err(FreeFormDefinitionError::InvalidFormType);
        }

        let converted_parameters = utility::convert_vec::<f32>(parameters).ok().ok_or(FreeFormDefinitionError::InvalidParameters)?;
        // If matrix is a curve, this will cause an upgrade to surface
        new_attributes.set_matrix_v(&converted_parameters);

        Ok(FreeFormType::BasisMatrix(new_attributes))
    }

    fn parse_matrix_step(current_type: &FreeFormType, parameters: &VecDeque<String>) -> Result<FreeFormType, FreeFormDefinitionError> {
        let mut new_attributes: BasisMatrixAttributes;
        // If we're the right type, get the attributes
        if let FreeFormType::BasisMatrix(attributes) = &current_type {
            new_attributes = attributes.clone();
        } else {
            return Err(FreeFormDefinitionError::InvalidFormType);
        }

        let converted_parameters = utility::convert_vec::<usize>(parameters).ok().ok_or(FreeFormDefinitionError::InvalidParameters)?;
        if let Err(BasisMatrixAttributesError::InvalidBufferSize) = new_attributes.set_step(&converted_parameters) {
            return Err(FreeFormDefinitionError::InvalidBufferSize);
        }

        Ok(FreeFormType::BasisMatrix(new_attributes))
    }
}