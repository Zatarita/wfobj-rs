/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * UVPair - Used for Step and Degree
 *   UVPair u required
 *   UVPair v only required for surfaces 
 * 
 * ------------------------------------------------------------------------------------*/

use std::collections::VecDeque;
use crate::utility;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UVPairError {
    InvalidParameters,
    InvalidBufferSize
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UVPair {
    Curve(usize),
    Surface(usize, usize)
}

impl UVPair {
    pub fn from(parameters: &VecDeque<String>) -> Result<UVPair, UVPairError> {
        let converted_parameters = match utility::convert_to_usize(parameters) {
            Ok(value) => value,
            _ => return Err(UVPairError::InvalidParameters)
        };

        match converted_parameters.len() {
            1 => Ok(UVPair::Curve(converted_parameters[0])),
            2 => Ok(UVPair::Surface(converted_parameters[0], converted_parameters[1])),
            _ => Err(UVPairError::InvalidBufferSize)
        }
    }

    pub fn u(&self) -> &usize {
        match self {
            Self::Curve(value)      => value,
            Self::Surface(value, _) => value
        }
    }

    pub fn v(&self) -> Option<&usize> {
        match self {
            Self::Surface(_, value) => Some(value),
            _                             => None
        }
    }

    pub fn is_curve(&self) -> bool {
        match self {
            Self::Curve(_) => true,
            _              => false
        }
    }

    pub fn is_surface(&self) -> bool {
        match self {
            Self::Surface(_, _) => true,
            _                   => false
        }
    }
}

impl Default for UVPair {
    fn default() -> Self { 
        UVPair::Curve(0)
    }
}
