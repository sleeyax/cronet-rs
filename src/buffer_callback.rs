use std::{sync::{Arc, Mutex}, collections::HashMap};

use once_cell::sync::Lazy;

use crate::{Cronet_BufferCallbackPtr, Cronet_BufferCallback_Destroy, Cronet_BufferCallback_CreateWith, Cronet_BufferPtr, Buffer};

#[no_mangle]
unsafe extern "C" fn cronetBufferCallbackOnDestroy (selfPtr: Cronet_BufferCallbackPtr, bufferPtr: Cronet_BufferPtr) {
  let mut lockedMap = bufferCallbackMap.lock().unwrap();
  if let Some(callback) = lockedMap.get(&selfPtr) {
    callback(BufferCallback { ptr: selfPtr }, Buffer {ptr: bufferPtr})
  }
  lockedMap.remove(&selfPtr);
}

/// Callback passed to `Buffer::initWithDataAndCallback` that gets invoked when the related `Buffer` is destroyed.
pub struct BufferCallback {
  pub(crate) ptr: Cronet_BufferCallbackPtr
}

pub type BufferCallbackFn = fn(callback: BufferCallback, buffer: Buffer);

/// Thread-safe HashMap that maps each native callback pointer to its respective Rust function.
/// Please note that this is a lazy loaded singleton / global variable and should never be publically accessible by consumers of this library.
static mut bufferCallbackMap: Lazy<Arc<Mutex<HashMap<Cronet_BufferCallbackPtr, BufferCallbackFn>>>> = Lazy::new(|| Arc::new(Mutex::new(HashMap::<Cronet_BufferCallbackPtr, BufferCallbackFn>::new())));

impl BufferCallback {
  pub fn new(onDestroy: BufferCallbackFn) -> Self {
    unsafe {
      let ptr = Cronet_BufferCallback_CreateWith(Some(cronetBufferCallbackOnDestroy));
      bufferCallbackMap.lock().unwrap().insert(ptr, onDestroy);
      BufferCallback { ptr }
    }
  }

  pub fn destroy(&self) {
    unsafe {
      Cronet_BufferCallback_Destroy(self.ptr);
    }
  }
}
