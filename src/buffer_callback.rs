use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use once_cell::sync::Lazy;

use crate::{
    Buffer, Cronet_BufferCallbackPtr, Cronet_BufferCallback_CreateWith,
    Cronet_BufferCallback_Destroy, Cronet_BufferPtr,
};

/// Thread-safe global HashMap to map each native callback pointer to its respective Rust function.
/// Please note that this is a lazy loaded global singleton that only exists for state management and should thus never be publically accessible to consumers of this library.
static mut buffer_callbacks: Lazy<Arc<Mutex<HashMap<Cronet_BufferCallbackPtr, BufferCallbackFn>>>> =
    Lazy::new(|| {
        Arc::new(Mutex::new(HashMap::<
            Cronet_BufferCallbackPtr,
            BufferCallbackFn,
        >::new()))
    });

#[no_mangle]
unsafe extern "C" fn cronetBufferCallbackOnDestroy(
    selfPtr: Cronet_BufferCallbackPtr,
    bufferPtr: Cronet_BufferPtr,
) {
    let mut lockedMap = buffer_callbacks.lock().unwrap();
    lockedMap.remove(&selfPtr);
    if let Some(callback) = lockedMap.get(&selfPtr) {
        callback(BufferCallback { ptr: selfPtr }, Buffer { ptr: bufferPtr })
    }
}

pub type BufferCallbackFn = fn(callback: BufferCallback, buffer: Buffer);

/// Callback passed to `Buffer::initWithDataAndCallback` that gets invoked when the related `Buffer` is destroyed.
pub struct BufferCallback {
    pub(crate) ptr: Cronet_BufferCallbackPtr,
}

impl BufferCallback {
    pub fn new(onDestroy: BufferCallbackFn) -> Self {
        unsafe {
            let ptr = Cronet_BufferCallback_CreateWith(Some(cronetBufferCallbackOnDestroy));
            buffer_callbacks.lock().unwrap().insert(ptr, onDestroy);
            BufferCallback { ptr }
        }
    }

    pub fn destroy(&self) {
        unsafe {
            Cronet_BufferCallback_Destroy(self.ptr);
        }
    }
}
