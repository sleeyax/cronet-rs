use std::ptr;

use crate::Cronet_RawDataPtr;

pub struct Annotation {
    pub(crate) ptr: Cronet_RawDataPtr,
}

impl Annotation {
    /// Create a null pointer [Annotation].
    /// Useful for testing.
    pub fn null() -> Self {
        Annotation {
            ptr: ptr::null_mut(),
        }
    }
}

impl Default for Annotation {
    fn default() -> Self {
        Self::null()
    }
}
