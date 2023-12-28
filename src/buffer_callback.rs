use once_cell::sync::Lazy;

use crate::{
    state::CronetCallbacks, Buffer, Cronet_BufferCallbackPtr, Cronet_BufferCallback_CreateWith,
    Cronet_BufferCallback_Destroy, Cronet_BufferPtr, Destroy,
};

static mut BUFFER_CALLBACKS: Lazy<CronetCallbacks<Cronet_BufferCallbackPtr, BufferCallbackFn>> =
    Lazy::new(CronetCallbacks::new);

#[no_mangle]
unsafe extern "C" fn cronetBufferCallbackOnDestroy(
    selfPtr: Cronet_BufferCallbackPtr,
    bufferPtr: Cronet_BufferPtr,
) {
    let mut lockedMap = BUFFER_CALLBACKS.map().lock().unwrap();
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
    pub fn new(on_destroy: BufferCallbackFn) -> Self {
        unsafe {
            let ptr = Cronet_BufferCallback_CreateWith(Some(cronetBufferCallbackOnDestroy));
            BUFFER_CALLBACKS
                .map()
                .lock()
                .unwrap()
                .insert(ptr, on_destroy);
            BufferCallback { ptr }
        }
    }
}

impl Destroy for BufferCallback {
    fn destroy(&self) {
        unsafe {
            Cronet_BufferCallback_Destroy(self.ptr);
        }
    }
}
