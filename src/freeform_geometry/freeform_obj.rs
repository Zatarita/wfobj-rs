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

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FreeformObject {
    curve_type: FreeFormType,
    rational:   bool,
    degree:     Degree,
}