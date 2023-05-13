/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/11/2023)
 * 
 * Free-Form Geometry
 *      Curves & Surfaces
 *          All Curves and Surfaces must have:
 *              Attributes (State-Setting)
 *                  - Type
 *                  - Degree
 *                      * Degree u required
 *                      * Degree v only required for surfaces 
 *                      * For cardinal curve any value other than 3 is ignored
 *              Elements
 *                  - Control Points
 *                  - 3D curves need parameter range
 *              Body Statements
 *                  - Global Parameters or Knot Vector
 *                  - Only valid when appearing between element and end statments
 *                  - Explicit "End" Statement
 *                      * At this step validation is done
 *          Additional Requirements for Basis Matrix:
 *              Attributes (State-Setting)
 *                  - Basis Matrix
 *                      * Matrix size must match degree(u, v): (u+1) x (v+1)
 *                  - Basis Step
 *                      * Step u required
 *                      * Step v only required for surfaces 
 *          Other important information
 *              All freeform curve and surface ATTRIBUTE statements are state-setting.
 *              Curves can be defined as rational or non-rational using the "rat" keyord
 * 
 * ------------------------------------------------------------------------------------*/

pub mod freeform_types;
pub mod freeform_obj;
pub mod basis_matrix;
pub mod uv_pair;

pub type Degree = uv_pair::UVPair;