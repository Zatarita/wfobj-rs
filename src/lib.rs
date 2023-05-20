#![allow(dead_code)]

pub mod vertex_data;
pub mod keywords;
pub mod parser;
pub mod interpreter;
pub mod vertex_buffer;
pub mod index;
pub mod freeform_geometry;
pub mod utility;


pub use crate::freeform_geometry::basis_matrix::matrix_elements::MatrixElements;
pub use crate::freeform_geometry::freeform_obj::FreeFormObject;
pub use crate::freeform_geometry::freeform_types::FreeFormType;
pub use crate::freeform_geometry::basis_matrix::basis_matrix_attributes::BasisMatrixAttributes;
pub use crate::freeform_geometry::basis_matrix::matrix::Matrix;
pub use crate::freeform_geometry::Degree;
pub use crate::freeform_geometry::basis_matrix::Step;


#[cfg(test)]
mod tests {
    use super::*;

    use crate::parser::ObjParser;
    #[test]
    fn test_bmatrix_01_cstype() {
        let test_obj_path = "test_objs/bmatrix_curve_definition_test_01.obj";

        let rational = false;
        let degree = Degree::Surface(3, 3);

        // Construct basis matrix (u,v) are the same
        let matrix_uv = vec![1.0, -3.0, 3.0, -1.0, 0.0, 3.0, -6.0, 3.0, 0.0, 0.0, 3.0, -3.0, 0.0, 0.0, 0.0, 1.0];
        let matrix_elements = MatrixElements::new(&matrix_uv);
        let bmatrix_matrix = Matrix::Surface(matrix_elements.clone(), matrix_elements);

        // Construct matrix step
        let matrix_step = Step::Surface(3, 3);

        // Construct matrix attributes
        let bmatrix_attributes = BasisMatrixAttributes {step: matrix_step, matrix: bmatrix_matrix};

        // Construct matrix free form type
        let form_type = FreeFormType::BasisMatrix(bmatrix_attributes);
        
        // Contruct freeform object
        let bmatrix_test = FreeFormObject::new(form_type, rational, degree);

        let mut parser = ObjParser::new(test_obj_path).unwrap();

        let bmatrix_parser_test = FreeFormObject::parse(&mut parser).unwrap();
        assert_eq!(bmatrix_test, bmatrix_parser_test);
    }


    #[test]
    fn test_bmatrix_02_cstype() {
        let test_obj_path = "test_objs/bmatrix_curve_definition_test_02.obj";

        let rational = false;
        let degree = Degree::Curve(3);

        // Construct basis matrix
        let matrix_u = vec![1.0, 0.0, -3.0, 2.0, 0.0, 0.0, 3.0, -2.0, 0.0, 1.0, -2.0, 1.0, 0.0, 0.0, -1.0, 1.0];
        let matrix_elements = MatrixElements::new(&matrix_u);
        let bmatrix_matrix = Matrix::Curve(matrix_elements);

        // Construct matrix step
        let matrix_step = Step::Curve(2);

        // Construct matrix attributes
        let bmatrix_attributes = BasisMatrixAttributes {step: matrix_step, matrix: bmatrix_matrix};

        // Construct matrix free form type
        let form_type = FreeFormType::BasisMatrix(bmatrix_attributes);
        
        // Contruct freeform object
        let bmatrix_test = FreeFormObject::new(form_type, rational, degree);

        let mut parser = ObjParser::new(test_obj_path).unwrap();

        let bmatrix_parser_test = FreeFormObject::parse(&mut parser).unwrap();
        assert_eq!(bmatrix_test, bmatrix_parser_test);
    }


    #[test]
    fn bmatrix_element_reference_test() {
        let test_obj_path = "test_objs/bmatrix_curve_definition_test_01.obj";
        let mut parser = ObjParser::new(test_obj_path).unwrap();
        let bmatrix = FreeFormObject::parse(&mut parser).unwrap();

        if let FreeFormType::BasisMatrix(attributes) = bmatrix.form_type {
            let row: Vec<Vec<&f32>> = attributes.matrix.get_row(2, &bmatrix.degree).unwrap();
            let expectation = vec![&0.0, &3.0, &-6.0, &3.0];

            assert_eq!(row[0], expectation);
            assert_eq!(row[1], expectation);

            let column: Vec<Vec<&f32>> = attributes.matrix.get_column(2, &bmatrix.degree).unwrap();
            let expectation = vec![&-3.0, &3.0, &0.0, &0.0];

            assert_eq!(column[0], expectation);
            assert_eq!(column[1], expectation);
        }

    }


    #[test]
    fn test_bspline_definition() {
        let test_obj_path = "test_objs/bspline_definition_test.obj";

        let rational = true;
        let degree = Degree::Surface(2, 2);

        // Construct BSpline freeform type
        let form_type = FreeFormType::BSpline;
        
        // Contruct freeform object
        let bspline_test = FreeFormObject::new(form_type, rational, degree);

        let mut parser = ObjParser::new(test_obj_path).unwrap();

        let bspline_parser_test = FreeFormObject::parse(&mut parser).unwrap();
        assert_eq!(bspline_test, bspline_parser_test);
    }
    
}