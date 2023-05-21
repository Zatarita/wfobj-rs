/* --------------------------------------------------------------------------------------

 * Author: Zatarita
 * Last Edit: Zatarita (5/11/2023)
 * 
 * OBJ Indices
 *      OBJ file indices start at 1 instead of 0.
 *      Negative indices are relative to the end of the buffer
 *      Positive indices are absolute
 * 
 *      This helps ensure that's obvious
 * 
 * ------------------------------------------------------------------------------------*/

use std::rc::Rc;

type BufferObject<T> = Vec<Rc<T>>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndexError {
    IndexIsZero    // Only value index cannot be is zero, negative indices are relative, positive indices are absolute
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Index(isize);
 
impl Index {
    pub fn new(value: isize) -> Result<Index, IndexError> {
        if value == 0 { 
            return Err(IndexError::IndexIsZero) 
        }
        Ok( Index(value) )
    }

    pub fn as_isize(&self) -> isize {
        // This should hopefully never be possible.
        if self.0 == 0 {
            panic!("Index is equal to 0!");
        }

        if self.0 > 0 {
            self.0 - 1  // Absolute
        } else {
            self.0      // Relative
        }
    }
}

impl Default for Index {
    fn default() -> Self { 
        Index(1isize)
    }
}