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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Matrix;