/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/8/2023)
 * 
 * OBJ File Keywords & Compliance Validation
 * http://www.martinreddy.net/gfx/3d/OBJ.spec
 * 
 * ------------------------------------------------------------------------------------*/


/* --------------------------------------------------------------------------------------
 * OBJ File Keywords 
 * ------------------------------------------------------------------------------------*/
pub const COMMENT:                                          &str = "#";
// Vertex data                      
pub const VERTEX:                                           &str = "v";
pub const TEXTURE_COORDINATE:                               &str = "vt";
pub const VERTEX_NORMAL:                                    &str = "vn";
pub const PARAMETER_SPACE_VERTEX:                           &str = "vp";
pub const CURVE_SURFACE_TYPE:                               &str = "cstype";
pub const DEGREE:                                           &str = "deg";
pub const BASIS_MATRIX:                                     &str = "bmat";
pub const STEP_SIZE:                                        &str = "step";
// elements                     
pub const POINT:                                            &str = "p";
pub const LINE:                                             &str = "l";
pub const FACE:                                             &str = "f";
pub const CURVE:                                            &str = "curv";
pub const CURVE_2D:                                         &str = "curv2";
pub const SURFACE:                                          &str = "surf";
// Freeform curve/surface body statements                       
pub const PARAMETER_VALUE:                                  &str = "parm";
pub const OUTER_TRIM_LOOP:                                  &str = "trim";
pub const INNER_TRIM_LOOP:                                  &str = "hole";
pub const SPECIAL_CURVE:                                    &str = "scrv";
pub const SPECIAL_POINT:                                    &str = "sp";
pub const END:                                              &str = "end";
// Connectivity between free-form surfaces                      
pub const CONNECTION:                                       &str = "con";
// Grouping                     
pub const GROUP_NAME:                                       &str = "g";
pub const SMOOTHING_GROUP:                                  &str = "s";
pub const MERGING_GROUP:                                    &str = "mg";
pub const OBJECT_NAME:                                      &str = "o";
// Display/render attributes                        
pub const BEVEL_INTERPOLATION:                              &str = "bevel";
pub const COLOR_INTERPOLATION:                              &str = "c_interp";
pub const DISSOLVE_INTERPOLATION:                           &str = "d_interp";
pub const LEVEL_OF_DETAIL:                                  &str = "lod";
pub const MAP_LIB:                                          &str = "maplib";
pub const USE_MAP:                                          &str = "usemap";
pub const MATERIAL_NAME:                                    &str = "usemtl";
pub const MATERIAL_LIBRARY:                                 &str = "mtllib";
pub const SHADOW_CASTING:                                   &str = "shadow_obj";
pub const RAY_TRACING:                                      &str = "trace_obj";
pub const CURVE_APPROXIMATION:                              &str = "ctech";
pub const SURFACE_APPROXIMATION:                            &str = "stech";
// Freeform Curve/Surface Approximation Techniques              
pub const CONSTANT_PARAMETRIC_SUBDIVISION:                  &str = "cparm";
pub const CONSTANT_SPATIAL_SUBDIVISION:                     &str = "cspace";
pub const CURVE_DEPENDANT_SUBDIVISION:                      &str = CURVE;
pub const CONSTANT_PARAMETRIC_SUBDIVISION_SURFACE_MULTI:    &str = "cparma";
pub const CONSTANT_PARAMETRIC_SUBDIVISION_SURFACE_SINGLE:   &str = "cparmb";
// Freeform curve/surface attributes                
pub const RATIONAL_CURVE:                                   &str = "rat";
// Supported Curve Types
pub const CURVE_TYPE_BMATRIX:                               &str = "bmatrix";
pub const CURVE_TYPE_BEZIER:                                &str = "bezier";
pub const CURVE_TYPE_BSPLINE:                               &str = "bspline";
pub const CURVE_TYPE_CARDINAL:                              &str = "cardinal";
pub const CURVE_TYPE_TAYLOR:                                &str = "taylor";
// Basis Matrix                     
pub const BASIS_MATRIX_U:                                   &str = "u";
pub const BASIS_MATRIX_V:                                   &str = "v";


/* --------------------------------------------------------------------------------------
 * Compliance
 * ------------------------------------------------------------------------------------*/

pub const VALID_KEYWORDS: [&'static str; 48] = [
    VERTEX,
    TEXTURE_COORDINATE,
    VERTEX_NORMAL,
    PARAMETER_SPACE_VERTEX,
    CURVE_SURFACE_TYPE,
    DEGREE,
    BASIS_MATRIX,
    STEP_SIZE,
    POINT,
    LINE,
    FACE,
    CURVE,
    CURVE_2D,
    SURFACE,
    PARAMETER_VALUE,
    OUTER_TRIM_LOOP,
    INNER_TRIM_LOOP,
    SPECIAL_CURVE,
    SPECIAL_POINT,
    END,          
    CONNECTION,
    GROUP_NAME,
    SMOOTHING_GROUP,
    MERGING_GROUP,
    OBJECT_NAME,
    BEVEL_INTERPOLATION,
    COLOR_INTERPOLATION,
    DISSOLVE_INTERPOLATION,
    LEVEL_OF_DETAIL,
    MATERIAL_NAME,
    MATERIAL_LIBRARY,
    SHADOW_CASTING,
    RAY_TRACING,
    CURVE_APPROXIMATION,   
    SURFACE_APPROXIMATION,
    CONSTANT_PARAMETRIC_SUBDIVISION,
    CONSTANT_SPATIAL_SUBDIVISION,
    CURVE_DEPENDANT_SUBDIVISION,
    CONSTANT_PARAMETRIC_SUBDIVISION_SURFACE_MULTI,
    CONSTANT_PARAMETRIC_SUBDIVISION_SURFACE_SINGLE,
    RATIONAL_CURVE,
    CURVE_TYPE_BMATRIX,
    CURVE_TYPE_BEZIER,
    CURVE_TYPE_BSPLINE,
    CURVE_TYPE_CARDINAL,
    CURVE_TYPE_TAYLOR,
    BASIS_MATRIX_U,
    BASIS_MATRIX_V
];

pub fn validate_keyword(keyword: &str) -> bool {
    if !VALID_KEYWORDS.contains( &keyword ) {
        return false;
    } 
    true
}

pub const VALID_CURVE_TYPES: [&'static str; 5] = [
    CURVE_TYPE_BMATRIX, 
    CURVE_TYPE_BEZIER,
    CURVE_TYPE_BSPLINE,
    CURVE_TYPE_CARDINAL,
    CURVE_TYPE_TAYLOR
];

pub fn validate_curve(keyword: &str) -> bool {
    if !VALID_CURVE_TYPES.contains(&keyword) {
        return false;
    }
    true
}

pub const VALID_BASIS_MATRIX_AXES: [&'static str; 2] = [
    BASIS_MATRIX_U,
    BASIS_MATRIX_V
];

pub fn validate_basis_axis(keyword: &str) -> bool {
    if !VALID_BASIS_MATRIX_AXES.contains(&keyword) {
        return false;
    }
    true
}

pub const VALID_FREEFORM_APPROXIMATION_TECHNIQUES: [&'static str; 5] = [
    CONSTANT_PARAMETRIC_SUBDIVISION,
    CONSTANT_SPATIAL_SUBDIVISION,
    CURVE_DEPENDANT_SUBDIVISION,
    CONSTANT_PARAMETRIC_SUBDIVISION_SURFACE_MULTI,
    CONSTANT_PARAMETRIC_SUBDIVISION_SURFACE_SINGLE
];

pub fn validate_approximation_techniques(keyword: &str) -> bool {
    if !VALID_FREEFORM_APPROXIMATION_TECHNIQUES.contains(&keyword) {
        return false;
    }
    true
}

pub const VALID_DISPLAY_RENDER_ATTRIBUTES: [&'static str; 12] = [
    BEVEL_INTERPOLATION,
    COLOR_INTERPOLATION,
    DISSOLVE_INTERPOLATION,
    LEVEL_OF_DETAIL,
    MAP_LIB,
    USE_MAP,
    MATERIAL_NAME,
    MATERIAL_LIBRARY,
    SHADOW_CASTING,
    RAY_TRACING,
    CURVE_APPROXIMATION,
    SURFACE_APPROXIMATION
];

pub fn validate_display_render_attributes(keyword: &str) -> bool {
    if !VALID_DISPLAY_RENDER_ATTRIBUTES.contains(&keyword) {
        return false;
    }
    true
}