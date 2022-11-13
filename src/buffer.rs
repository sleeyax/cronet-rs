use std::ffi::c_void;

use crate::{Cronet_BufferPtr, Cronet_Buffer_Create, Cronet_Buffer_Destroy, Cronet_Buffer_InitWithAlloc, Cronet_Buffer_GetSize, Cronet_Buffer_GetData};

pub struct Buffer {
  ptr: Cronet_BufferPtr,
}

impl Buffer {
  pub fn new() -> Self {
    unsafe {
      Buffer { ptr: Cronet_Buffer_Create() }
    }
  }
  
  pub fn destroy(&self) {
    unsafe {
      Cronet_Buffer_Destroy(self.ptr);
    }
  }
  
  pub fn initWithAlloc(&self, size: u64) {
    unsafe {
      Cronet_Buffer_InitWithAlloc(self.ptr, size);
    }
  }
  
  // TODO: Cronet_Buffer_InitWithDataAndCallback

  pub fn size(&self) -> u64 {
    unsafe {
      Cronet_Buffer_GetSize(self.ptr)
    }
  }

  pub fn data(&self) -> *mut c_void {
    unsafe {
      Cronet_Buffer_GetData(self.ptr)
    }
  }
}
