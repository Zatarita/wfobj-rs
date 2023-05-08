/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/8/2023)
 * 
 * OBJ Vertex Data Macro & Definitions
 * 
 * ------------------------------------------------------------------------------------*/

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

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

            pub fn from(elements: &[&str]) -> Option<$struct_name> {
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
    };
}

vertex_data!(Vertex, f32, x, y, z, w);
vertex_data!(ParameterSpaceVertex, f32, u, v, w);
vertex_data!(VertexNormal, f32, i, j, k);
vertex_data!(TextureCoordinates, f32, u, v, w);