// Holds all valid OBJ keywords according to: 
//  http://www.martinreddy.net/gfx/3d/OBJ.spec      (OBJ File Specification)
//  https://www.fileformat.info/format/material/    (MTL File Specification)

pub enum KeywordError {
    InvalidKeyword,
    InvalidCurveType,
    InvalidBasisMatrixAxis
}

/* ---------------------------------------------------------
 * OBJ File Keywords 
 * -------------------------------------------------------*/

pub const KEYWORD_COMMENT:                  &str = "#";
// Vertex data
pub const KEYWORD_VERTEX:                   &str = "v";
pub const KEYWORD_TEXTURE_COORDINATE:       &str = "vt";
pub const KEYWORD_VERTEX_NORMAL:            &str = "vn";
pub const KEYWORD_PARAMETER_SPACE_VERTEX:   &str = "vp";
pub const KEYWORD_CURVE_SURFACE_TYPE:       &str = "cstype";
pub const KEYWORD_DEGREE:                   &str = "deg";
pub const KEYWORD_BASIS_MATRIX:             &str = "bmat";
pub const KEYWORD_STEP_SIZE:                &str = "step";
// elements
pub const KEYWORD_POINT:                    &str = "p";
pub const KEYWORD_LINE:                     &str = "l";
pub const KEYWORD_FACE:                     &str = "f";
pub const KEYWORD_CURVE:                    &str = "curv";
pub const KEYWORD_CURVE_2D:                 &str = "curv2";
pub const KEYWORD_SURFACE:                  &str = "surf";
// Freeform curve/surface body statements
pub const KEYWORD_PARAMETER_VALUE:          &str = "parm";
pub const KEYWORD_OUTER_TRIM_LOOP:          &str = "trim";
pub const KEYWORD_INNER_TRIM_LOOP:          &str = "hole";
pub const KEYWORD_SPECIAL_CURVE:            &str = "scrv";
pub const KEYWORD_SPECIAL_POINT:            &str = "sp";
pub const KEYWORD_END:                      &str = "end";
// Connectivity between free-form surfaces
pub const KEYWORD_CONNECTION:               &str = "con";
// Grouping
pub const KEYWORD_GROUP_NAME:               &str = "g";
pub const KEYWORD_SMOOTHING_GROUP:          &str = "s";
pub const KEYWORD_MERGING_GROUP:            &str = "mg";
pub const KEYWORD_OBJECT_NAME:              &str = "o";
// Display/render attributes
pub const KEYWORD_BEVEL_INTERPOLATION:      &str = "bevel";
pub const KEYWORD_COLOR_INTERPOLATION:      &str = "c_interp";
pub const KEYWORD_DISSOLVE_INTERPOLATION:   &str = "d_interp";
pub const KEYWORD_LEVEL_OF_DETAIL:          &str = "lod";
pub const KEYWORD_MATERIAL_NAME:            &str = "usemtl";
pub const KEYWORD_MATERIAL_LIBRARY:         &str = "mtllib";
pub const KEYWORD_SHADOW_CASTING:           &str = "shadow_obj";
pub const KEYWORD_RAY_TRACING:              &str = "trace_obj";
pub const KEYWORD_CURVE_APPROXIMATION:      &str = "ctech";
pub const KEYWORD_SURFACE_APPROXIMATION:    &str = "stech";

// Freeform curve/surface attributes
pub const KEYWORD_RATIONAL_CURVE:           &str = "rat";

pub const KEYWORD_CURVE_TYPE_BMATRIX:       &str = "bmatrix";
pub const KEYWORD_CURVE_TYPE_BEZIER:        &str = "bezier";
pub const KEYWORD_CURVE_TYPE_BSPLINE:       &str = "bspline";
pub const KEYWORD_CURVE_TYPE_CARDINAL:      &str = "cardinal";
pub const KEYWORD_CURVE_TYPE_TAYLOR:        &str = "taylor";

// Basis Matrix
pub const KEYWORD_BASIS_MATRIX_U:           &str = "u";
pub const KEYWORD_BASIS_MATRIX_V:           &str = "v";


/* ---------------------------------------------------------
 * Compliance
 * -------------------------------------------------------*/

pub const VALID_KEYWORDS: [&'static str; 54] = [
    KEYWORD_VERTEX,
    KEYWORD_TEXTURE_COORDINATE,
    KEYWORD_VERTEX_NORMAL,
    KEYWORD_PARAMETER_SPACE_VERTEX,
    KEYWORD_CURVE_SURFACE_TYPE,
    KEYWORD_DEGREE,
    KEYWORD_BASIS_MATRIX,
    KEYWORD_STEP_SIZE,
    KEYWORD_POINT,
    KEYWORD_LINE,
    KEYWORD_FACE,
    KEYWORD_CURVE,
    KEYWORD_CURVE_2D,
    KEYWORD_SURFACE,
    KEYWORD_PARAMETER_VALUE,
    KEYWORD_OUTER_TRIM_LOOP,
    KEYWORD_INNER_TRIM_LOOP,
    KEYWORD_SPECIAL_CURVE,
    KEYWORD_SPECIAL_POINT,
    KEYWORD_END,          
    KEYWORD_CONNECTION,
    KEYWORD_GROUP_NAME,
    KEYWORD_SMOOTHING_GROUP,
    KEYWORD_MERGING_GROUP,
    KEYWORD_OBJECT_NAME,
    KEYWORD_BEVEL_INTERPOLATION,
    KEYWORD_COLOR_INTERPOLATION,
    KEYWORD_DISSOLVE_INTERPOLATION,
    KEYWORD_LEVEL_OF_DETAIL,
    KEYWORD_MATERIAL_NAME,
    KEYWORD_MATERIAL_LIBRARY,
    KEYWORD_SHADOW_CASTING,
    KEYWORD_RAY_TRACING,
    KEYWORD_CURVE_APPROXIMATION,   
    KEYWORD_SURFACE_APPROXIMATION, 

    KEYWORD_NEW_MATERIAL,
    KEYWORD_AMBIENT_REFLECTIVITY,
    KEYWORD_DIFFUSE_REFLECTIVITY,
    KEYWORD_SPECULAR_REFLECTIVITY,
    KEYWORD_TRANSMISSION_FILTER,
    KEYWORD_ILLUMINATION_MODEL,
    KEYWORD_DISSOLVE_FACTOR,
    KEYWORD_SPECULAR_EXPONENT,
    KEYWORD_SHARPNESS,
    KEYWORD_OPTICAL_DENSITY,
    KEYWORD_AMBIENT_REFLECTIVITY_MAP,
    KEYWORD_DIFFUSE_REFLECTIVITY_MAP,
    KEYWORD_SPECULAR_REFLECTIVITY_MAP,
    KEYWORD_SPECULAR_EXPONENT_MAP,
    KEYWORD_DISSOLVE_FACTOR_MAP,
    KEYWORD_DISPLACEMENT_MAP,
    KEYWORD_DECAL_MAP,
    KEYWORD_BUMP_MAP,
    KEYWORD_REFLECTION_MAP
];

pub fn validate_keyword(keyword: &str) -> bool {
    if !VALID_KEYWORDS.contains( &keyword ) {
        return false;
    } 
    true
}

pub const VALID_CURVE_TYPES: [&'static str; 5] = [
    KEYWORD_CURVE_TYPE_BMATRIX, 
    KEYWORD_CURVE_TYPE_BEZIER,
    KEYWORD_CURVE_TYPE_BSPLINE,
    KEYWORD_CURVE_TYPE_CARDINAL,
    KEYWORD_CURVE_TYPE_TAYLOR
];

pub fn validate_curve(keyword: &str) -> bool {
    if !VALID_CURVE_TYPES.contains(&keyword) {
        return false;
    }
    true
}

pub const VALID_BASIS_MATRIX_AXES: [&'static str; 2] = [
    KEYWORD_BASIS_MATRIX_U,
    KEYWORD_BASIS_MATRIX_V
];

pub fn validate_basis_axis(keyword: &str) -> bool {
    if !VALID_BASIS_MATRIX_AXES.contains(&keyword) {
        return true;
    }
    false
}
