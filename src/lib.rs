#![allow(dead_code)]

pub mod vertex_data;
pub mod keywords;
pub mod parser;
pub mod interpreter;
pub mod vertex_buffer;
pub mod index;
pub mod freeform_geometry;
pub mod utility;

pub use crate::parser::{ObjLine, ObjParser};

pub use crate::freeform_geometry::basis_matrix::matrix::{Matrix, MatrixError};
pub use crate::freeform_geometry::basis_matrix::matrix_elements::{MatrixElements, MatrixColumn, MatrixRow};
pub use crate::freeform_geometry::basis_matrix::basis_matrix_attributes::{BasisMatrixAttributes, BasisMatrixAttributesError};

pub use crate::freeform_geometry::freeform_definition::{FreeFormDefinition, FreeFormDefinitionError, FreeFormValidationExceptions};
pub use crate::freeform_geometry::freeform_types::{FreeFormType, FreeFormTypesErrors};

pub use crate::freeform_geometry::uv_pair::UVPairError;
pub use crate::freeform_geometry::Degree;
pub use crate::freeform_geometry::basis_matrix::Step;

pub use crate::vertex_buffer::{VertexBuffer, VertexBufferError};
pub use crate::vertex_data::*;
pub use crate::index::{Index, IndexError};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_buffer_indexing() {
        let mut buffer = VertexBuffer::new();

        let third_vert          = Vertex { x:  10.0, y:  10.0, z:  10.0, w: 1.0 };
        let second_to_last_vert = Vertex { x:  10.0, y:  10.0, z: -10.0, w: 1.0 };

        // Default cube
        // Front
        buffer.add_vertex(Vertex { x: -10.0, y: -10.0, z:  10.0, w: 1.0 });
        buffer.add_vertex(Vertex { x:  10.0, y: -10.0, z:  10.0, w: 1.0 });
        buffer.add_vertex(third_vert.clone());
        buffer.add_vertex(Vertex { x: -10.0, y:  10.0, z:  10.0, w: 1.0 });
        // Back
        buffer.add_vertex(Vertex { x: -10.0, y: -10.0, z: -10.0, w: 1.0 });
        buffer.add_vertex(Vertex { x:  10.0, y: -10.0, z: -10.0, w: 1.0 });
        buffer.add_vertex(second_to_last_vert.clone());
        buffer.add_vertex(Vertex { x: -10.0, y:  10.0, z: -10.0, w: 1.0 });

        let third_index = Index::new(3).unwrap();
        let second_to_last_index = Index::new(-2).unwrap();

        assert_eq!(*buffer.get_vertex(third_index).unwrap(), third_vert);
        assert_eq!(*buffer.get_vertex(second_to_last_index).unwrap(), second_to_last_vert);
    }

    #[test]
    fn test_free_form_validation() {
        // Construct a matrix with a degree as a curve, but the matrix step as a surface.
        let rational = false;
        let matrix_uv = vec![1.0, -3.0,  3.0, -1.0, 
                                       0.0,  3.0, -6.0,  3.0, 
                                       0.0,  0.0,  3.0, -3.0,
                                       0.0,  0.0,  0.0,  1.0];
        let matrix_elements = MatrixElements::new(&matrix_uv);
        let bmatrix_matrix = Matrix::Surface(matrix_elements.clone(), matrix_elements);
        let matrix_step = Step::Surface(3, 3);
        let bmatrix_attributes = BasisMatrixAttributes {step: matrix_step, matrix: bmatrix_matrix};
        let form_type = FreeFormType::BasisMatrix(bmatrix_attributes);
        let invalid_degree = Degree::Curve(3);
        let bmatrix_test = FreeFormDefinition::new(form_type.clone(), rational.clone(), invalid_degree.clone());
        assert_eq!(bmatrix_test.validate(), Err(FreeFormValidationExceptions::CurveSurfaceMismatch));

        // Construct a matrix that has a matrix too large for the assigned V degree
        let too_small_of_degree = Degree::Surface(3, 2);
        let bmatrix_with_too_big_of_matrix = FreeFormDefinition::new(form_type.clone(), rational.clone(), too_small_of_degree);
        assert_eq!(bmatrix_with_too_big_of_matrix.validate(), Err(FreeFormValidationExceptions::InvalidMatrixSize));

        // Construct a correct basis matrix
        let correct_degree = Degree::Surface(3, 3);
        let correct_bmatrix_test = FreeFormDefinition::new(form_type.clone(), rational.clone(), correct_degree);
        assert_eq!(correct_bmatrix_test.validate(), Ok(()));

        // Cardinal curves always have degree 3
        let invalid_cardinal = FreeFormDefinition::new(FreeFormType::Cardinal, false, Degree::Curve(2));
        assert_eq!(invalid_cardinal.validate(), Err(FreeFormValidationExceptions::CardinalDegreeNotEqualToThree));

        // Correct cardinal curve
        let cardinal = FreeFormDefinition::new(FreeFormType::Cardinal, false, Degree::Curve(3));
        assert_eq!(cardinal.validate(), Ok(()));

    }

    #[test]
    fn test_bmatrix_01_cstype() {
        let test_obj_path = "test_objs/bmatrix_curve_definition_test_01.obj";

        let rational = false;
        let degree = Degree::Surface(3, 3);

        // Construct basis matrix (u,v) are the same
        let matrix_uv = vec![1.0, -3.0,  3.0, -1.0, 
                                       0.0,  3.0, -6.0,  3.0, 
                                       0.0,  0.0,  3.0, -3.0,
                                       0.0,  0.0,  0.0,  1.0];
        let matrix_elements = MatrixElements::new(&matrix_uv);
        let bmatrix_matrix = Matrix::Surface(matrix_elements.clone(), matrix_elements);

        // Construct matrix step
        let matrix_step = Step::Surface(3, 3);

        // Construct matrix attributes
        let bmatrix_attributes = BasisMatrixAttributes {step: matrix_step, matrix: bmatrix_matrix};

        // Construct matrix free form type
        let form_type = FreeFormType::BasisMatrix(bmatrix_attributes);
        
        // Contruct freeform object
        let bmatrix_test = FreeFormDefinition::new(form_type, rational, degree);

        let mut parser = ObjParser::new(test_obj_path).unwrap();

        let bmatrix_parser_test = FreeFormDefinition::parse(&mut parser).unwrap();
        assert_eq!(bmatrix_test, bmatrix_parser_test);
    }


    #[test]
    fn test_bmatrix_02_cstype() {
        let test_obj_path = "test_objs/bmatrix_curve_definition_test_02.obj";

        let rational = false;
        let degree = Degree::Curve(3);

        // Construct basis matrix
        let matrix_u = vec![1.0, 0.0, -3.0,  2.0, 
                                      0.0, 0.0,  3.0, -2.0, 
                                      0.0, 1.0, -2.0,  1.0, 
                                      0.0, 0.0, -1.0,  1.0];
        let matrix_elements = MatrixElements::new(&matrix_u);
        let bmatrix_matrix = Matrix::Curve(matrix_elements);

        // Construct matrix step
        let matrix_step = Step::Curve(2);

        // Construct matrix attributes
        let bmatrix_attributes = BasisMatrixAttributes {step: matrix_step, matrix: bmatrix_matrix};

        // Construct matrix free form type
        let form_type = FreeFormType::BasisMatrix(bmatrix_attributes);
        
        // Contruct freeform object
        let bmatrix_test = FreeFormDefinition::new(form_type, rational, degree);

        let mut parser = ObjParser::new(test_obj_path).unwrap();

        let bmatrix_parser_test = FreeFormDefinition::parse(&mut parser).unwrap();
        assert_eq!(bmatrix_test, bmatrix_parser_test);
    }


    #[test]
    fn bmatrix_element_reference_test() {
        let test_obj_path = "test_objs/bmatrix_curve_definition_test_01.obj";
        let mut parser = ObjParser::new(test_obj_path).unwrap();
        let bmatrix = FreeFormDefinition::parse(&mut parser).unwrap();

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
        let bspline_test = FreeFormDefinition::new(form_type, rational, degree);

        let mut parser = ObjParser::new(test_obj_path).unwrap();

        let bspline_parser_test = FreeFormDefinition::parse(&mut parser).unwrap();
        assert_eq!(bspline_test, bspline_parser_test);
    }
    
}