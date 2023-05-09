/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/8/2023)
 * 
 * OBJ Vertex Data Macro & Definitions
 * 
 * ------------------------------------------------------------------------------------*/

use std::collections::VecDeque;

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

// Generalized Vertex data. Could be a derive, but I dont' feel like it for something this small
macro_rules! vertex_data {
    ( $struct_name:ident, $ty:ty, $( $v:ident ),* ) => {
        #[derive(Debug, Clone, Default, PartialEq)]
        pub struct $struct_name {
            $(
                pub $v: $ty,
            )*
        }

        #[allow(unused_assignments, dead_code)]
        impl $struct_name {
            pub fn new() -> $struct_name {
                $struct_name {
                    $(
                        $v: <$ty>::default(),
                    )*
                }
            }
        }
    };
}

// Generalized from method. Could be a derive, but I don't feel like it for something this small
macro_rules! vertex_from {
    ( $struct_name:ident, $ty:ty, $( $v:ident ),* ) => {
        pub fn from(elements: &VecDeque<String>) -> Option<$struct_name> {
            // Validate the number of elements read from the line
            if elements.len() < count!($($v)*) {
                return None;
            }

            // convert the elements into the correct type
            let mut converted_elements = Vec::<$ty>::new();
            converted_elements.reserve(elements.len());
            for element in elements {
                converted_elements.push(element.parse().ok()?);
            }

            // Create variables for struct using short hand naming convention
            let mut i = 0usize;
            $(
                let $v = converted_elements[i];
                i += 1;
            )*

            // Return the struct
            Some($struct_name {$($v,)*})
        }
    }
}

vertex_data!(Vertex, f32, x, y, z, w);
vertex_data!(ParameterSpaceVertex, f32, u, v, w);
vertex_data!(VertexNormal, f32, i, j, k);
vertex_data!(TextureCoordinates, f32, u, v, w);

// Every "from" method can be generalized except for vertex
impl ParameterSpaceVertex   { vertex_from!(ParameterSpaceVertex, f32, u, v, w); }
impl VertexNormal           { vertex_from!(VertexNormal, f32, i, j, k);         }
impl TextureCoordinates     { vertex_from!(TextureCoordinates, f32, u, v, w);   }

// The Vertex "from" method is unique as it has the chance to have a default value.
impl Vertex {
    pub fn from(elements: &VecDeque<String>) -> Option<Vertex> {
        // convert the elements into the correct type
        let mut converted_elements = Vec::<f32>::new();
        converted_elements.reserve(elements.len());
        for element in elements {
            converted_elements.push(element.parse().ok()?);
        }

        match elements.len() {
            3 => Some(Vertex { x: converted_elements[0], y: converted_elements[1], z: converted_elements[2], w: 1.0 }),
            4 => Some(Vertex { x: converted_elements[0], y: converted_elements[1], z: converted_elements[2], w: converted_elements[3] }),
            _ => None
        }
    }
}