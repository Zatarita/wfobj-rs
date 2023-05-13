/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/13/2023)
 * 
 * Basis Matrix Attributes
 *      Basis matrices have extra attribute data
 *      this defines the Step and Matrix that define the curve/surface
 * 
 * ------------------------------------------------------------------------------------*/

use super::Step;
use super::matrix::Matrix;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct BasisMatrixAttributes {
    step: Step,
    matrix: Matrix
}