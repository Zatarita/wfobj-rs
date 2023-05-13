/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * Basis Matrix Additional Attributes
 *      Basis matrices have additional information.
 *  
 *      A Step must be defined, this is a UV pair
 *          - U must be supplied at all times.
 *          - V must Supplied for all surfaces.
 * 
 * ------------------------------------------------------------------------------------*/

use crate::freeform_geometry::uv_pair::UVPair;

pub mod matrix;
pub mod basis_matrix_attributes;

pub type Step = UVPair;